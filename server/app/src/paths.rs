//! Laravel's path handling before routing.
//!
//! - `ValidatePathEncoding` (first global middleware): if
//!   `rawurldecode(path)` isn't valid UTF-8, answer 400
//!   `{"message": "Malformed URL."}`. It runs before CORS, the post-size
//!   check, routing and auth, so that response has no CORS headers.
//! - `UriValidator`: routes match `rawurldecode(rtrim(pathInfo, '/'))`.
//!   Trailing slashes are trimmed from the **raw** path first, so `x/`
//!   matches `x` but `x%2F` does not. Matching on the decoded path means
//!   `/api/%63hapters` is the index, `/api/chapters%2Freorder` is the
//!   reorder route, and `a%2Fb` (decoded `a/b`) matches no `{id}` route.
//!   Route parameters are the decoded text, decoded once (`a%2541` → `a%41`);
//!   `+` stays `+`.
//! - Error messages (route 404, 405) quote the raw path, `$request->path()`.
//!
//! axum routes on the raw path and percent-decodes `Path` parameters. So
//! the middleware rewrites the URI path to the decoded path re-encoded
//! with only unreserved characters and `/` left as they are; matchit then
//! sees the decoded segments, and `Path` decodes back to exactly Laravel's
//! parameter. The raw path is kept in the [`RawPath`] extension for error
//! messages. Contract test: `framework::paths_are_decoded_like_laravel`.

use crate::error::AppError;
use axum::extract::Request;
use axum::http::Uri;
use axum::http::uri::PathAndQuery;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

/// The request path as received, before [`middleware`] rewrote it.
#[derive(Clone, Debug)]
pub struct RawPath(pub String);

/// The raw path of a request: the [`RawPath`] extension if the middleware
/// ran, else the URI path.
pub fn raw_path(req: &Request) -> String {
    req.extensions()
        .get::<RawPath>()
        .map_or_else(|| req.uri().path().to_string(), |p| p.0.clone())
}

/// PHP `rawurldecode`: `%XX` with two hex digits becomes that byte; any
/// other `%` stays literal; `+` is not touched.
pub fn rawurldecode(s: &str) -> Vec<u8> {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%'
            && i + 2 < b.len()
            && let (Some(h), Some(l)) = (hex(b[i + 1]), hex(b[i + 2]))
        {
            out.push((h << 4) | l);
            i += 3;
        } else {
            out.push(b[i]);
            i += 1;
        }
    }
    out
}

fn hex(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

/// The path axum should route on, or `None` when Laravel would answer
/// "Malformed URL.". The validity check is on the whole decoded path;
/// the routing path is the raw path right-trimmed of `/`, then decoded,
/// then re-encoded with everything but `A-Za-z0-9-._~` and `/` escaped.
pub fn routing_path(raw: &str) -> Option<String> {
    if std::str::from_utf8(&rawurldecode(raw)).is_err() {
        return None;
    }
    let trimmed = raw.trim_end_matches('/');
    let decoded = rawurldecode(if trimmed.is_empty() { "/" } else { trimmed });
    let mut out = String::with_capacity(decoded.len());
    for &c in &decoded {
        if c.is_ascii_alphanumeric() || matches!(c, b'-' | b'.' | b'_' | b'~' | b'/') {
            out.push(char::from(c));
        } else {
            const HEX: &[u8; 16] = b"0123456789ABCDEF";
            out.push('%');
            out.push(char::from(HEX[usize::from(c >> 4)]));
            out.push(char::from(HEX[usize::from(c & 15)]));
        }
    }
    Some(out)
}

/// Runs before CORS (Laravel's order); see the module docs.
pub async fn middleware(mut req: Request, next: Next) -> Response {
    let raw = req.uri().path().to_string();
    let Some(path) = routing_path(&raw) else {
        return AppError::MalformedUrl.into_response();
    };
    if path != raw {
        let pq = match req.uri().query() {
            Some(q) => format!("{path}?{q}"),
            None => path,
        };
        // The re-encoded path only holds URI-safe bytes and the query was
        // already valid, so this parse cannot fail in practice. If it did,
        // routing on the raw path is the safe fallback.
        if let Ok(pq) = pq.parse::<PathAndQuery>() {
            let mut parts = req.uri().clone().into_parts();
            parts.path_and_query = Some(pq);
            if let Ok(uri) = Uri::from_parts(parts) {
                *req.uri_mut() = uri;
            }
        }
    }
    req.extensions_mut().insert(RawPath(raw));
    next.run(req).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_like_php() {
        assert_eq!(rawurldecode("a%20b+c"), b"a b+c");
        assert_eq!(rawurldecode("%41%4a%4A"), b"AJJ");
        assert_eq!(rawurldecode("%"), b"%");
        assert_eq!(rawurldecode("%4"), b"%4");
        assert_eq!(rawurldecode("%zz%2"), b"%zz%2");
        assert_eq!(rawurldecode("a%2541"), b"a%41");
        assert_eq!(rawurldecode("%FF"), [0xFF]);
    }

    #[test]
    fn routing_paths() {
        let r = |p: &str| routing_path(p).unwrap();
        assert_eq!(r("/api/chapters"), "/api/chapters");
        assert_eq!(r("/api/chapters/"), "/api/chapters");
        assert_eq!(r("/api/chapters//"), "/api/chapters");
        assert_eq!(r("/"), "/");
        assert_eq!(r("//"), "/");
        assert_eq!(r(""), "/");
        assert_eq!(r("/api/%63hapters"), "/api/chapters");
        assert_eq!(r("/api/chapters%2Freorder"), "/api/chapters/reorder");
        assert_eq!(r("/api/chapters/x%2F"), "/api/chapters/x/");
        assert_eq!(r("/api/chapters/a%2541"), "/api/chapters/a%2541");
        assert_eq!(r("/api/chapters/a+b"), "/api/chapters/a%2Bb");
        assert_eq!(r("/api/chapters/a%20b"), "/api/chapters/a%20b");
        assert_eq!(r("/api/chapters/%"), "/api/chapters/%25");
        assert_eq!(r("/api/chapters/%C3%A9"), "/api/chapters/%C3%A9");
        assert_eq!(r("/api//chapters"), "/api//chapters");
    }

    #[test]
    fn malformed_paths() {
        assert_eq!(routing_path("/api/chapters/%FF"), None);
        assert_eq!(routing_path("/api/auth/login%C3"), None);
        // The check covers the whole path, trailing part included.
        assert_eq!(routing_path("/x/%E9/"), None);
    }
}
