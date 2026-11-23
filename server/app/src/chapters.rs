//! Chapters (`ChapterController`): R3 index and R4 show (slice 3.2), R5
//! store, R6 update, R7 destroy and R8 reorder (slice 3.3).
//!
//! **Writes.** The body is validated as Laravel's validator would see it
//! ([`validation_view`]: blank strings are null, nothing is trimmed) with
//! the rules of `validatePayload` ([`chapter_rules`]); then the **raw**
//! input is stored, so `lesson` and `items` keep every key and every string
//! exactly as sent (K1, K2). `payload` holds `{id, number, section, title,
//! titleEm, lesson, items}` as sent (`number` too, K10); the columns hold
//! the integer `number` and `position` (PHP `(int)`, [`php::int_value`]).
//! `position`: a non-null value wins, else an update keeps the stored one,
//! else `max(position) + 1` (0 on an empty table). A blank `position`
//! counts as null, as in Laravel. Like Laravel, store checks `unique` and
//! `max(position)` outside a transaction, so two racing stores can collide
//! (a 500 for the loser).
//!
//! **Reorder.** In one transaction, each `order` entry sets `position` to
//! its key, bound as text so SQLite's integer affinity stores what PDO
//! would: list indexes and integer-like keys become integers, other object
//! keys stay text (K8). Plain column reads (`index`, `show`) decode such
//! text the way SQLite's `sqlite3_value_int64` does, which matches PHP's
//! `(int)` ('x' → 0). A read with a type override (`"x: i64"`) or of an
//! expression fails on text instead (a 500), so those queries use
//! `cast(... as integer)`: `max(position)` in `to_row` and the stored
//! position in `update_chapter`. Export (slice 3.5) must do the same.
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
//! GET, PUT and DELETE `/api/chapters/reorder` are show, update and
//! destroy with id "reorder", as in Laravel (see the routing note in
//! `http.rs`).

use crate::auth::AuthUser;
use crate::error::{AppError, CHAPTER_MODEL};
use crate::http::AppState;
use crate::input::Input;
use crate::php;
use crate::validation::{Pattern, Rule, ValidationErrors, validate, validation_view};
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use serde_json::{Map, Value, json};

const ITEM_TYPES: &[&str] = &["fill", "qcm", "transform"];

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
    .ok_or_else(|| not_found(id))?;
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

/// `validatePayload()`. `unique` is `Some(taken)` on store.
pub fn chapter_rules(unique: Option<bool>) -> Vec<(&'static str, Vec<Rule>)> {
    use Rule::{Array, In, Integer, Max, Min, Nullable, Present, Regex, Required, Sometimes};
    let mut id = vec![Required, Rule::String, Max(120), Regex(Pattern::Slug)];
    if let Some(taken) = unique {
        id.push(Rule::Unique { taken });
    }
    let string = Rule::String;
    vec![
        ("id", id),
        ("number", vec![Required, Integer, Min(1)]),
        ("section", vec![Required, string, Max(120)]),
        ("title", vec![Required, string, Max(200)]),
        ("titleEm", vec![Nullable, string, Max(200)]),
        ("position", vec![Sometimes, Nullable, Integer]),
        ("lesson", vec![Present, Array]),
        ("lesson.intro", vec![Nullable, string]),
        ("lesson.formation", vec![Present, Array]),
        ("lesson.formation.*", vec![Nullable, string]),
        ("lesson.irregulars", vec![Present, Array]),
        ("lesson.irregulars.*", vec![Array]),
        ("lesson.irregulars.*.v", vec![Nullable, string]),
        ("lesson.irregulars.*.forms", vec![Nullable, string]),
        ("lesson.triggers", vec![Present, Array]),
        ("lesson.triggers.*", vec![Array]),
        ("lesson.triggers.*.title", vec![Nullable, string]),
        ("lesson.triggers.*.body", vec![Nullable, string]),
        ("lesson.triggers.*.note", vec![Nullable, string]),
        ("lesson.triggers.*.subcases", vec![Nullable, Array]),
        ("lesson.triggers.*.subcases.*", vec![Array]),
        ("lesson.triggers.*.subcases.*.label", vec![Nullable, string]),
        (
            "lesson.triggers.*.subcases.*.example",
            vec![Nullable, string],
        ),
        ("items", vec![Present, Array]),
        ("items.*", vec![Array]),
        ("items.*.type", vec![Nullable, In(ITEM_TYPES)]),
        ("items.*.rule", vec![Nullable, string]),
        ("items.*.consolidation", vec![Nullable, Array]),
    ]
}

