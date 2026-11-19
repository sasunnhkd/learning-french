//! R5 store, R6 update, R7 destroy, R8 reorder. Validation messages are the
//! exact Laravel strings captured in Phase 1. Every test deletes the chapters
//! it creates and never edits seeded rows, so later suites see the seed.

use contract::{Ctx, assert_validation, keys};
use serde_json::{Value, json};

const CHAPTER_KEYS: [&str; 8] = [
    "id", "number", "section", "title", "titleEm", "position", "lesson", "items",
];

fn minimal(id: &str) -> Value {
    json!({
        "id": id,
        "number": 900,
        "section": "Contract",
        "title": "Contract chapter",
        "titleEm": null,
        "lesson": { "intro": null, "formation": [], "irregulars": [], "triggers": [] },
        "items": []
    })
}

struct Cleanup<'a> {
    c: &'a Ctx,
    token: String,
    ids: Vec<String>,
}

impl Drop for Cleanup<'_> {
    fn drop(&mut self) {
        for id in &self.ids {
            let _ = self
                .c
                .delete(&format!("/api/chapters/{id}"))
                .bearer(&self.token)
                .send();
        }
    }
}

fn index(c: &Ctx, t: &str) -> Vec<Value> {
    c.get("/api/chapters")
        .bearer(t)
        .send()
        .expect(200)
        .as_array()
        .unwrap()
        .clone()
}

#[test]
fn store_validation_all_missing() {
    let c = Ctx::from_env();
    let t = c.login();
    let r = c.post("/api/chapters").bearer(&t).json(&json!({})).send();
    assert_validation(
        &r,
        &[
            ("id", &["The id field is required."]),
            ("number", &["The number field is required."]),
            ("section", &["The section field is required."]),
            ("title", &["The title field is required."]),
            ("lesson", &["The lesson field must be present."]),
            (
                "lesson.formation",
                &["The lesson.formation field must be present."],
            ),
            (
                "lesson.irregulars",
                &["The lesson.irregulars field must be present."],
            ),
            (
                "lesson.triggers",
                &["The lesson.triggers field must be present."],
            ),
            ("items", &["The items field must be present."]),
        ],
    );
}

#[test]
fn store_validation_wrong_types() {
    let c = Ctx::from_env();
    let t = c.login();
    let r = c
        .post("/api/chapters")
        .bearer(&t)
        .json(&json!({
            "id": "Bad Id", "number": 0, "section": 5, "title": "", "titleEm": [],
            "position": "x", "lesson": "s", "items": { "a": 1 }
        }))
        .send();
    assert_validation(
        &r,
        &[
            ("id", &["The id field format is invalid."]),
            ("number", &["The number field must be at least 1."]),
            ("section", &["The section field must be a string."]),
            ("title", &["The title field is required."]),
            ("titleEm", &["The title em field must be a string."]),
            ("position", &["The position field must be an integer."]),
            ("lesson", &["The lesson field must be an array."]),
            (
                "lesson.formation",
                &["The lesson.formation field must be present."],
            ),
            (
                "lesson.irregulars",
                &["The lesson.irregulars field must be present."],
            ),
            (
                "lesson.triggers",
                &["The lesson.triggers field must be present."],
            ),
            ("items.a", &["The items.a field must be an array."]),
        ],
    );
}

#[test]
fn store_validation_unique_id() {
    let c = Ctx::from_env();
    let t = c.login();
    let r = c
        .post("/api/chapters")
        .bearer(&t)
        .json(&minimal("subjonctif-present"))
        .send();
    assert_validation(&r, &[("id", &["The id has already been taken."])]);
}

#[test]
fn store_validation_lesson_keys_present() {
    let c = Ctx::from_env();
    let t = c.login();
    let mut p = minimal("contract-lesson-keys");
    p["lesson"] = json!({});
    let r = c.post("/api/chapters").bearer(&t).json(&p).send();
    assert_validation(
        &r,
        &[
            (
                "lesson.formation",
                &["The lesson.formation field must be present."],
            ),
            (
                "lesson.irregulars",
                &["The lesson.irregulars field must be present."],
            ),
            (
                "lesson.triggers",
                &["The lesson.triggers field must be present."],
            ),
        ],
    );
}

#[test]
fn store_validation_max_length_and_item_type() {
    let c = Ctx::from_env();
    let t = c.login();
    let mut p = minimal(&"a".repeat(121));
    p["items"] = json!([{ "type": "bad" }]);
    let r = c.post("/api/chapters").bearer(&t).json(&p).send();
    assert_validation(
        &r,
        &[
            (
                "id",
                &["The id field must not be greater than 120 characters."],
            ),
            ("items.0.type", &["The selected items.0.type is invalid."]),
        ],
    );
}

/// `required` fails a whitespace-only string even without TrimStrings
/// (Laravel checks `trim($value) === ''`), so both targets agree.
#[test]
fn store_validation_whitespace_only_is_missing() {
    let c = Ctx::from_env();
    let t = c.login();
    let mut p = minimal("contract-blank-title");
    p["title"] = json!("   ");
    let r = c.post("/api/chapters").bearer(&t).json(&p).send();
    assert_validation(&r, &[("title", &["The title field is required."])]);
}

