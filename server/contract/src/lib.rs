//! Contract (parity) suite helpers.
//!
//! The suite is black-box HTTP: every test talks to `BASE_URL` and runs
//! unchanged against the Laravel app and the Rust port. The only per-target
//! branches are the approved or defaulted differences listed in
//! `docs/migration/KNOWN_ISSUES.md`; use [`Ctx::expect_by_target`] for those
//! and cite the K-number.
//!
//! Environment (set by `run-laravel.sh` / `run-rust.sh`):
//! - `BASE_URL`         `http://127.0.0.1:<port>` (plain HTTP only)
//! - `EDITOR_PASSWORD`  the login password configured for the server
//! - `TARGET`           `laravel` or `rust`
//! - `SANDBOX_ROOT`     temp dir mirroring the repo (`src/`, `public/uploads/`)
//! - `CONTRACT_DB`      the SQLite file the server uses (fixtures only)
//!
//! HTTP is a small std-only HTTP/1.1 client (`Connection: close`, handles
//! `Content-Length` and chunked bodies), so the suite needs no async runtime.

use serde_json::Value;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    Laravel,
    Rust,
}

#[derive(Debug, Clone)]
pub struct Ctx {
    pub host: String,
    pub port: u16,
    pub password: String,
    pub target: Target,
    pub sandbox: PathBuf,
    pub db: PathBuf,
}

fn env(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| {
        panic!(
            "{name} is not set: run the suite through server/contract/run-laravel.sh or run-rust.sh"
        )
    })
}

impl Ctx {
    pub fn from_env() -> Self {
        let base = env("BASE_URL");
        let hostport = base
            .strip_prefix("http://")
            .unwrap_or_else(|| panic!("BASE_URL must be http://host:port, got {base}"))
            .trim_end_matches('/');
        let (host, port) = hostport
            .rsplit_once(':')
            .unwrap_or_else(|| panic!("BASE_URL needs an explicit port, got {base}"));
        let target = match env("TARGET").as_str() {
            "laravel" => Target::Laravel,
            "rust" => Target::Rust,
            other => panic!("TARGET must be laravel or rust, got {other}"),
        };
        Ctx {
            host: host.to_string(),
            port: port.parse().expect("BASE_URL port"),
            password: env("EDITOR_PASSWORD"),
            target,
            sandbox: PathBuf::from(env("SANDBOX_ROOT")),
            db: PathBuf::from(env("CONTRACT_DB")),
        }
    }

    pub fn is_laravel(&self) -> bool {
        self.target == Target::Laravel
    }

    /// Pick the expectation for this target. `issue` names the KNOWN_ISSUES
    /// entry that justifies the difference (for the failure message).
    pub fn expect_by_target<T>(&self, issue: &str, laravel: T, rust: T) -> T {
        let _ = issue;
        match self.target {
            Target::Laravel => laravel,
            Target::Rust => rust,
        }
    }

