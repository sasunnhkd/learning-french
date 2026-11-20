//! R3 index and R4 show, checked against the seeded content
//! (`src/chapters.json` copied into the sandbox seed).

use contract::{Ctx, keys};
use serde_json::{Value, json};

const SUMMARY_KEYS: [&str; 6] = ["id", "number", "section", "title", "titleEm", "position"];
const CHAPTER_KEYS: [&str; 8] = [
    "id", "number", "section", "title", "titleEm", "position", "lesson", "items",
];

/// The seeder stores `position = row.position ?? index`.
fn seed_position(row: &Value, index: usize) -> i64 {
    row["position"]
        .as_i64()
        .unwrap_or(i64::try_from(index).unwrap())
}

#[test]
fn index_shape_order_and_values() {
    let c = Ctx::from_env();
    let t = c.login();
    let body = c.get("/api/chapters").bearer(&t).send().expect(200);
    let rows = body.as_array().expect("array");
    let seed = c.seed_chapters();
    assert_eq!(rows.len(), seed.len());

    for row in rows {
        assert_eq!(keys(row), SUMMARY_KEYS);
    }
    // ORDER BY position, number
    let order: Vec<(i64, i64)> = rows
        .iter()
        .map(|r| {
            (
                r["position"].as_i64().unwrap(),
                r["number"].as_i64().unwrap(),
            )
        })
        .collect();
    let mut sorted = order.clone();
    sorted.sort_unstable();
    assert_eq!(order, sorted, "rows sorted by position, then number");

    for (i, s) in seed.iter().enumerate() {
        let id = s["id"].as_str().unwrap();
        let row = rows
            .iter()
            .find(|r| r["id"] == id)
            .unwrap_or_else(|| panic!("{id} missing"));
        let want = json!({
            "id": id,
            "number": s["number"],
            "section": s["section"],
            "title": s["title"],
            "titleEm": s.get("titleEm").cloned().unwrap_or(Value::Null),
            "position": seed_position(s, i),
        });
        assert_eq!(row, &want, "index row {id}");
    }
}

#[test]
fn show_matches_seed_for_every_chapter() {
    let c = Ctx::from_env();
    let t = c.login();
    for (i, s) in c.seed_chapters().iter().enumerate() {
        let id = s["id"].as_str().unwrap();
        let body = c
            .get(&format!("/api/chapters/{id}"))
            .bearer(&t)
            .send()
            .expect(200);
        assert_eq!(keys(&body), CHAPTER_KEYS, "{id}");
        assert_eq!(body["id"], id);
        assert_eq!(body["number"], s["number"], "{id} number");
        assert_eq!(body["section"], s["section"], "{id} section");
        assert_eq!(body["title"], s["title"], "{id} title");
        assert_eq!(
            body["titleEm"],
            s.get("titleEm").cloned().unwrap_or(Value::Null),
            "{id} titleEm"
        );
        assert_eq!(body["position"], seed_position(s, i), "{id} position");
        assert_eq!(
            body["lesson"],
            s.get("lesson").cloned().unwrap_or(Value::Null),
            "{id} lesson"
        );
        assert_eq!(
            body["items"],
            s.get("items").cloned().unwrap_or(json!([])),
            "{id} items"
        );
    }
}

#[test]
fn show_unknown_chapter_is_404() {
    let c = Ctx::from_env();
    let t = c.login();
    let body = c
        .get("/api/chapters/does-not-exist")
        .bearer(&t)
        .send()
        .expect(404);
    assert_eq!(
        body,
        json!({ "message": "No query results for model [App\\Models\\Chapter] does-not-exist" })
    );
}

/// `reorder` only has POST; Laravel matches other methods on
/// `/api/chapters/{id}` with id "reorder". Rust must route them the same way.
#[test]
fn reorder_path_falls_back_to_id_routes() {
    let c = Ctx::from_env();
    let t = c.login();
    let want = json!({ "message": "No query results for model [App\\Models\\Chapter] reorder" });
    for method in ["GET", "PUT", "DELETE"] {
        let r = c
            .req(method, "/api/chapters/reorder")
            .bearer(&t)
            .json(&json!({}))
            .send();
        assert_eq!(r.expect(404), want, "{method}");
    }
}