/// K1 (D-03) and K2 (D-04), approved fixes Q2 + Q2a. Laravel keeps only the
/// validated keys (and turns `items` into an object keyed "1","0"), trims
/// every string and turns "" into null. Rust stores the payload as sent.
#[test]
fn store_keeps_payload_exactly() {
    let c = Ctx::from_env();
    let t = c.login();
    let mut guard = Cleanup {
        c: &c,
        token: t.clone(),
        ids: vec!["contract-lossless".into()],
    };
    let lesson = json!({
        "intro": "",
        "formation": ["  a  ", ""],
        "irregulars": [{ "v": " être ", "forms": "x", "extra": 1 }],
        "triggers": [{ "title": "t", "subcases": [{ "label": "l", "example": "e", "x": 2 }], "y": 3 }],
        "z": 4
    });
    let items = json!([
        { "before": "Il faut que tu ", "hint": "être", "after": " là.", "answers": ["sois", ""], "rule": " r ", "consolidation": {} },
        { "type": "qcm", "prompt": "p", "options": ["a", "b"], "correctIndex": 1, "consolidation": { "type": "qcm", "prompt": "q" } }
    ]);
    let payload = json!({
        "id": "contract-lossless", "number": "3", "section": "  Sec  ", "title": " T ", "titleEm": "",
        "lesson": lesson, "items": items, "unknown": "u"
    });
    let created = c
        .post("/api/chapters")
        .bearer(&t)
        .json(&payload)
        .send()
        .expect(201);
    let shown = c
        .get("/api/chapters/contract-lossless")
        .bearer(&t)
        .send()
        .expect(200);

    let want = if c.is_laravel() {
        json!({
            "id": "contract-lossless", "number": 3, "section": "Sec", "title": "T", "titleEm": null,
            "lesson": {
                "intro": null,
                "formation": ["a", null],
                "irregulars": [{ "v": "être", "forms": "x" }],
                "triggers": [{ "title": "t", "subcases": [{ "label": "l", "example": "e" }] }]
            },
            "items": {
                "1": { "type": "qcm", "consolidation": { "type": "qcm", "prompt": "q" } },
                "0": { "rule": "r", "consolidation": [] }
            }
        })
    } else {
        json!({
            "id": "contract-lossless", "number": 3, "section": "  Sec  ", "title": " T ", "titleEm": "",
            "lesson": lesson, "items": items
        })
    };
    for (label, body) in [("store", &created), ("show", &shown)] {
        assert_eq!(keys(body), CHAPTER_KEYS, "{label} keys");
        let mut got = body.clone();
        got.as_object_mut().unwrap().remove("position");
        assert_eq!(got, want, "{label} body");
        if !c.is_laravel() {
            assert_eq!(
                keys(&body["lesson"]),
                keys(&want["lesson"]),
                "{label} lesson key order"
            );
        }
    }
    guard.ids.clear();
    assert_eq!(
        c.delete("/api/chapters/contract-lossless")
            .bearer(&t)
            .send()
            .expect(200),
        json!({ "ok": true })
    );
}

/// Q2 round trip: saving each seeded chapter unchanged (as a copy) must give
/// back identical lesson and items in Rust. Laravel damages every chapter
/// that has exercises (K1).
#[test]
fn round_trip_of_every_seed_chapter() {
    let c = Ctx::from_env();
    let t = c.login();
    let mut guard = Cleanup {
        c: &c,
        token: t.clone(),
        ids: Vec::new(),
    };
    let mut damaged = 0;
    for s in c.seed_chapters() {
        let id = format!("rt-{}", s["id"].as_str().unwrap());
        let mut p = s.clone();
        p["id"] = json!(id);
        p.as_object_mut().unwrap().remove("position");
        guard.ids.push(id.clone());
        c.post("/api/chapters")
            .bearer(&t)
            .json(&p)
            .send()
            .expect(201);
        let got = c
            .get(&format!("/api/chapters/{id}"))
            .bearer(&t)
            .send()
            .expect(200);
        let same = got["lesson"] == s["lesson"] && got["items"] == s["items"];
        if c.is_laravel() {
            if s["items"].as_array().is_some_and(|i| !i.is_empty()) && !same {
                damaged += 1;
            }
        } else {
            assert!(same, "{id}: lesson/items changed on save");
        }
    }
    if c.is_laravel() {
        assert!(
            damaged > 0,
            "Laravel is expected to damage chapters on save (K1)"
        );
    }
}

#[test]
fn store_appends_position_and_casts_number() {
    let c = Ctx::from_env();
    let t = c.login();
    let _guard = Cleanup {
        c: &c,
        token: t.clone(),
        ids: vec!["contract-pos".into(), "contract-pos2".into()],
    };
    let max = index(&c, &t)
        .iter()
        .map(|r| r["position"].as_i64().unwrap())
        .max()
        .unwrap_or(-1);
    let mut p = minimal("contract-pos");
    p["number"] = json!("7");
    let b = c
        .post("/api/chapters")
        .bearer(&t)
        .json(&p)
        .send()
        .expect(201);
    assert_eq!(b["position"], max + 1);
    assert_eq!(b["number"], 7);

    let mut p2 = minimal("contract-pos2");
    p2["position"] = json!(500);
    let b2 = c
        .post("/api/chapters")
        .bearer(&t)
        .json(&p2)
        .send()
        .expect(201);
    assert_eq!(b2["position"], 500);
}