    pub fn req(&self, method: &str, path: &str) -> Req<'_> {
        Req {
            ctx: self,
            method: method.to_string(),
            path: path.to_string(),
            headers: vec![("Accept".into(), "application/json".into())],
            body: Vec::new(),
        }
    }

    pub fn get(&self, path: &str) -> Req<'_> {
        self.req("GET", path)
    }
    pub fn post(&self, path: &str) -> Req<'_> {
        self.req("POST", path)
    }
    pub fn put(&self, path: &str) -> Req<'_> {
        self.req("PUT", path)
    }
    pub fn delete(&self, path: &str) -> Req<'_> {
        self.req("DELETE", path)
    }

    /// Log in with the configured password and return the bearer token.
    pub fn login(&self) -> String {
        let r = self
            .post("/api/auth/login")
            .json(&serde_json::json!({ "password": self.password }))
            .send();
        assert_eq!(r.status, 200, "login failed: {}", r.text());
        r.json()["token"]
            .as_str()
            .expect("token is a string")
            .to_string()
    }

    /// Run SQL against the server's SQLite file through the `sqlite3` CLI.
    /// Fixtures only (states the HTTP API cannot create, such as an expired
    /// token). Callers pass literals or server-generated integer ids only.
    pub fn sqlite(&self, sql: &str) -> String {
        let out = Command::new("sqlite3")
            .arg(&self.db)
            .arg(sql)
            .output()
            .expect("sqlite3 CLI is required for fixtures");
        assert!(
            out.status.success(),
            "sqlite3 failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    }

    /// Run PHP inside the sandbox's Laravel app (`SANDBOX_ROOT/backend`, the
    /// same database as the server under test), with the framework booted.
    /// Cross-checks only, e.g. a token Laravel mints is accepted by Rust and
    /// vice versa. `args` reach the code as `$argv[1..]`, so values are never
    /// spliced into the source. Returns trimmed stdout.
    pub fn php(&self, code: &str, args: &[&str]) -> String {
        let boot = "require 'vendor/autoload.php'; $app = require 'bootstrap/app.php'; \
                    $app->make(Illuminate\\Contracts\\Console\\Kernel::class)->bootstrap(); ";
        let out = Command::new(std::env::var("PHP").unwrap_or_else(|_| "php".into()))
            .current_dir(self.sandbox.join("backend"))
            .arg("-r")
            .arg(format!("{boot}{code}"))
            .arg("--")
            .args(args)
            .output()
            .expect("php CLI is required for cross-checks");
        assert!(
            out.status.success(),
            "php failed: {}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    }

    /// The authoritative content, as the server was seeded with it.
    pub fn seed_chapters(&self) -> Vec<Value> {
        let path = self.sandbox.join("backend/database/seeders/chapters.json");
        let path = if path.exists() {
            path
        } else {
            self.sandbox.join("seed/chapters.json")
        };
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        match serde_json::from_str(&text) {
            Ok(Value::Array(v)) => v,
            other => panic!("seed chapters.json is not an array: {other:?}"),
        }
    }
}

pub struct Req<'a> {
    ctx: &'a Ctx,
    method: String,
    path: String,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

impl Req<'_> {
    pub fn header(mut self, name: &str, value: &str) -> Self {
        self.headers.retain(|(n, _)| !n.eq_ignore_ascii_case(name));
        self.headers.push((name.to_string(), value.to_string()));
        self
    }

    pub fn without_header(mut self, name: &str) -> Self {
        self.headers.retain(|(n, _)| !n.eq_ignore_ascii_case(name));
        self
    }

    pub fn bearer(self, token: &str) -> Self {
        self.header("Authorization", &format!("Bearer {token}"))
    }

    pub fn json(self, v: &Value) -> Self {
        let body = serde_json::to_vec(v).expect("serialize body");
        self.raw("application/json", body)
    }

    pub fn raw(mut self, content_type: &str, body: Vec<u8>) -> Self {
        self = self.header("Content-Type", content_type);
        self.body = body;
        self
    }

    /// `multipart/form-data` with one part named `field`.
    pub fn multipart(
        self,
        field: &str,
        filename: Option<&str>,
        part_type: &str,
        data: &[u8],
    ) -> Self {
        let boundary = "----contract-boundary-7f3a9c";
        let mut body = Vec::new();
        body.extend_from_slice(format!("--{boundary}\r\n").as_bytes());
        match filename {
            Some(f) => body.extend_from_slice(
                format!(
                    "Content-Disposition: form-data; name=\"{field}\"; filename=\"{f}\"\r\nContent-Type: {part_type}\r\n\r\n"
                )
                .as_bytes(),
            ),
            None => body.extend_from_slice(
                format!("Content-Disposition: form-data; name=\"{field}\"\r\n\r\n").as_bytes(),
            ),
        }
        body.extend_from_slice(data);
        body.extend_from_slice(format!("\r\n--{boundary}--\r\n").as_bytes());
        self.raw(&format!("multipart/form-data; boundary={boundary}"), body)
    }

    pub fn send(self) -> Resp {
        let addr = format!("{}:{}", self.ctx.host, self.ctx.port);
        let mut stream =
            TcpStream::connect(&addr).unwrap_or_else(|e| panic!("connect {addr}: {e}"));
        stream
            .set_read_timeout(Some(Duration::from_secs(60)))
            .expect("set timeout");
        let mut head = format!(
            "{} {} HTTP/1.1\r\nHost: {addr}\r\nConnection: close\r\nContent-Length: {}\r\n",
            self.method,
            self.path,
            self.body.len()
        );
        for (n, v) in &self.headers {
            head.push_str(&format!("{n}: {v}\r\n"));
        }
        head.push_str("\r\n");
        stream.write_all(head.as_bytes()).expect("write head");
        // The server may reject early (e.g. 413) and close; ignore write errors
        // on the body and read whatever response came back.
        let _ = stream.write_all(&self.body);
        let mut raw = Vec::new();
        let _ = stream.read_to_end(&mut raw);
        Resp::parse(&raw).unwrap_or_else(|e| {
            panic!(
                "{} {}: bad HTTP response ({e}): {:?}",
                self.method,
                self.path,
                String::from_utf8_lossy(&raw)
            )
        })
    }
}

#[derive(Debug)]
pub struct Resp {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl Resp {
    fn parse(raw: &[u8]) -> Result<Resp, String> {
        let split = raw
            .windows(4)
            .position(|w| w == b"\r\n\r\n")
            .ok_or("no header terminator")?;
        let head = std::str::from_utf8(&raw[..split]).map_err(|e| e.to_string())?;
        let mut lines = head.split("\r\n");
        let status_line = lines.next().ok_or("empty response")?;
        let status = status_line
            .split(' ')
            .nth(1)
            .ok_or("no status code")?
            .parse()
            .map_err(|e| format!("status: {e}"))?;
        let headers: Vec<(String, String)> = lines
            .filter_map(|l| l.split_once(':'))
            .map(|(n, v)| (n.trim().to_string(), v.trim().to_string()))
            .collect();
        let rest = &raw[split + 4..];
        let chunked = headers.iter().any(|(n, v)| {
            n.eq_ignore_ascii_case("transfer-encoding") && v.eq_ignore_ascii_case("chunked")
        });
        let body = if chunked {
            dechunk(rest)?
        } else {
            rest.to_vec()
        };
        Ok(Resp {
            status,
            headers,
            body,
        })
    }

    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(n, _)| n.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }

    pub fn text(&self) -> String {
        String::from_utf8_lossy(&self.body).into_owned()
    }

    pub fn json(&self) -> Value {
        serde_json::from_slice(&self.body).unwrap_or_else(|e| {
            panic!(
                "status {} body is not JSON ({e}): {}",
                self.status,
                self.text()
            )
        })
    }

    pub fn is_json(&self) -> bool {
        self.header("content-type")
            .is_some_and(|v| v.starts_with("application/json"))
    }

    /// Assert status and return the JSON body.
    #[track_caller]
    pub fn expect(&self, status: u16) -> Value {
        assert_eq!(
            self.status,
            status,
            "unexpected status; body: {}",
            self.text()
        );
        assert!(
            self.is_json(),
            "expected JSON content-type, got {:?}",
            self.header("content-type")
        );
        self.json()
    }
}

fn dechunk(mut data: &[u8]) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();
    loop {
        let eol = data
            .windows(2)
            .position(|w| w == b"\r\n")
            .ok_or("chunk size line")?;
        let size_str = std::str::from_utf8(&data[..eol]).map_err(|e| e.to_string())?;
        let size = usize::from_str_radix(size_str.split(';').next().unwrap_or("").trim(), 16)
            .map_err(|e| format!("chunk size: {e}"))?;
        data = &data[eol + 2..];
        if size == 0 {
            return Ok(out);
        }
        if data.len() < size + 2 {
            return Err("truncated chunk".into());
        }
        out.extend_from_slice(&data[..size]);
        data = &data[size + 2..];
    }
}

/// Keys of a JSON object, in order.
pub fn keys(v: &Value) -> Vec<String> {
    v.as_object()
        .unwrap_or_else(|| panic!("not an object: {v}"))
        .keys()
        .cloned()
        .collect()
}

/// Laravel 422 body: `{"message": first[ (and N more error|errors)], "errors": {...}}`.
pub fn validation_body(errors: &[(&str, &[&str])]) -> Value {
    let total: usize = errors.iter().map(|(_, m)| m.len()).sum();
    let first = errors
        .first()
        .and_then(|(_, m)| m.first())
        .expect("at least one error");
    let message = match total - 1 {
        0 => (*first).to_string(),
        1 => format!("{first} (and 1 more error)"),
        n => format!("{first} (and {n} more errors)"),
    };
    let mut map = serde_json::Map::new();
    for (field, msgs) in errors {
        map.insert(
            (*field).to_string(),
            Value::Array(
                msgs.iter()
                    .map(|m| Value::String((*m).to_string()))
                    .collect(),
            ),
        );
    }
    serde_json::json!({ "message": message, "errors": Value::Object(map) })
}

/// Assert a 422 with exactly these errors (order-sensitive, like Laravel).
#[track_caller]
pub fn assert_validation(r: &Resp, errors: &[(&str, &[&str])]) {
    let body = r.expect(422);
    let want = validation_body(errors);
    assert_eq!(body, want, "422 body mismatch");
    assert_eq!(
        keys(&body["errors"]),
        keys(&want["errors"]),
        "422 error key order"
    );
}

/// The bytes PHP writes for
/// `json_encode($v, JSON_PRETTY_PRINT|JSON_UNESCAPED_UNICODE|JSON_UNESCAPED_SLASHES) . "\n"`:
/// 4-space indent, `": "` separators, raw UTF-8, `/` not escaped, but U+2028
/// and U+2029 still escaped (no JSON_UNESCAPED_LINE_TERMINATORS).
pub fn php_pretty(v: &Value) -> Vec<u8> {
    fn scalar(s: &str) -> String {
        serde_json::to_string(s)
            .expect("serialize string")
            .replace('\u{2028}', "\\u2028")
            .replace('\u{2029}', "\\u2029")
    }
    fn write(v: &Value, depth: usize, out: &mut String) {
        let pad = |d: usize| "    ".repeat(d);
        match v {
            Value::String(s) => out.push_str(&scalar(s)),
            Value::Array(items) if items.is_empty() => out.push_str("[]"),
            Value::Object(map) if map.is_empty() => out.push_str("{}"),
            Value::Array(items) => {
                out.push_str("[\n");
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        out.push_str(",\n");
                    }
                    out.push_str(&pad(depth + 1));
                    write(item, depth + 1, out);
                }
                out.push('\n');
                out.push_str(&pad(depth));
                out.push(']');
            }
            Value::Object(map) => {
                out.push_str("{\n");
                for (i, (k, item)) in map.iter().enumerate() {
                    if i > 0 {
                        out.push_str(",\n");
                    }
                    out.push_str(&pad(depth + 1));
                    out.push_str(&scalar(k));
                    out.push_str(": ");
                    write(item, depth + 1, out);
                }
                out.push('\n');
                out.push_str(&pad(depth));
                out.push('}');
            }
            other => out.push_str(&other.to_string()),
        }
    }
    let mut s = String::new();
    write(v, 0, &mut s);
    s.push('\n');
    s.into_bytes()
}

