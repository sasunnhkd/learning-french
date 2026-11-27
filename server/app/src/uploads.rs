//! Uploads (`UploadController`): R9 list (slice 3.1), R10 store and R11
//! delete (slice 3.4). Files live in `<REPO_ROOT>/public/uploads/`, the
//! SPA's static folder.
//!
//! **Store** validates `file: required|file|max:5120|mimetypes:…` like
//! Laravel ([`validate_file`]), with the approved differences:
//! - SVG is not an accepted type (K4).
//! - The limit is the rule's 5120 KB (K7). Laravel under `php -S` stops at
//!   PHP's `upload_max_filesize` (2M).
//! - The row id is the file name's ULID in lowercase (K3). Laravel's row id
//!   is a second, unrelated ULID.
//!
//! The type is sniffed from the bytes with the rules libmagic 5.46 (PHP
//! `finfo`) applies to the four accepted types ([`sniff`]); the extension
//! follows the type. As in Laravel, a client file name ending in a PHP
//! extension (`x.php`, `x.PHTML `) fails the type rule whatever the bytes.
//!
//! **Which value is "file"** ([`upload_value`]): a text field named `file`
//! wins over a file part (`array_replace_recursive($input, $files,
//! $input)`); of several `file` parts the last wins; a part with an empty
//! file name is no file (`UPLOAD_ERR_NO_FILE`); `file[]`/`file[x]` parts
//! make an array, which isn't a file. `original_name` is the client name
//! after the last `/` or `\`.
//!
//! **Delete** finds the row (404), removes `public/uploads/<basename(path)>`
//! if it is a file, then deletes the row. Rows Laravel made (K3) work too,
//! because the file is found through `path`.

use crate::auth::AuthUser;
use crate::error::{AppError, UPLOAD_MODEL};
use crate::http::AppState;
use crate::input::{Input, UploadedFile};
use crate::php;
use crate::validation::ValidationErrors;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use serde_json::{Value, json};
use std::path::PathBuf;

/// Accepted types, in the rule's order, with their file extension.
const TYPES: [(&str, &str); 4] = [
    ("image/png", "png"),
    ("image/jpeg", "jpg"),
    ("image/webp", "webp"),
    ("image/gif", "gif"),
];

/// Laravel's `max:5120` (kilobytes) on a file: `size / 1024 <= 5120`.
pub const MAX_BYTES: usize = 5120 * 1024;

/// `shouldBlockPhpUpload`: client extensions refused whatever the content.
const PHP_EXTENSIONS: [&str; 8] = [
    "php", "php3", "php4", "php5", "php7", "php8", "phtml", "phar",
];

/// `UploadController::publicUrl`: `rtrim(FRONTEND_BASE_URL, '/')`, then
/// `/`, then the path without leading slashes.
pub fn public_url(base: &str, path: &str) -> String {
    format!(
        "{}/{}",
        base.trim_end_matches('/'),
        path.trim_start_matches('/')
    )
}

/// Eloquent's JSON date: stored `Y-m-d H:i:s[.u]` (UTC) becomes
/// `Y-m-d\TH:i:s.uuuuuu\Z`. Text in any other shape is returned unchanged.
pub fn laravel_json_date(stored: &str) -> String {
    let (main, frac) = stored.split_once('.').unwrap_or((stored, ""));
    let b = main.as_bytes();
    let shape_ok = b.len() == 19
        && b[4] == b'-'
        && b[7] == b'-'
        && (b[10] == b' ' || b[10] == b'T')
        && b[13] == b':'
        && b[16] == b':'
        && frac.len() <= 6
        && frac.bytes().all(|c| c.is_ascii_digit());
    if !shape_ok {
        return stored.to_string();
    }
    format!("{}T{}.{:0<6}Z", &main[..10], &main[11..], frac)
}