/// The values a save writes (`toRow()`).
struct Row {
    id: String,
    number: i64,
    section: String,
    title: String,
    title_em: Option<String>,
    position: i64,
    payload: String,
}

fn view_str(view: &Map<String, Value>, key: &str) -> Result<String, AppError> {
    match view.get(key) {
        Some(Value::String(s)) => Ok(s.clone()),
        _ => Err(AppError::Internal(anyhow::anyhow!(
            "validated {key} is not a string"
        ))),
    }
}

/// `toRow()` on validated input. `existing_position` is the stored
/// position on update.
async fn to_row(
    state: &AppState,
    id: &str,
    raw: &Map<String, Value>,
    view: &Map<String, Value>,
    existing_position: Option<i64>,
) -> Result<Row, AppError> {
    let number = view
        .get("number")
        .and_then(php::int_value)
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("validated number is not an integer")))?;
    let position = match view.get("position").filter(|v| !v.is_null()) {
        Some(v) => php::int_value(v).ok_or_else(|| {
            AppError::Internal(anyhow::anyhow!("validated position is not an integer"))
        })?,
        None => match existing_position {
            Some(p) => p,
            None => {
                // `(int) (Chapter::max('position') ?? -1) + 1`. The cast is
                // PHP's `(int)` when reorder left text in a position ('x' →
                // 0); sqlx won't decode a text `max()` as an integer.
                let max = sqlx::query_scalar!(
                    r#"select cast(max("position") as integer) as "max: i64" from "chapters""#
                )
                .fetch_one(&state.db)
                .await?;
                // Saturates where PHP would turn i64::MAX + 1 into a float.
                max.unwrap_or(-1).saturating_add(1)
            }
        },
    };
    let title_em = match raw.get("titleEm") {
        Some(Value::String(s)) => Some(s.clone()),
        _ => None,
    };
    let field = |k: &str| raw.get(k).cloned().unwrap_or(Value::Null);
    let payload = json!({
        "id": id,
        "number": field("number"),
        "section": field("section"),
        "title": field("title"),
        "titleEm": title_em,
        "lesson": field("lesson"),
        "items": raw.get("items").cloned().unwrap_or_else(|| json!([])),
    });
    Ok(Row {
        id: id.to_string(),
        number,
        // Validated non-blank strings: the view holds them unchanged.
        section: view_str(view, "section")?,
        title: view_str(view, "title")?,
        title_em,
        position,
        payload: serde_json::to_string(&payload)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("payload: {e}")))?,
    })
}

/// `shape()` of a row just written.
fn shape(row: &Row) -> Value {
    let (lesson, items) = lesson_and_items(row.payload.as_bytes());
    json!({
        "id": row.id,
        "number": row.number,
        "section": row.section,
        "title": row.title,
        "titleEm": row.title_em,
        "position": row.position,
        "lesson": lesson,
        "items": items,
    })
}

