//! Uploads (`UploadController`). Slice 3.1 ports only R9, the list, which
//! the auth contract tests use as their authenticated probe. Store and
//! delete come in slice 3.4.

use crate::auth::AuthUser;
use crate::error::AppError;
use crate::http::AppState;
use axum::Json;
use axum::extract::State;
use serde_json::{Value, json};

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
}
