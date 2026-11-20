//! Request input as Laravel's `$request->all()` sees it, minus the query
//! string (KNOWN_ISSUES K14: Rust reads only the body, so passwords never
//! come from URLs).
//!
//! | Content-Type | Laravel | Here |
//! |---|---|---|
//! | contains `/json` or `+json` | `json_decode($body, true)` cast to array | same: object as is, array keyed `"0".."n"`, scalar as `{"0": v}`, invalid or `null` as `{}` |
//! | `application/x-www-form-urlencoded` | PHP `parse_str` | same, including `a[]` and `a[b]` (see [`php_insert`]) |
//! | `multipart/form-data` | text fields + files | text fields as above; files in [`Input::files`] |
//! | anything else (none, `text/plain`) | empty | empty |
//!
//! Never use axum's `Json`/`Form` extractors for request input: their
//! rejections are plain-text 400/415/422 answers, while Laravel treats an
//! unreadable body as empty input.

use crate::error::AppError;
use axum::body::Bytes;
use axum::extract::{FromRequest, Multipart, Request};
use axum::http::header;
use serde_json::{Map, Value};

/// One uploaded file from a multipart body.
#[derive(Debug, Clone)]
pub struct UploadedFile {
    /// Client file name (`$file->getClientOriginalName()`), if sent.
    pub file_name: Option<String>,
    /// Client-declared type. Not trusted: uploads sniff the bytes.
    pub content_type: Option<String>,
    pub bytes: Bytes,
}

#[derive(Debug, Default, Clone)]
pub struct Input {
    pub fields: Map<String, Value>,
    /// Multipart file parts, by field name, in request order.
    pub files: Vec<(String, UploadedFile)>,
}

impl Input {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.fields.get(key)
    }

    pub fn file(&self, key: &str) -> Option<&UploadedFile> {
        self.files.iter().find(|(k, _)| k == key).map(|(_, f)| f)
    }
}

fn content_type(req: &Request) -> String {
    req.headers()
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_ascii_lowercase()
}

/// Laravel's `Request::isJson()`.
fn is_json(ct: &str) -> bool {
    ct.contains("/json") || ct.contains("+json")
}

/// `(array) json_decode($body, true)`, as Laravel's `InputBag` holds it.
pub fn json_input(body: &[u8]) -> Map<String, Value> {
    match serde_json::from_slice::<Value>(body) {
        Ok(Value::Object(map)) => map,
        Ok(Value::Array(items)) => items
            .into_iter()
            .enumerate()
            .map(|(i, v)| (i.to_string(), v))
            .collect(),
        Ok(Value::Null) | Err(_) => Map::new(),
        Ok(scalar) => std::iter::once(("0".to_string(), scalar)).collect(),
    }
}

/// PHP `parse_str` on an `application/x-www-form-urlencoded` body.
pub fn form_input(body: &[u8]) -> Map<String, Value> {
    let mut root = Map::new();
    for (k, v) in form_urlencoded::parse(body) {
        php_insert(&mut root, &k, Value::String(v.into_owned()));
    }
    finish(root)
}

/// Split a PHP input name into its base and bracket segments:
/// `a[b][]` → (`a`, [`b`, ``]). A `[` without a closing `]` makes the rest
/// part of the base name, and PHP turns `.`, ` ` and that lone `[` into `_`.
fn php_key(name: &str) -> (String, Vec<String>) {
    let (base, rest) = match name.find('[') {
        Some(i) if name[i..].contains(']') => (&name[..i], &name[i..]),
        _ => (name, ""),
    };
    let base: String = base
        .chars()
        .map(|c| if matches!(c, '.' | ' ' | '[') { '_' } else { c })
        .collect();
    let mut segs = Vec::new();
    let mut s = rest;
    while let Some(stripped) = s.strip_prefix('[') {
        match stripped.find(']') {
            Some(end) => {
                segs.push(stripped[..end].to_string());
                s = &stripped[end + 1..];
            }
            None => break,
        }
    }
    (base, segs)
}

/// Insert one form value like PHP: later duplicates win, `[]` appends with
/// the next integer key, `[k]` nests.
pub fn php_insert(root: &mut Map<String, Value>, name: &str, value: Value) {
    let (base, segs) = php_key(name);
    if base.is_empty() {
        return;
    }
    let mut cur = root;
    let mut key = base;
    for seg in segs {
        let slot = cur.entry(key).or_insert_with(|| Value::Object(Map::new()));
        if !slot.is_object() {
            *slot = Value::Object(Map::new());
        }
        let Value::Object(next) = slot else { return };
        key = if seg.is_empty() {
            let max = next.keys().filter_map(|k| k.parse::<u64>().ok()).max();
            max.map_or(0, |m| m + 1).to_string()
        } else {
            seg
        };
        cur = next;
    }
    cur.insert(key, value);
}

