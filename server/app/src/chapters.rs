//! Chapters (`ChapterController`). Slice 3.2 ports the reads: R3 index and
//! R4 show. Store, update, destroy and reorder come in slice 3.3.
//!
//! `show` returns `lesson` and `items` from the stored `payload` JSON **as
//! stored** (KNOWN_ISSUES K2, K16): Laravel's `array` cast would turn `{}`
//! into `[]`, objects keyed "0".."n-1" into lists, integers above
//! `PHP_INT_MAX` into floats and `1.0` into `1`. Text Laravel stored is
//! already in that shape, so seeded chapters read the same on both.
//! Fallbacks copy PHP's `??`: a missing or null key gives `lesson: null` /
//! `items: []`; any other value (even `false` or `0`) is returned. A payload
//! that isn't a JSON object, or doesn't parse, counts as empty. serde_json
//! stops at 128 levels of nesting where PHP allows 512 (K17).
//!
//! `GET /api/chapters/reorder` is `show` with id "reorder", as in Laravel
//! (see the routing note in `http.rs`).

use crate::auth::AuthUser;
use crate::error::{AppError, CHAPTER_MODEL};
use crate::http::AppState;
use axum::Json;
use axum::extract::{Path, State};
use serde_json::{Map, Value, json};

/// R3 `GET /api/chapters`: summary rows ordered by `position`, then
/// `number`. The SQL is Eloquent's, verbatim, so SQLite picks the same plan
/// (and the same order for full ties) on the same file.
pub async fn index(
    State(state): State<AppState>,
    _user: AuthUser,
) -> Result<Json<Value>, AppError> {
    let rows = sqlx::query!(
        r#"select "id", "number", "section", "title", "title_em", "position" from "chapters" order by "position" asc, "number" asc"#
    )
    .fetch_all(&state.db)
    .await?;
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            json!({
                "id": r.id,
                "number": r.number,
                "section": r.section,
                "title": r.title,
                "titleEm": r.title_em,
                "position": r.position,
            })
        })
        .collect();
    Ok(Json(Value::Array(list)))
}

/// R4 `GET /api/chapters/{id}`.
pub async fn show(
    State(state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<String>,
) -> Result<Json<Value>, AppError> {
    find_shape(&state, &id).await.map(Json)
}

/// `GET /api/chapters/reorder`: Laravel's `{id}` route with id "reorder".
pub async fn show_reorder(
    State(state): State<AppState>,
    _user: AuthUser,
) -> Result<Json<Value>, AppError> {
    find_shape(&state, "reorder").await.map(Json)
}

/// `findOrFail($id)` + `shape()`.
async fn find_shape(state: &AppState, id: &str) -> Result<Value, AppError> {
    let row = sqlx::query!(
        r#"select "id", "number", "section", "title", "title_em", "position", "payload" as "payload: Vec<u8>"
           from "chapters" where "chapters"."id" = ? limit 1"#,
        id
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::ModelNotFound {
        model: CHAPTER_MODEL,
        id: id.to_string(),
    })?;
    let (lesson, items) = lesson_and_items(&row.payload);
    Ok(json!({
        "id": row.id,
        "number": row.number,
        "section": row.section,
        "title": row.title,
        "titleEm": row.title_em,
        "position": row.position,
        "lesson": lesson,
        "items": items,
    }))
}

/// `$payload['lesson'] ?? null` and `$payload['items'] ?? []` on the stored
/// JSON, kept as stored (module docs).
pub fn lesson_and_items(payload: &[u8]) -> (Value, Value) {
    let mut obj = match serde_json::from_slice::<Value>(payload) {
        Ok(Value::Object(m)) => m,
        _ => Map::new(),
    };
    let mut take = |key: &str| obj.remove(key).filter(|v| !v.is_null());
    let lesson = take("lesson").unwrap_or(Value::Null);
    let items = take("items").unwrap_or_else(|| json!([]));
    (lesson, items)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn li(s: &str) -> (Value, Value) {
        lesson_and_items(s.as_bytes())
    }

    #[test]
    fn fallbacks_copy_php_null_coalescing() {
        let empty = (Value::Null, json!([]));
        for p in [
            "not json",
            "",
            "[1,2]",
            "\"text\"",
            "null",
            "{}",
            r#"{"lesson":null,"items":null}"#,
        ] {
            assert_eq!(li(p), empty, "{p}");
        }
        assert_eq!(
            li(r#"{"lesson":false,"items":0}"#),
            (json!(false), json!(0))
        );
    }

    #[test]
    fn payload_is_returned_as_stored() {
        let (lesson, items) =
            li(r#"{"lesson":{},"items":[{"b":{"0":"x"},"a":1.0,"n":12345678901234567890}]}"#);
        assert_eq!(lesson, json!({}));
        assert_eq!(
            serde_json::to_string(&items).unwrap(),
            r#"[{"b":{"0":"x"},"a":1.0,"n":12345678901234567890}]"#
        );
    }

    #[test]
    fn nesting_beyond_serde_limit_counts_as_empty() {
        let deep = format!(r#"{{"items":{}{}}}"#, "[".repeat(200), "]".repeat(200));
        assert_eq!(li(&deep), (Value::Null, json!([])));
    }
}
