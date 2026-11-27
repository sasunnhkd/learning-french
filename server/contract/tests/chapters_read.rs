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
    // ORDER BY position, number. The exact id sequence is pinned, so a
    // different tie-break would fail too. The seed has no full
    // (position, number) ties; seed order breaks any that appear, which is
    // SQLite's scan order for Eloquent's SQL on a freshly seeded table.
    let mut want_order: Vec<(i64, i64, usize, &str)> = seed
        .iter()
        .enumerate()
        .map(|(i, s)| {
            (
                seed_position(s, i),
                s["number"].as_i64().unwrap(),
                i,
                s["id"].as_str().unwrap(),
            )
        })
        .collect();
    want_order.sort_unstable();
    let got: Vec<&str> = rows.iter().map(|r| r["id"].as_str().unwrap()).collect();
    let want: Vec<&str> = want_order.iter().map(|w| w.3).collect();
    assert_eq!(got, want, "rows sorted by position, then number");

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

/// Deletes the fixture rows on drop, so a failed assertion can't leave
/// extra chapters behind for the other (serial) tests.
struct FixtureRows<'a>(&'a Ctx);

impl Drop for FixtureRows<'_> {
    fn drop(&mut self) {
        self.0
            .sqlite("delete from chapters where id like 'zz-contract-%'");
    }
}

fn insert_fixture(c: &Ctx, id: &str, position: i64, payload: &str) {
    c.sqlite(&format!(
        "insert into chapters (id, number, section, title, title_em, payload, position, created_at, updated_at) \
         values ('{id}', 7, 'S', 'T', null, '{payload}', {position}, '2026-01-01 00:00:00', '2026-01-01 00:00:00')"
    ));
}

fn show(c: &Ctx, t: &str, id: &str) -> Value {
    c.get(&format!("/api/chapters/{id}"))
        .bearer(t)
        .send()
        .expect(200)
}

/// Laravel reads `payload` with Eloquent's `array` cast
/// (`json_decode($v, true)`) and answers with `json_encode`, so PHP's array
/// model shows through: `{}` becomes `[]`, an object keyed "0".."n-1" in
/// order becomes a list, integers above `PHP_INT_MAX` become floats, whole
/// floats lose their `.0`. Rust answers with the stored JSON as it is (K2,
/// K16), so what a save stores is what a read returns. Text Laravel itself
/// stores is already in PHP's shape, so the seed reads the same on both.
/// The API can't store these shapes through Laravel, hence the raw SQL.
#[test]
fn show_returns_stored_payload() {
    let c = Ctx::from_env();
    let t = c.login();
    let _cleanup = FixtureRows(&c);
    insert_fixture(
        &c,
        "zz-contract-shapes",
        1001,
        r#"{"lesson":{},"items":[{"list":{"0":"a","1":"b"},"sparse":{"1":"a"},"swapped":{"1":"a","0":"b"},"empty":{},"nested":[{}]}]}"#,
    );
    insert_fixture(
        &c,
        "zz-contract-numbers",
        1000,
        r#"{"lesson":null,"items":[{"big":12345678901234567890,"max":9223372036854775807,"float":1.0,"exp":1e2,"neg":-5,"frac":0.1}]}"#,
    );
    // `??` only falls back on a missing key or null.
    insert_fixture(
        &c,
        "zz-contract-falsy",
        1000,
        r#"{"lesson":false,"items":0}"#,
    );
    insert_fixture(&c, "zz-contract-garbage", 1000, "not json");
    insert_fixture(
        &c,
        "zz-contract-nulls",
        1000,
        r#"{"lesson":null,"items":null}"#,
    );
    insert_fixture(&c, "zz-contract-list", 1000, "[1,2]");
    insert_fixture(&c, "zz-contract-scalar", 1000, r#""text""#);

    let body = show(&c, &t, "zz-contract-shapes");
    assert_eq!(keys(&body), CHAPTER_KEYS);
    let want = c.expect_by_target(
        "K2",
        json!({
            "lesson": [],
            "items": [{
                "list": ["a", "b"],
                "sparse": { "1": "a" },
                "swapped": { "1": "a", "0": "b" },
                "empty": [],
                "nested": [[]],
            }],
        }),
        json!({
            "lesson": {},
            "items": [{
                "list": { "0": "a", "1": "b" },
                "sparse": { "1": "a" },
                "swapped": { "1": "a", "0": "b" },
                "empty": {},
                "nested": [{}],
            }],
        }),
    );
    assert_eq!(body["lesson"], want["lesson"], "lesson");
    assert_eq!(body["items"], want["items"], "items");
    assert_eq!(
        keys(&body["items"][0]["swapped"]),
        ["1", "0"],
        "key order kept"
    );
    assert_eq!(
        json!([
            body["id"],
            body["number"],
            body["section"],
            body["title"],
            body["titleEm"],
            body["position"]
        ]),
        json!(["zz-contract-shapes", 7, "S", "T", null, 1001])
    );

    let body = show(&c, &t, "zz-contract-numbers");
    let want = c.expect_by_target(
        "K16",
        json!({
            "big": 12_345_678_901_234_567_890.0_f64,
            "max": 9_223_372_036_854_775_807_i64,
            "float": 1,
            "exp": 100,
            "neg": -5,
            "frac": 0.1,
        }),
        json!({
            "big": 12_345_678_901_234_567_890_u64,
            "max": 9_223_372_036_854_775_807_i64,
            "float": 1.0,
            "exp": 100.0,
            "neg": -5,
            "frac": 0.1,
        }),
    );
    assert_eq!(body["items"], json!([want]));

    let body = show(&c, &t, "zz-contract-falsy");
    assert_eq!(
        (&body["lesson"], &body["items"]),
        (&json!(false), &json!(0))
    );

    for id in [
        "zz-contract-garbage",
        "zz-contract-nulls",
        "zz-contract-list",
        "zz-contract-scalar",
    ] {
        let body = show(&c, &t, id);
        assert_eq!(body["lesson"], Value::Null, "{id} lesson");
        assert_eq!(body["items"], json!([]), "{id} items");
    }

    let index = c.get("/api/chapters").bearer(&t).send().expect(200);
    let last = index.as_array().unwrap().last().unwrap().clone();
    assert_eq!(
        last,
        json!({ "id": "zz-contract-shapes", "number": 7, "section": "S", "title": "T", "titleEm": null, "position": 1001 }),
        "the highest position sorts last"
    );
}

/// PHP's `json_decode` allows 512 levels of nesting; serde_json stops at
/// 128 and Rust then treats the payload as undecodable (K17). The seed
/// nests at most 6 deep.
#[test]
fn show_deeply_nested_payload() {
    let c = Ctx::from_env();
    let t = c.login();
    let _cleanup = FixtureRows(&c);
    let depth = 200;
    let deep = format!("{}{}", "[".repeat(depth), "]".repeat(depth));
    insert_fixture(
        &c,
        "zz-contract-deep",
        1000,
        &format!(r#"{{"lesson":null,"items":{deep}}}"#),
    );
    // Counted on the raw text: the suite's own serde_json has the same
    // 128 limit and can't parse Laravel's answer.
    let r = c.get("/api/chapters/zz-contract-deep").bearer(&t).send();
    assert_eq!(r.status, 200, "{}", r.text());
    let text = r.text();
    let key = r#""items":"#;
    let at = text.find(key).expect("items key") + key.len();
    let items_depth = text[at..].bytes().take_while(|&b| b == b'[').count();
    assert_eq!(
        items_depth,
        c.expect_by_target("K17", depth, 1),
        "items nesting"
    );
}