/// PHP arrays with keys exactly `0..n` serialise as JSON lists; convert so
/// form input matches what the same data would look like as JSON.
fn finish(map: Map<String, Value>) -> Map<String, Value> {
    map.into_iter().map(|(k, v)| (k, finish_value(v))).collect()
}

fn finish_value(v: Value) -> Value {
    match v {
        Value::Object(map) => {
            let sequential = !map.is_empty()
                && map
                    .keys()
                    .enumerate()
                    .all(|(i, k)| k.parse::<usize>().ok() == Some(i));
            let map = finish(map);
            if sequential {
                Value::Array(map.into_iter().map(|(_, v)| v).collect())
            } else {
                Value::Object(map)
            }
        }
        other => other,
    }
}

impl<S: Send + Sync> FromRequest<S> for Input {
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let ct = content_type(&req);
        if ct.starts_with("multipart/form-data") {
            return read_multipart(req, state).await;
        }
        let body = match Bytes::from_request(req, state).await {
            Ok(b) => b,
            Err(e) if e.status() == axum::http::StatusCode::PAYLOAD_TOO_LARGE => {
                return Err(AppError::PostTooLarge);
            }
            Err(_) => Bytes::new(),
        };
        let fields = if is_json(&ct) {
            json_input(&body)
        } else if ct.starts_with("application/x-www-form-urlencoded") {
            form_input(&body)
        } else {
            Map::new()
        };
        Ok(Input {
            fields,
            files: Vec::new(),
        })
    }
}

async fn read_multipart<S: Send + Sync>(req: Request, state: &S) -> Result<Input, AppError> {
    let Ok(mut multipart) = Multipart::from_request(req, state).await else {
        return Ok(Input::default());
    };
    let mut fields = Map::new();
    let mut files = Vec::new();
    loop {
        let field = match multipart.next_field().await {
            Ok(Some(f)) => f,
            Ok(None) => break,
            Err(e) if e.status() == axum::http::StatusCode::PAYLOAD_TOO_LARGE => {
                return Err(AppError::PostTooLarge);
            }
            // A malformed body ends the input, like PHP keeping what it parsed.
            Err(_) => break,
        };
        let Some(name) = field.name().map(str::to_string) else {
            continue;
        };
        let file_name = field.file_name().map(str::to_string);
        let content_type = field.content_type().map(str::to_string);
        let Ok(bytes) = field.bytes().await else {
            break;
        };
        if file_name.is_some() {
            files.push((
                name,
                UploadedFile {
                    file_name,
                    content_type,
                    bytes,
                },
            ));
        } else {
            let text = String::from_utf8_lossy(&bytes).into_owned();
            php_insert(&mut fields, &name, Value::String(text));
        }
    }
    Ok(Input {
        fields: finish(fields),
        files,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn obj(v: Value) -> Map<String, Value> {
        match v {
            Value::Object(m) => m,
            _ => panic!("not an object"),
        }
    }

    #[test]
    fn json_shapes_like_php_array_cast() {
        assert_eq!(json_input(br#"{"a":1}"#), obj(json!({"a": 1})));
        assert_eq!(
            json_input(br#"["x","y"]"#),
            obj(json!({"0": "x", "1": "y"}))
        );
        assert_eq!(json_input(br#""x""#), obj(json!({"0": "x"})));
        assert_eq!(json_input(b"null"), Map::new());
        assert_eq!(json_input(b"{bad"), Map::new());
        assert_eq!(json_input(b""), Map::new());
    }

    #[test]
    fn form_like_parse_str() {
        assert_eq!(
            form_input(b"password=a%20b"),
            obj(json!({"password": "a b"}))
        );
        assert_eq!(
            form_input(b"password=nope&password=yes"),
            obj(json!({"password": "yes"}))
        );
        assert_eq!(form_input(b"password[]=x"), obj(json!({"password": ["x"]})));
        assert_eq!(
            form_input(b"order[]=a&order[]=b"),
            obj(json!({"order": ["a", "b"]}))
        );
        assert_eq!(
            form_input(b"lesson[formation][]=p&lesson[intro]=i"),
            obj(json!({"lesson": {"formation": ["p"], "intro": "i"}}))
        );
        assert_eq!(
            form_input(b"a.b=1&c d=2"),
            obj(json!({"a_b": "1", "c_d": "2"}))
        );
        assert_eq!(
            form_input(b"x[5]=a&x[]=b"),
            obj(json!({"x": {"5": "a", "6": "b"}}))
        );
        assert_eq!(form_input(b"=orphan"), Map::new());
    }
}