/// R9 `GET /api/uploads`.
pub async fn index(
    State(state): State<AppState>,
    _user: AuthUser,
) -> Result<Json<Value>, AppError> {
    let rows = sqlx::query!(
        r#"select id, path, original_name, mime, size, created_at as "created_at: String"
           from uploads order by created_at desc"#
    )
    .fetch_all(&state.db)
    .await?;
    let base = &state.config.frontend_base_url;
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            json!({
                "id": r.id,
                "path": r.path,
                "url": public_url(base, &r.path),
                "original_name": r.original_name,
                "mime": r.mime,
                "size": r.size,
                "created_at": r.created_at.as_deref().map(laravel_json_date),
            })
        })
        .collect();
    Ok(Json(Value::Array(list)))
}

/// The MIME type PHP's `finfo` reports, for the four accepted types only
/// (`None` for anything else, SVG included). libmagic 5.46 rules, probed in
/// slice 3.4: PNG needs the signature plus the IHDR chunk header; JPEG needs
/// `FF D8 FF` and a fourth byte; GIF needs `GIF8`; WebP needs
/// `RIFF????WEBP`.
pub fn sniff(bytes: &[u8]) -> Option<&'static str> {
    const PNG: &[u8] = b"\x89PNG\r\n\x1a\n\x00\x00\x00\x0dIHDR";
    if bytes.starts_with(PNG) {
        Some("image/png")
    } else if bytes.len() >= 4 && bytes.starts_with(b"\xff\xd8\xff") {
        Some("image/jpeg")
    } else if bytes.starts_with(b"GIF8") {
        Some("image/gif")
    } else if bytes.len() >= 12 && bytes.starts_with(b"RIFF") && &bytes[8..12] == b"WEBP" {
        Some("image/webp")
    } else {
        None
    }
}

/// PHP's `basename` for upload names: the text after the last `/` or `\`.
pub fn client_basename(name: &str) -> &str {
    name.rsplit(['/', '\\']).next().unwrap_or(name)
}

/// `pathinfo($name, PATHINFO_EXTENSION)`: the text after the last `.` of
/// the base name, or "" when there is none.
fn client_extension(name: &str) -> &str {
    client_basename(name)
        .rsplit_once('.')
        .map_or("", |(_, ext)| ext)
}

/// What Laravel's validator sees as `file`.
#[derive(Debug)]
pub enum UploadValue<'a> {
    Missing,
    /// A text field, an array, or a JSON value (blank strings already null).
    Field(Value),
    File(&'a UploadedFile),
}

/// See the module docs.
pub fn upload_value(input: &Input) -> UploadValue<'_> {
    if let Some(v) = input.get("file") {
        return match v {
            Value::String(s) if php::is_blank(s) => UploadValue::Field(Value::Null),
            other => UploadValue::Field(other.clone()),
        };
    }
    let last = input.files.iter().rev().find(|(name, _)| {
        name == "file"
            || name
                .strip_prefix("file")
                .is_some_and(|rest| rest.starts_with('['))
    });
    match last {
        None => UploadValue::Missing,
        Some((name, _)) if name != "file" => UploadValue::Field(json!([{}])),
        Some((_, f)) if f.file_name.as_deref().unwrap_or("").is_empty() => {
            UploadValue::Field(Value::Null)
        }
        Some((_, f)) => UploadValue::File(f),
    }
}

fn types_message() -> String {
    let list: Vec<&str> = TYPES.iter().map(|(m, _)| *m).collect();
    format!(
        "The file field must be a file of type: {}.",
        list.join(", ")
    )
}

/// `required|file|max:5120|mimetypes:…` on `file`. Returns the file and
/// its sniffed type when everything passes.
pub fn validate_file<'a>(
    value: &UploadValue<'a>,
) -> Result<(&'a UploadedFile, &'static str), ValidationErrors> {
    let mut errors = ValidationErrors::new();
    let attr = "file";
    match value {
        UploadValue::Missing | UploadValue::Field(Value::Null) => {
            errors.add(attr, "The file field is required.");
        }
        UploadValue::Field(v) => {
            let empty = matches!(v, Value::Array(a) if a.is_empty())
                || matches!(v, Value::Object(o) if o.is_empty());
            if empty {
                errors.add(attr, "The file field is required.");
            } else {
                errors.add(attr, "The file field must be a file.");
                // `max` on a non-file counts characters or items.
                let size = match v {
                    Value::Array(a) => a.len(),
                    Value::Object(o) => o.len(),
                    other => php::to_string(other).unwrap_or_default().chars().count(),
                };
                if size > 5120 {
                    errors.add(
                        attr,
                        "The file field must not be greater than 5120 characters.",
                    );
                }
                errors.add(attr, types_message());
            }
        }
        UploadValue::File(f) => {
            if f.bytes.len() > MAX_BYTES {
                errors.add(
                    attr,
                    "The file field must not be greater than 5120 kilobytes.",
                );
            }
            let name = f.file_name.as_deref().unwrap_or("");
            let blocked = PHP_EXTENSIONS.contains(
                &php::trim(client_extension(client_basename(name)))
                    .to_ascii_lowercase()
                    .as_str(),
            );
            match sniff(&f.bytes) {
                Some(mime) if !blocked && errors.is_empty() => return Ok((f, mime)),
                Some(_) if !blocked => {}
                _ => errors.add(attr, types_message()),
            }
        }
    }
    Err(errors)
}