/// CRC-32 (IEEE), as PHP `hash('crc32b', ...)`.
pub fn crc32b(data: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &b in data {
        crc ^= u32::from(b);
        for _ in 0..8 {
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    !crc
}

/// A valid 1x1 RGBA PNG (69 bytes).
pub const TINY_PNG: &[u8] = &[
    0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44, 0x52,
    0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00, 0x00, 0x1f, 0x15, 0xc4,
    0x89, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x44, 0x41, 0x54, 0x78, 0x9c, 0x63, 0xf8, 0xff, 0xff, 0x3f,
    0x00, 0x05, 0xfe, 0x02, 0xfe, 0xa7, 0x35, 0x81, 0x84, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4e,
    0x44, 0xae, 0x42, 0x60, 0x82,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc32b_matches_php() {
        // php -r 'echo hash("crc32b", "hello");' => 3610a686
        assert_eq!(format!("{:08x}", crc32b(b"hello")), "3610a686");
    }

    #[test]
    fn validation_message_suffix() {
        let one: &[&str] = &["A."];
        let two: &[&str] = &["B.", "C."];
        assert_eq!(validation_body(&[("a", one)])["message"], "A.");
        assert_eq!(
            validation_body(&[("a", one), ("b", &["B."])])["message"],
            "A. (and 1 more error)"
        );
        assert_eq!(
            validation_body(&[("a", one), ("b", two)])["message"],
            "A. (and 2 more errors)"
        );
    }

    #[test]
    fn php_pretty_shape() {
        let v = serde_json::json!({"a": "é/x", "b": [], "c": {"d": 1}, "e": "\u{2028}"});
        let s = String::from_utf8(php_pretty(&v)).unwrap();
        assert_eq!(
            s,
            "{\n    \"a\": \"é/x\",\n    \"b\": [],\n    \"c\": {\n        \"d\": 1\n    },\n    \"e\": \"\\u2028\"\n}\n"
        );
    }

    #[test]
    fn dechunk_works() {
        assert_eq!(
            dechunk(b"3\r\nabc\r\n2\r\nde\r\n0\r\n\r\n").unwrap(),
            b"abcde"
        );
    }
}
