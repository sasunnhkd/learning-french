//! Request input as Laravel's `$request->all()` sees it, minus the query
//! string (KNOWN_ISSUES K14: Rust reads only the body, so passwords never
//! come from URLs).
//!
//! | Content-Type | Laravel | Here |
//! |---|---|---|
//! | contains `/json` or `+json` | `json_decode($body, true)` cast to array | same: object as is, array keyed `"0".."n"`, scalar as `{"0": v}`, invalid or `null` as `{}` |
//! | `application/x-www-form-urlencoded` | PHP `parse_str` | same, including `a[]` and `a[b]` (see [`php_insert`]) |
//! | `multipart/form-data` | text fields + files | text fields as above; files in [`Input::files`]. Each part's `Content-Disposition` is read with PHP's rules ([`php_disposition`]), not multer's |
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
    for (k, v) in form_urlencoded::parse(body).take(MAX_FORM_VARS) {
        php_insert(&mut root, &k, Value::String(v.into_owned()));
    }
    finish(root)
}

/// PHP's `max_input_vars` (1000) as Laravel under `php -S` applies it,
/// probed in the Phase 5 audit: a urlencoded body keeps its first **1001**
/// fields (PHP's POST reader is off by one), a multipart body its first
/// **1000** text fields; file parts don't count. Later fields are dropped.
/// This also bounds the work `php_insert` does per request.
pub const MAX_FORM_VARS: usize = 1001;
pub const MAX_MULTIPART_VARS: usize = 1000;
/// PHP's `max_input_nesting_level` (64): a name with more bracket levels is
/// dropped. Without it, `a[b][b]…` 20,000 deep overflowed the stack in the
/// recursive `finish` and aborted the server (Phase 5 audit, F1).
pub const MAX_INPUT_NESTING: usize = 64;
/// PHP's `max_file_uploads` (20): file parts after the 20th with a
/// non-empty file name are ignored; parts with an empty name don't count.
pub const MAX_FILE_UPLOADS: usize = 20;