/// A new ULID (Crockford base32, uppercase): 48-bit Unix milliseconds and
/// 80 random bits, as Symfony's `Ulid` (`Str::ulid()`) makes them.
pub fn new_ulid() -> Result<String, AppError> {
    const ALPHABET: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";
    let ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| AppError::Internal(e.into()))?
        .as_millis();
    let mut random = [0u8; 10];
    getrandom::fill(&mut random)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("getrandom: {e}")))?;
    let random = random
        .iter()
        .fold(0u128, |acc, b| (acc << 8) | u128::from(*b));
    let value = ((ms & 0xFFFF_FFFF_FFFF) << 80) | random;
    Ok((0..26)
        .rev()
        .map(|i| char::from(ALPHABET[((value >> (i * 5)) & 31) as usize]))
        .collect())
}

fn uploads_dir(state: &AppState) -> PathBuf {
    state.config.repo_root.join("public").join("uploads")
}

/// Create `dir` if needed (0755, recursive), run Laravel's realpath
/// containment guard, and write the file (never over an existing one).
fn write_upload(dir: &std::path::Path, file_name: &str, bytes: &[u8]) -> Result<PathBuf, AppError> {
    use std::io::Write;
    if !dir.is_dir() {
        let mut builder = std::fs::DirBuilder::new();
        builder.recursive(true);
        #[cfg(unix)]
        std::os::unix::fs::DirBuilderExt::mode(&mut builder, 0o755);
        builder
            .create(dir)
            .map_err(|e| AppError::Internal(e.into()))?;
    }
    let target = dir.join(file_name);
    let real_dir = std::fs::canonicalize(dir).ok();
    let target_dir = target.parent().and_then(|p| std::fs::canonicalize(p).ok());
    match (real_dir, target_dir) {
        (Some(r), Some(t)) if t.starts_with(&r) => {}
        _ => return Err(AppError::UploadPathOutside),
    }
    let mut out = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&target)
        .map_err(|e| AppError::Internal(e.into()))?;
    if let Err(e) = out.write_all(bytes) {
        drop(out);
        let _ = std::fs::remove_file(&target);
        return Err(AppError::Internal(e.into()));
    }
    Ok(target)
}

/// R10 `POST /api/uploads`: 201 `{id, path, url, mime, size}`.
pub async fn store(
    State(state): State<AppState>,
    _user: AuthUser,
    input: Input,
) -> Result<(StatusCode, Json<Value>), AppError> {
    let value = upload_value(&input);
    let (file, mime) = validate_file(&value).map_err(AppError::Validation)?;
    let ext = TYPES
        .iter()
        .find(|(m, _)| *m == mime)
        .map_or("bin", |(_, e)| *e);
    let ulid = new_ulid()?;
    let file_name = format!("{ulid}.{ext}");
    let relative = format!("uploads/{file_name}");

    let dir = uploads_dir(&state);
    let bytes = file.bytes.clone();
    let target = tokio::task::spawn_blocking(move || write_upload(&dir, &file_name, &bytes))
        .await
        .map_err(|e| AppError::Internal(e.into()))??;

    let id = ulid.to_ascii_lowercase();
    let original_name = client_basename(file.file_name.as_deref().unwrap_or("")).to_string();
    let size = i64::try_from(file.bytes.len()).unwrap_or(i64::MAX);
    let inserted = sqlx::query!(
        r#"insert into "uploads" ("id", "path", "original_name", "mime", "size", "updated_at", "created_at")
           values (?, ?, ?, ?, ?, datetime('now'), datetime('now'))"#,
        id,
        relative,
        original_name,
        mime,
        size
    )
    .execute(&state.db)
    .await;
    if let Err(e) = inserted {
        // Laravel would leave the file behind; don't.
        let _ = tokio::task::spawn_blocking(move || std::fs::remove_file(target)).await;
        return Err(e.into());
    }
    Ok((
        StatusCode::CREATED,
        Json(json!({
            "id": id,
            "path": relative,
            "url": public_url(&state.config.frontend_base_url, &relative),
            "mime": mime,
            "size": size,
        })),
    ))
}

