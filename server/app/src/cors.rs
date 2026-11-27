//! CORS exactly as Laravel 13 does it: `Illuminate\Http\Middleware\HandleCors`
//! with `fruitcake/php-cors` and this repo's `backend/config/cors.php`
//! (paths `api/*`, `sanctum/csrf-cookie`; listed origins; methods and headers
//! `*`; no credentials; max age 0).
//!
//! Hand-written instead of `tower_http::cors` because the details the
//! contract suite checks differ: preflights answer **204**, an unknown
//! origin still gets a 204 (without `Access-Control-Allow-Origin`), allowed
//! methods and headers echo the request, and every matching response varies
//! on `Origin`, even without an `Origin` header.

use axum::extract::{Request, State};
use axum::http::{HeaderMap, HeaderValue, Method, StatusCode, header};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Cors {
    origins: Arc<Vec<String>>,
}

impl Cors {
    pub fn new(origins: Vec<String>) -> Self {
        Cors {
            origins: Arc::new(origins),
        }
    }

    fn allowed_origin<'a>(&self, headers: &'a HeaderMap) -> Option<&'a HeaderValue> {
        let origin = headers.get(header::ORIGIN)?;
        let s = origin.to_str().ok()?;
        self.origins.iter().any(|o| o == s).then_some(origin)
    }
}

/// `config/cors.php` paths: `api/*` and `sanctum/csrf-cookie`.
fn matches_path(path: &str) -> bool {
    let p = path.trim_start_matches('/');
    p.starts_with("api/") || p == "sanctum/csrf-cookie"
}

/// Append to `Vary` unless already listed (php-cors `varyHeader`).
fn vary(headers: &mut HeaderMap, name: &str) {
    let current = headers
        .get(header::VARY)
        .and_then(|v| v.to_str().ok())
        .map(str::to_string);
    let next = match current {
        None => name.to_string(),
        Some(cur) if cur.split(',').any(|v| v.trim().eq_ignore_ascii_case(name)) => return,
        Some(cur) => format!("{cur}, {name}"),
    };
    if let Ok(v) = HeaderValue::from_str(&next) {
        headers.insert(header::VARY, v);
    }
}

pub async fn middleware(State(cors): State<Cors>, req: Request, next: Next) -> Response {
    if !matches_path(req.uri().path()) {
        return next.run(req).await;
    }
    let req_headers = req.headers().clone();
    let allow_origin = cors.allowed_origin(&req_headers).cloned();

    let is_preflight = req.method() == Method::OPTIONS
        && req_headers.contains_key(header::ACCESS_CONTROL_REQUEST_METHOD);
    if is_preflight {
        let mut res = StatusCode::NO_CONTENT.into_response();
        let h = res.headers_mut();
        if let Some(origin) = allow_origin {
            h.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin);
            vary(h, "Origin");
            // allowed_methods = ['*']: echo the requested method, upper-cased.
            let method = req_headers
                .get(header::ACCESS_CONTROL_REQUEST_METHOD)
                .and_then(|v| v.to_str().ok())
                .map(str::to_ascii_uppercase)
                .unwrap_or_default();
            if let Ok(v) = HeaderValue::from_str(&method) {
                h.insert(header::ACCESS_CONTROL_ALLOW_METHODS, v);
            }
            vary(h, "Access-Control-Request-Method");
            // allowed_headers = ['*']: echo the requested headers verbatim.
            let headers = req_headers
                .get(header::ACCESS_CONTROL_REQUEST_HEADERS)
                .cloned()
                .unwrap_or_else(|| HeaderValue::from_static(""));
            h.insert(header::ACCESS_CONTROL_ALLOW_HEADERS, headers);
            vary(h, "Access-Control-Request-Headers");
            h.insert(
                header::ACCESS_CONTROL_MAX_AGE,
                HeaderValue::from_static("0"),
            );
        } else {
            vary(h, "Origin");
        }
        vary(h, "Access-Control-Request-Method");
        return res;
    }

    let is_options = req.method() == Method::OPTIONS;
    let mut res = next.run(req).await;
    let h = res.headers_mut();
    if is_options {
        vary(h, "Access-Control-Request-Method");
    }
    if let Some(origin) = allow_origin {
        h.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin);
    }
    vary(h, "Origin");
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_matching() {
        assert!(matches_path("/api/chapters"));
        assert!(matches_path("/api/x/y"));
        assert!(matches_path("/sanctum/csrf-cookie"));
        assert!(!matches_path("/api"));
        assert!(!matches_path("/up"));
        assert!(!matches_path("/"));
    }

    #[test]
    fn vary_appends_once() {
        let mut h = HeaderMap::new();
        vary(&mut h, "Origin");
        vary(&mut h, "Origin");
        vary(&mut h, "Access-Control-Request-Method");
        assert_eq!(h[header::VARY], "Origin, Access-Control-Request-Method");
    }
}