/// Split a PHP input name into its base and bracket segments:
/// `a[b][]` → (`a`, [`b`, `""`]). A `[` without a closing `]` makes the rest
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
    if base.is_empty() || segs.len() > MAX_INPUT_NESTING {
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
            // PHP's next index: one past the largest integer key. Only
            // canonical integers count ("05" is a string key in PHP). Past
            // PHP_INT_MAX, PHP drops the value ("next element is already
            // occupied").
            let max = next
                .keys()
                .filter_map(|k| k.parse::<i64>().ok().filter(|n| n.to_string() == *k))
                .max();
            match max {
                None => "0".to_string(),
                Some(m) => match m.checked_add(1) {
                    Some(n) => n.to_string(),
                    None => return,
                },
            }
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

/// PHP's `rfc1867.c` reading of a part's `Content-Disposition`: split on
/// `;` outside quotes, keep the `name` and `filename` parameters (matched
/// case-insensitively; `filename*` is ignored), and read each value with
/// `php_ap_getword_conf`: leading spaces skipped, a `"…"` or `'…'` value up
/// to its closing quote with `\"` and `\\` unescaped, else up to the next
/// space. Bytes are kept as sent: a file name that isn't UTF-8 is later
/// stored with U+FFFD in place of the bad bytes (KNOWN_ISSUES K19).
pub fn php_disposition(header: &[u8]) -> (Option<Vec<u8>>, Option<Vec<u8>>) {
    fn is_space(c: u8) -> bool {
        matches!(c, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c)
    }
    // php_ap_getword(line, stop): up to `stop`, skipping over quoted parts.
    fn getword(line: &[u8], stop: u8) -> (&[u8], &[u8]) {
        let mut i = 0;
        while i < line.len() && line[i] != stop {
            let q = line[i];
            if q == b'"' || q == b'\'' {
                i += 1;
                while i < line.len() && line[i] != q {
                    i += if line[i] == b'\\' && line.get(i + 1) == Some(&q) {
                        2
                    } else {
                        1
                    };
                }
                if i < line.len() {
                    i += 1;
                }
            } else {
                i += 1;
            }
        }
        let i = i.min(line.len());
        let rest = if i < line.len() {
            &line[i + 1..]
        } else {
            &line[i..]
        };
        (&line[..i], rest)
    }
    // php_ap_getword_conf + substring_conf
    fn value(v: &[u8]) -> Vec<u8> {
        let start = v.iter().position(|c| !is_space(*c)).unwrap_or(v.len());
        let v = &v[start..];
        let (body, quote) = match v.first() {
            Some(&q @ (b'"' | b'\'')) => (&v[1..], Some(q)),
            _ => {
                let end = v.iter().position(|c| is_space(*c)).unwrap_or(v.len());
                (&v[..end], None)
            }
        };
        let mut out = Vec::with_capacity(body.len());
        let mut i = 0;
        while i < body.len() && Some(body[i]) != quote {
            if body[i] == b'\\'
                && body
                    .get(i + 1)
                    .is_some_and(|n| *n == b'\\' || Some(*n) == quote)
            {
                out.push(body[i + 1]);
                i += 2;
            } else {
                out.push(body[i]);
                i += 1;
            }
        }
        out
    }
    let (mut name, mut file_name) = (None, None);
    let mut cd = header;
    while let Some(start) = cd.iter().position(|c| !is_space(*c)) {
        let (pair, rest) = getword(&cd[start..], b';');
        cd = rest;
        if let Some(eq) = pair.iter().position(|c| *c == b'=') {
            let (key, val) = (&pair[..eq], &pair[eq + 1..]);
            if key.eq_ignore_ascii_case(b"name") {
                name = Some(value(val));
            } else if key.eq_ignore_ascii_case(b"filename") {
                file_name = Some(value(val));
            }
        }
    }
    (name, file_name)
}

async fn read_multipart<S: Send + Sync>(req: Request, state: &S) -> Result<Input, AppError> {
    let Ok(mut multipart) = Multipart::from_request(req, state).await else {
        return Ok(Input::default());
    };
    let mut fields = Map::new();
    let mut files = Vec::new();
    let mut text_vars = 0usize;
    let mut real_files = 0usize;
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
        // PHP's own Content-Disposition parsing, not multer's (see
        // `php_disposition`).
        let (name, file_name) = field
            .headers()
            .get(header::CONTENT_DISPOSITION)
            .map_or((None, None), |v| php_disposition(v.as_bytes()));
        let Some(name) = name else {
            continue;
        };
        let name = String::from_utf8_lossy(&name).into_owned();
        let file_name = file_name.map(|f| String::from_utf8_lossy(&f).into_owned());
        let content_type = field.content_type().map(str::to_string);
        let Ok(bytes) = field.bytes().await else {
            break;
        };
        if let Some(fname) = &file_name {
            // Only parts with a file name count toward the cap (probed on
            // Laravel: 20 empty-name parts don't use it up).
            if !fname.is_empty() {
                real_files += 1;
                if real_files > MAX_FILE_UPLOADS {
                    continue;
                }
            }
            files.push((
                name,
                UploadedFile {
                    file_name,
                    content_type,
                    bytes,
                },
            ));
        } else {
            text_vars += 1;
            if text_vars > MAX_MULTIPART_VARS {
                continue;
            }
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

    #[test]
    fn content_disposition_like_php() {
        let d = |h: &[u8]| {
            let (n, f) = php_disposition(h);
            (
                n.map(|v| String::from_utf8_lossy(&v).into_owned()),
                f.map(|v| String::from_utf8_lossy(&v).into_owned()),
            )
        };
        let s = |v: &str| Some(v.to_string());
        assert_eq!(
            d(br#"form-data; name="file"; filename="b.png""#),
            (s("file"), s("b.png"))
        );
        assert_eq!(
            d(br#"form-data; Name="file"; FILENAME="b.png""#),
            (s("file"), s("b.png"))
        );
        assert_eq!(
            d(br#"form-data; name=file; filename=plain.png"#),
            (s("file"), s("plain.png"))
        );
        assert_eq!(
            d(br#"form-data; name='file'; filename='a;b.png'"#),
            (s("file"), s("a;b.png"))
        );
        assert_eq!(
            d(br#"form-data; name="file"; filename="a\"b.png""#),
            (s("file"), s("a\"b.png"))
        );
        assert_eq!(
            d(br#"form-data; name="file"; filename="a\\b\x.png""#),
            (s("file"), s("a\\b\\x.png"))
        );
        assert_eq!(
            d(b"form-data; name=\"file\"; filename*=UTF-8''%C3%A9.png"),
            (s("file"), None)
        );
        assert_eq!(
            d(br#"form-data; name="file"; filename="""#),
            (s("file"), s(""))
        );
        assert_eq!(d(br#"form-data; filename="b.png""#), (None, s("b.png")));
        assert_eq!(
            d(br#"form-data;name="a b";filename=x y.png"#),
            (s("a b"), s("x"))
        );
        let (_, f) = php_disposition(b"form-data; name=\"file\"; filename=\"caf\xe9.png\"");
        assert_eq!(f.unwrap(), b"caf\xe9.png");
    }

    #[test]
    fn php_input_limits() {
        let deep = |n: usize| format!("a{}=x", "[b]".repeat(n));
        let ok = form_input(deep(MAX_INPUT_NESTING).as_bytes());
        assert!(ok.contains_key("a"));
        let dropped = form_input(deep(MAX_INPUT_NESTING + 1).as_bytes());
        assert!(dropped.is_empty());
        // 20,000 levels: dropped, not a stack overflow.
        assert!(form_input(deep(20_000).as_bytes()).is_empty());
        let many: Vec<String> = (0..1005).map(|i| format!("v{i}=1")).collect();
        let m = form_input(many.join("&").as_bytes());
        assert_eq!(m.len(), MAX_FORM_VARS);
        assert!(m.contains_key("v1000") && !m.contains_key("v1001"));
        let appends = vec!["a[]=1"; 1005].join("&");
        assert_eq!(
            form_input(appends.as_bytes())["a"]
                .as_array()
                .unwrap()
                .len(),
            MAX_FORM_VARS
        );
    }
}