/// R11 `DELETE /api/uploads/{id}`.
pub async fn destroy(
    State(state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<String>,
) -> Result<Json<Value>, AppError> {
    let path = sqlx::query_scalar!(
        r#"select "path" from "uploads" where "uploads"."id" = ? limit 1"#,
        id
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::ModelNotFound {
        model: UPLOAD_MODEL,
        id: id.clone(),
    })?;
    let absolute = uploads_dir(&state).join(php_basename(&path));
    tokio::task::spawn_blocking(move || {
        // `if (is_file($absolute)) @unlink($absolute);`: failures are ignored.
        if absolute.is_file() {
            let _ = std::fs::remove_file(&absolute);
        }
    })
    .await
    .map_err(|e| AppError::Internal(e.into()))?;
    sqlx::query!(r#"delete from "uploads" where "id" = ?"#, id)
        .execute(&state.db)
        .await?;
    Ok(Json(json!({ "ok": true })))
}

/// PHP `basename($path)` on a stored `uploads/<file>` path: the last
/// component, trailing slashes ignored.
fn php_basename(path: &str) -> &str {
    path.trim_end_matches('/').rsplit('/').next().unwrap_or("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urls() {
        assert_eq!(
            public_url("http://localhost:5173/learning-french/", "/uploads/A.png"),
            "http://localhost:5173/learning-french/uploads/A.png"
        );
        assert_eq!(
            public_url("https://x.example", "uploads/A.png"),
            "https://x.example/uploads/A.png"
        );
    }

    #[test]
    fn json_dates() {
        assert_eq!(
            laravel_json_date("2026-11-19 17:02:35"),
            "2026-11-19T17:02:35.000000Z"
        );
        assert_eq!(
            laravel_json_date("2026-11-19 17:02:35.5"),
            "2026-11-19T17:02:35.500000Z"
        );
        assert_eq!(laravel_json_date("garbage"), "garbage");
    }

    #[test]
    fn sniffing_follows_libmagic() {
        let png = b"\x89PNG\r\n\x1a\n\x00\x00\x00\x0dIHDR";
        assert_eq!(sniff(png), Some("image/png"));
        assert_eq!(sniff(&png[..15]), None);
        assert_eq!(sniff(b"\x89PNG\r\n\x1a\nabcdefgh"), None);
        assert_eq!(sniff(b"\xff\xd8\xff\x00"), Some("image/jpeg"));
        assert_eq!(sniff(b"\xff\xd8\xff"), None);
        assert_eq!(sniff(b"GIF8"), Some("image/gif"));
        assert_eq!(sniff(b"GIF9a"), None);
        assert_eq!(sniff(b"RIFF\0\0\0\0WEBP"), Some("image/webp"));
        assert_eq!(sniff(b"RIFF\0\0\0\0WAVE"), None);
        assert_eq!(sniff(b"<svg xmlns=\"http://www.w3.org/2000/svg\"/>"), None);
        assert_eq!(sniff(b""), None);
    }

    #[test]
    fn client_names() {
        assert_eq!(client_basename("a/b\\c.png"), "c.png");
        assert_eq!(client_basename("dir\\x.png"), "x.png");
        assert_eq!(client_basename("plain.png"), "plain.png");
        assert_eq!(client_extension("x.php "), "php ");
        assert_eq!(client_extension("noext"), "");
        assert_eq!(client_extension(".png"), "png");
        assert_eq!(client_extension("a.b/c"), "");
        assert_eq!(php_basename("uploads/A.png"), "A.png");
        assert_eq!(php_basename("uploads/x/"), "x");
    }

    #[test]
    fn ulids() {
        let a = new_ulid().unwrap();
        let b = new_ulid().unwrap();
        assert_eq!(a.len(), 26);
        assert_ne!(a, b);
        assert!(
            a.bytes()
                .all(|c| b"0123456789ABCDEFGHJKMNPQRSTVWXYZ".contains(&c))
        );
        // The first 10 characters encode the time, so they sort by it.
        assert!(a[..10] <= b[..10]);
        assert!(a.as_bytes()[0] <= b'7', "48-bit timestamp");
    }

    fn file(name: &str, bytes: impl Into<axum::body::Bytes>) -> UploadedFile {
        UploadedFile {
            file_name: Some(name.to_string()),
            content_type: None,
            bytes: bytes.into(),
        }
    }

    fn errors(v: &UploadValue<'_>) -> Value {
        validate_file(v).expect_err("invalid").body()["errors"]["file"].clone()
    }

    #[test]
    fn file_rules() {
        let types = types_message();
        let png = file("a.png", &b"\x89PNG\r\n\x1a\n\x00\x00\x00\x0dIHDR"[..]);
        assert!(validate_file(&UploadValue::File(&png)).is_ok());
        assert_eq!(
            errors(&UploadValue::Missing),
            json!(["The file field is required."])
        );
        assert_eq!(
            errors(&UploadValue::Field(json!([]))),
            json!(["The file field is required."])
        );
        assert_eq!(
            errors(&UploadValue::Field(json!("x"))),
            json!(["The file field must be a file.", types])
        );
        assert_eq!(
            errors(&UploadValue::Field(json!("a".repeat(5121)))),
            json!([
                "The file field must be a file.",
                "The file field must not be greater than 5120 characters.",
                types
            ])
        );
        let php = file("x.PHP ", &b"\x89PNG\r\n\x1a\n\x00\x00\x00\x0dIHDR"[..]);
        assert_eq!(errors(&UploadValue::File(&php)), json!([types]));
        let empty = file("e.png", &b""[..]);
        assert_eq!(errors(&UploadValue::File(&empty)), json!([types]));
        let mut big_bytes = vec![0u8; MAX_BYTES + 1];
        big_bytes[..4].copy_from_slice(b"GIF8");
        let big = file("b.gif", big_bytes.clone());
        assert_eq!(
            errors(&UploadValue::File(&big)),
            json!(["The file field must not be greater than 5120 kilobytes."])
        );
        big_bytes.pop();
        let ok = file("b.gif", big_bytes);
        assert!(validate_file(&UploadValue::File(&ok)).is_ok());
    }

    #[test]
    fn which_value_is_file() {
        let png = file("b.png", &b"x"[..]);
        let txt = file("a.txt", &b"y"[..]);
        let mut input = Input::default();
        assert!(matches!(upload_value(&input), UploadValue::Missing));
        input.files = vec![("file".into(), txt.clone()), ("file".into(), png.clone())];
        assert!(
            matches!(upload_value(&input), UploadValue::File(f) if f.file_name.as_deref() == Some("b.png"))
        );
        input.files = vec![("file".into(), file("", &b"x"[..]))];
        assert!(matches!(
            upload_value(&input),
            UploadValue::Field(Value::Null)
        ));
        input.files = vec![("file[]".into(), png.clone())];
        assert!(matches!(
            upload_value(&input),
            UploadValue::Field(Value::Array(_))
        ));
        input.files = vec![("files".into(), png.clone())];
        assert!(matches!(upload_value(&input), UploadValue::Missing));
        input.files = vec![("file".into(), png)];
        input.fields.insert("file".into(), json!("text"));
        assert!(matches!(
            upload_value(&input),
            UploadValue::Field(Value::String(_))
        ));
        input.fields.insert("file".into(), json!(" "));
        assert!(matches!(
            upload_value(&input),
            UploadValue::Field(Value::Null)
        ));
    }
}
