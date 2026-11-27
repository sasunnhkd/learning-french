//! R12 `POST /api/export` (`ExportController`): every chapter in index
//! order, in the `show` shape, written to `<REPO_ROOT>/src/chapters.json`
//! with PHP's pretty print ([`php::json_pretty`], 4-space indent, K13).
//!
//! - The query is Eloquent's, verbatim (`select * … order by "position"
//!   asc, "number" asc`), so SQLite returns full ties in the same order as
//!   for Laravel. It is a runtime query (not `query!`) because `select *`
//!   can't take column type overrides, and `number`/`position` are read
//!   unchecked: a text `position` left by reorder (K8) reads as PHP's
//!   `(int)` ('x' → 0) instead of failing.
//! - `lesson`/`items` come from the stored payload as stored
//!   ([`chapters::lesson_and_items`], K2, K16, K17), so export equals show.
//! - The file is written like `@file_put_contents`: truncate and write in
//!   place, no temp file. A failed write still answers 200 with
//!   `written: false` (K6, kept); the file is left as it was when it can't
//!   be opened.
//! - `path` is `src/chapters.json`, relative to the repo root (K6, fixed:
//!   Laravel returns the absolute path).

use crate::auth::AuthUser;
use crate::chapters::lesson_and_items;
use crate::error::AppError;
use crate::http::AppState;
use crate::php;
use axum::Json;
use axum::extract::State;
use serde_json::{Value, json};
use sqlx::Row;

/// The `path` in the response.
pub const EXPORT_PATH: &str = "src/chapters.json";

/// R12.
pub async fn run(State(state): State<AppState>, _user: AuthUser) -> Result<Json<Value>, AppError> {
    let rows = sqlx::query(r#"select * from "chapters" order by "position" asc, "number" asc"#)
        .fetch_all(&state.db)
        .await?;
    let mut out = Vec::with_capacity(rows.len());
    for r in &rows {
        let payload: Vec<u8> = r.try_get_unchecked("payload")?;
        let (lesson, items) = lesson_and_items(&payload);
        out.push(json!({
            "id": r.try_get::<String, _>("id")?,
            "number": r.try_get_unchecked::<i64, _>("number")?,
            "section": r.try_get::<String, _>("section")?,
            "title": r.try_get::<String, _>("title")?,
            "titleEm": r.try_get::<Option<String>, _>("title_em")?,
            "position": r.try_get_unchecked::<i64, _>("position")?,
            "lesson": lesson,
            "items": items,
        }));
    }
    let chapters = Value::Array(out);
    let text = php::json_pretty(&chapters);
    let target = state.config.repo_root.join(EXPORT_PATH);
    let written = tokio::task::spawn_blocking(move || match std::fs::write(&target, text) {
        Ok(()) => true,
        Err(e) => {
            tracing::warn!(path = %target.display(), error = %e, "export write failed");
            false
        }
    })
    .await
    .map_err(|e| AppError::Internal(e.into()))?;
    let count = chapters.as_array().map_or(0, Vec::len);
    Ok(Json(json!({
        "count": count,
        "path": EXPORT_PATH,
        "written": written,
        "chapters": chapters,
    })))
}