/// R5 `POST /api/chapters`: 201 with the chapter.
pub async fn store(
    State(state): State<AppState>,
    _user: AuthUser,
    input: Input,
) -> Result<(StatusCode, Json<Value>), AppError> {
    let view = validation_view(&input.fields);
    // `unique:chapters,id` needs the database, so look the id up first.
    let taken = match view.get("id") {
        Some(Value::String(id)) => sqlx::query_scalar!(
            r#"select count(*) as "n: i64" from "chapters" where "id" = ?"#,
            id
        )
        .fetch_one(&state.db)
        .await?
        .is_positive(),
        _ => false,
    };
    validate(&view, &chapter_rules(Some(taken))).into_result()?;
    let id = view_str(&view, "id")?;
    let row = to_row(&state, &id, &input.fields, &view, None).await?;
    sqlx::query!(
        r#"insert into "chapters" ("id", "number", "section", "title", "title_em", "position", "payload", "updated_at", "created_at")
           values (?, ?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'))"#,
        row.id,
        row.number,
        row.section,
        row.title,
        row.title_em,
        row.position,
        row.payload
    )
    .execute(&state.db)
    .await?;
    Ok((StatusCode::CREATED, Json(shape(&row))))
}

/// R6 `PUT /api/chapters/{id}`: 404 before validation; the body's `id`
/// must equal the route id.
pub async fn update(
    State(state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<String>,
    input: Input,
) -> Result<Json<Value>, AppError> {
    update_chapter(&state, &id, &input).await
}

/// `PUT /api/chapters/reorder`: update with id "reorder".
pub async fn update_reorder(
    State(state): State<AppState>,
    _user: AuthUser,
    input: Input,
) -> Result<Json<Value>, AppError> {
    update_chapter(&state, "reorder", &input).await
}

async fn update_chapter(
    state: &AppState,
    id: &str,
    input: &Input,
) -> Result<Json<Value>, AppError> {
    let existing = sqlx::query_scalar!(
        // `(int) $existing->position`; the cast reads a text position
        // left by reorder as PHP does ('x' → 0), where a typed read fails.
        r#"select cast("position" as integer) as "position!: i64" from "chapters" where "chapters"."id" = ? limit 1"#,
        id
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| not_found(id))?;
    let view = validation_view(&input.fields);
    validate(&view, &chapter_rules(None)).into_result()?;
    if view_str(&view, "id")? != id {
        return Err(AppError::Validation(ValidationErrors::single(
            "id",
            "The id of an existing chapter cannot be changed.",
        )));
    }
    let row = to_row(state, id, &input.fields, &view, Some(existing)).await?;
    sqlx::query!(
        r#"update "chapters" set "number" = ?, "section" = ?, "title" = ?, "title_em" = ?, "position" = ?, "payload" = ?, "updated_at" = datetime('now')
           where "id" = ?"#,
        row.number,
        row.section,
        row.title,
        row.title_em,
        row.position,
        row.payload,
        row.id
    )
    .execute(&state.db)
    .await?;
    // `$chapter->refresh()`
    find_shape(state, id).await.map(Json)
}

/// R7 `DELETE /api/chapters/{id}`.
pub async fn destroy(
    State(state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<String>,
) -> Result<Json<Value>, AppError> {
    destroy_chapter(&state, &id).await
}

/// `DELETE /api/chapters/reorder`: destroy with id "reorder".
pub async fn destroy_reorder(
    State(state): State<AppState>,
    _user: AuthUser,
) -> Result<Json<Value>, AppError> {
    destroy_chapter(&state, "reorder").await
}

async fn destroy_chapter(state: &AppState, id: &str) -> Result<Json<Value>, AppError> {
    let done = sqlx::query!(r#"delete from "chapters" where "id" = ?"#, id)
        .execute(&state.db)
        .await?;
    if done.rows_affected() == 0 {
        return Err(not_found(id));
    }
    Ok(Json(json!({ "ok": true })))
}

/// R8 `POST /api/chapters/reorder`.
pub async fn reorder(
    State(state): State<AppState>,
    _user: AuthUser,
    input: Input,
) -> Result<Json<Value>, AppError> {
    let view = validation_view(&input.fields);
    validate(
        &view,
        &[
            ("order", vec![Rule::Required, Rule::Array]),
            ("order.*", vec![Rule::String]),
        ],
    )
    .into_result()?;
    // `foreach ($data['order'] as $position => $id)`: list index or
    // object key.
    let entries: Vec<(String, String)> = match view.get("order") {
        Some(Value::Array(items)) => items
            .iter()
            .enumerate()
            .filter_map(|(i, v)| v.as_str().map(|s| (i.to_string(), s.to_string())))
            .collect(),
        Some(Value::Object(map)) => map
            .iter()
            .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
            .collect(),
        _ => Vec::new(),
    };
    let mut tx = state.db.begin().await?;
    for (position, id) in &entries {
        sqlx::query!(
            r#"update "chapters" set "position" = ?, "updated_at" = datetime('now') where "id" = ?"#,
            position,
            id
        )
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    Ok(Json(json!({ "ok": true })))
}

fn not_found(id: &str) -> AppError {
    AppError::ModelNotFound {
        model: CHAPTER_MODEL,
        id: id.to_string(),
    }
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

    /// Error bodies captured from Laravel in slice 3.3 (probe of the
    /// sandboxed app), replayed through the Rust validator.
    #[test]
    fn validation_matches_captured_laravel_errors() {
        let base = |patch: Value| {
            let mut p = json!({
                "id": "x", "number": 1, "section": "s", "title": "t",
                "lesson": { "formation": [], "irregulars": [], "triggers": [] },
                "items": []
            });
            for (k, v) in patch.as_object().unwrap() {
                p[k] = v.clone();
            }
            p
        };
        let lesson = |patch: Value| {
            let mut l = json!({ "formation": [], "irregulars": [], "triggers": [] });
            for (k, v) in patch.as_object().unwrap() {
                l[k] = v.clone();
            }
            json!({ "lesson": l })
        };
        let cases: Vec<(Value, Value)> = vec![
            (
                base(
                    json!({ "items": null, "lesson": { "formation": [1], "irregulars": [], "triggers": [] } }),
                ),
                json!({ "items": ["The items field must be an array."], "lesson.formation.0": ["The lesson.formation.0 field must be a string."] }),
            ),
            (
                base(json!({ "items": [{ "rule": ["x"] }, { "rule": 5 }] })),
                json!({ "items.0.rule": ["The items.0.rule field must be a string."], "items.1.rule": ["The items.1.rule field must be a string."] }),
            ),
            (
                base(lesson(json!({ "formation": [["a"], 5] }))),
                json!({ "lesson.formation.1": ["The lesson.formation.1 field must be a string."], "lesson.formation.0": ["The lesson.formation.0 field must be a string."] }),
            ),
            (
                base(json!({ "items": [{ "rule": 1 }, "z"] })),
                json!({ "items.1": ["The items.1 field must be an array."], "items.0.rule": ["The items.0.rule field must be a string."] }),
            ),
            (
                base(lesson(
                    json!({ "triggers": [{ "subcases": [{ "label": [1] }, "y", { "label": 2 }] }, "t", { "subcases": "s" }] }),
                )),
                json!({
                    "lesson.triggers.1": ["The lesson.triggers.1 field must be an array."],
                    "lesson.triggers.2.subcases": ["The lesson.triggers.2.subcases field must be an array."],
                    "lesson.triggers.0.subcases.1": ["The lesson.triggers.0.subcases.1 field must be an array."],
                    "lesson.triggers.0.subcases.0.label": ["The lesson.triggers.0.subcases.0.label field must be a string."],
                    "lesson.triggers.0.subcases.2.label": ["The lesson.triggers.0.subcases.2.label field must be a string."]
                }),
            ),
            (
                base(json!({ "id": 1.5 })),
                json!({ "id": ["The id field must be a string.", "The id field format is invalid."] }),
            ),
            (
                base(json!({ "id": true })),
                json!({ "id": ["The id field must be a string.", "The id field format is invalid."] }),
            ),
            (
                base(json!({ "id": -5 })),
                json!({ "id": ["The id field must be a string."] }),
            ),
            (
                base(json!({ "id": 1e20 })),
                json!({ "id": ["The id field must be a string.", "The id field format is invalid."] }),
            ),
            (
                base(json!({ "id": 1e13 })),
                json!({ "id": ["The id field must be a string."] }),
            ),
            (
                base(json!({ "id": 1e14 })),
                json!({ "id": ["The id field must be a string.", "The id field format is invalid."] }),
            ),
            (
                base(json!({ "id": false })),
                json!({ "id": ["The id field must be a string.", "The id field format is invalid."] }),
            ),
            (
                base(json!({ "id": vec!["a"; 121] })),
                json!({ "id": ["The id field must be a string.", "The id field must not be greater than 120 characters.", "The id field format is invalid."] }),
            ),
            (
                base(json!({ "number": false })),
                json!({ "number": ["The number field must be an integer.", "The number field must be at least 1."] }),
            ),
            (
                base(json!({ "number": 0.5 })),
                json!({ "number": ["The number field must be an integer.", "The number field must be at least 1."] }),
            ),
            (
                base(json!({ "number": 1e-7 })),
                json!({ "number": ["The number field must be an integer.", "The number field must be at least 1."] }),
            ),
            (
                base(json!({ "number": "1." })),
                json!({ "number": ["The number field must be an integer."] }),
            ),
            (
                base(json!({ "number": [1] })),
                json!({ "number": ["The number field must be an integer."] }),
            ),
            (
                base(json!({ "number": "9223372036854775808" })),
                json!({ "number": ["The number field must be an integer."] }),
            ),
            (
                base(json!({ "number": 1e20 })),
                json!({ "number": ["The number field must be an integer."] }),
            ),
            (
                base(json!({ "number": " 1e3 " })),
                json!({ "number": ["The number field must be an integer."] }),
            ),
            (
                base(json!({ "number": "-0" })),
                json!({ "number": ["The number field must be at least 1."] }),
            ),
            (base(json!({ "number": 0.999_999_999_999_999 })), json!({})),
            (base(json!({ "number": "+5" })), json!({})),
            (base(json!({ "number": "\u{b}4\n" })), json!({})),
            (
                base(json!({ "section": vec!["a"; 121] })),
                json!({ "section": ["The section field must be a string.", "The section field must not be greater than 120 characters."] }),
            ),
            (
                base(json!({ "items": [{ "type": ["fill"] }] })),
                json!({ "items.0.type": ["The selected items.0.type is invalid."] }),
            ),
            (
                base(json!({ "items": [{ "type": true }] })),
                json!({ "items.0.type": ["The selected items.0.type is invalid."] }),
            ),
            (base(json!({ "items": [{ "type": "\u{a0}" }] })), json!({})),
            (
                base(json!({ "title": "\u{a0}" })),
                json!({ "title": ["The title field is required."] }),
            ),
            (
                base(json!({ "title": "\u{ad}" })),
                json!({ "title": ["The title field is required."] }),
            ),
            (base(json!({ "position": "\u{a0}" })), json!({})),
            (
                base(lesson(json!({ "formation": { "a.b": 1, "c*": 2 } }))),
                json!({ "lesson.formation.a.b": ["The lesson.formation.a.b field must be a string."], "lesson.formation.c*": ["The lesson.formation.c* field must be a string."] }),
            ),
            (
                base(json!({ "items": { "": { "rule": 1 } } })),
                json!({ "items..rule": ["The items..rule field must be a string."] }),
            ),
            (
                base(json!({ "items": { "1": 5, "0": 6 } })),
                json!({ "items.1": ["The items.1 field must be an array."], "items.0": ["The items.0 field must be an array."] }),
            ),
            (
                base(json!({ "id": "SUBJONCTIF-PRESENT" })),
                json!({ "id": ["The id field format is invalid."] }),
            ),
            (
                json!({ "id": 5, "number": false, "lesson": null, "items": null }),
                json!({
                    "id": ["The id field must be a string."],
                    "number": ["The number field must be an integer.", "The number field must be at least 1."],
                    "section": ["The section field is required."],
                    "title": ["The title field is required."],
                    "lesson": ["The lesson field must be an array."],
                    "lesson.formation": ["The lesson.formation field must be present."],
                    "lesson.irregulars": ["The lesson.irregulars field must be present."],
                    "lesson.triggers": ["The lesson.triggers field must be present."],
                    "items": ["The items field must be an array."]
                }),
            ),
            (
                base(json!({ "lesson": ["a"] })),
                json!({
                    "lesson.formation": ["The lesson.formation field must be present."],
                    "lesson.irregulars": ["The lesson.irregulars field must be present."],
                    "lesson.triggers": ["The lesson.triggers field must be present."]
                }),
            ),
        ];
        for (payload, want) in cases {
            let view = validation_view(payload.as_object().unwrap());
            let got = validate(&view, &chapter_rules(None)).body()["errors"].clone();
            assert_eq!(got, want, "{payload}");
            assert_eq!(
                got.as_object().unwrap().keys().collect::<Vec<_>>(),
                want.as_object().unwrap().keys().collect::<Vec<_>>(),
                "key order for {payload}"
            );
        }
    }

    #[test]
    fn unique_runs_only_without_earlier_id_errors() {
        let mut p = json!({
            "id": "taken", "number": 1, "section": "s", "title": "t",
            "lesson": { "formation": [], "irregulars": [], "triggers": [] }, "items": []
        });
        let errors = |p: &Value| {
            let view = validation_view(p.as_object().unwrap());
            validate(&view, &chapter_rules(Some(true))).body()["errors"].clone()
        };
        assert_eq!(
            errors(&p),
            json!({ "id": ["The id has already been taken."] })
        );
        p["id"] = json!("Taken");
        assert_eq!(
            errors(&p),
            json!({ "id": ["The id field format is invalid."] })
        );
    }
}