#[test]
fn update_rules() {
    let c = Ctx::from_env();
    let t = c.login();
    let _guard = Cleanup {
        c: &c,
        token: t.clone(),
        ids: vec!["contract-upd".into()],
    };
    let mut p = minimal("contract-upd");
    p["position"] = json!(321);
    c.post("/api/chapters")
        .bearer(&t)
        .json(&p)
        .send()
        .expect(201);

    // Missing chapter: 404 before validation.
    let r = c
        .put("/api/chapters/contract-missing")
        .bearer(&t)
        .json(&json!({}))
        .send();
    assert_eq!(
        r.expect(404),
        json!({ "message": "No query results for model [App\\Models\\Chapter] contract-missing" })
    );

    // Same rules as store, minus unique.
    let r = c
        .put("/api/chapters/contract-upd")
        .bearer(&t)
        .json(&json!({}))
        .send();
    assert_eq!(r.status, 422);
    assert_eq!(
        r.json()["message"],
        "The id field is required. (and 8 more errors)"
    );

    // id cannot change.
    let r = c
        .put("/api/chapters/contract-upd")
        .bearer(&t)
        .json(&minimal("contract-other"))
        .send();
    assert_validation(
        &r,
        &[("id", &["The id of an existing chapter cannot be changed."])],
    );

    // No position: keep the stored one.
    let mut u = minimal("contract-upd");
    u["title"] = json!("Updated");
    let b = c
        .put("/api/chapters/contract-upd")
        .bearer(&t)
        .json(&u)
        .send()
        .expect(200);
    assert_eq!(keys(&b), CHAPTER_KEYS);
    assert_eq!(b["title"], "Updated");
    assert_eq!(b["position"], 321);

    // Explicit position wins.
    u["position"] = json!(5);
    let b = c
        .put("/api/chapters/contract-upd")
        .bearer(&t)
        .json(&u)
        .send()
        .expect(200);
    assert_eq!(b["position"], 5);
}

#[test]
fn destroy_then_404() {
    let c = Ctx::from_env();
    let t = c.login();
    c.post("/api/chapters")
        .bearer(&t)
        .json(&minimal("contract-del"))
        .send()
        .expect(201);
    assert_eq!(
        c.delete("/api/chapters/contract-del")
            .bearer(&t)
            .send()
            .expect(200),
        json!({ "ok": true })
    );
    let gone =
        json!({ "message": "No query results for model [App\\Models\\Chapter] contract-del" });
    assert_eq!(
        c.get("/api/chapters/contract-del")
            .bearer(&t)
            .send()
            .expect(404),
        gone
    );
    assert_eq!(
        c.delete("/api/chapters/contract-del")
            .bearer(&t)
            .send()
            .expect(404),
        gone
    );
}

#[test]
fn reorder_validation() {
    let c = Ctx::from_env();
    let t = c.login();
    let r = c
        .post("/api/chapters/reorder")
        .bearer(&t)
        .json(&json!({}))
        .send();
    assert_validation(&r, &[("order", &["The order field is required."])]);
    let r = c
        .post("/api/chapters/reorder")
        .bearer(&t)
        .json(&json!({ "order": "x" }))
        .send();
    assert_validation(&r, &[("order", &["The order field must be an array."])]);
    let r = c
        .post("/api/chapters/reorder")
        .bearer(&t)
        .json(&json!({ "order": [1, "a"] }))
        .send();
    assert_validation(&r, &[("order.0", &["The order.0 field must be a string."])]);
}

/// K8: positions follow the list index; unknown ids are ignored.
#[test]
fn reorder_sets_positions_by_index() {
    let c = Ctx::from_env();
    let t = c.login();
    let ids = ["contract-ro-a", "contract-ro-b", "contract-ro-c"];
    let _guard = Cleanup {
        c: &c,
        token: t.clone(),
        ids: ids.iter().map(|s| (*s).to_string()).collect(),
    };
    for id in ids {
        c.post("/api/chapters")
            .bearer(&t)
            .json(&minimal(id))
            .send()
            .expect(201);
    }
    let body = c
        .post("/api/chapters/reorder")
        .bearer(&t)
        .json(&json!({ "order": ["contract-ro-c", "contract-unknown", "contract-ro-a", "contract-ro-b"] }))
        .send()
        .expect(200);
    assert_eq!(body, json!({ "ok": true }));
    let pos = |id: &str| {
        c.get(&format!("/api/chapters/{id}"))
            .bearer(&t)
            .send()
            .expect(200)["position"]
            .clone()
    };
    assert_eq!(pos("contract-ro-c"), 0);
    assert_eq!(pos("contract-ro-a"), 2);
    assert_eq!(pos("contract-ro-b"), 3);
}
