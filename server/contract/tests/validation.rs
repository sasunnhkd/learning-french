//! Edge cases of the chapter payload rules (INVENTORY §1.3), captured from
//! Laravel in Phase 1. The Rust validator must reproduce these exactly:
//!
//! - errors come in **rule declaration order**, except that every rule
//!   without `*` comes before all wildcard expansions; each wildcard rule is
//!   expanded over the data before the next one, and for a rule ending in `*`
//!   scalar children come before non-empty arrays and objects;
//! - `integer` is PHP `filter_var(FILTER_VALIDATE_INT)`: it accepts `true`,
//!   `1.0` and `" 4 "` (surrounding whitespace), and rejects `"03"`, `"3.0"`,
//!   `"1e3"` and `3.5`;
//! - `max` on strings counts characters (`mb_strlen`), not bytes;
//! - `required` treats `""`, whitespace-only strings, `null` and `[]` as missing;
//! - `array` accepts JSON arrays and objects;
//! - a blank string (Unicode spaces and invisible characters too) counts as
//!   `null` for every rule: Laravel's middleware nulls it before validation.
//!
//! Rust: `server/app/src/validation.rs` (engine), `php.rs` (PHP number and
//! string rules).

use contract::{Ctx, assert_validation};
use serde_json::{Value, json};

fn minimal(id: &str) -> Value {
    json!({
        "id": id, "number": 1, "section": "s", "title": "t",
        "lesson": { "formation": [], "irregulars": [], "triggers": [] },
        "items": []
    })
}

fn with(id: &str, field: &str, value: Value) -> Value {
    let mut p = minimal(id);
    p[field] = value;
    p
}

/// POST and, if it was created, delete it again.
fn store(c: &Ctx, t: &str, p: &Value) -> contract::Resp {
    let r = c.post("/api/chapters").bearer(t).json(p).send();
    if r.status == 201 {
        let id = p["id"].as_str().unwrap();
        c.delete(&format!("/api/chapters/{id}"))
            .bearer(t)
            .send()
            .expect(200);
    }
    r
}

#[test]
fn max_lengths() {
    let c = Ctx::from_env();
    let t = c.login();
    let mut p = minimal("contract-max");
    p["section"] = json!("s".repeat(121));
    p["title"] = json!("t".repeat(201));
    p["titleEm"] = json!("e".repeat(201));
    assert_validation(
        &store(&c, &t, &p),
        &[
            (
                "section",
                &["The section field must not be greater than 120 characters."],
            ),
            (
                "title",
                &["The title field must not be greater than 200 characters."],
            ),
            (
                "titleEm",
                &["The title em field must not be greater than 200 characters."],
            ),
        ],
    );
}

#[test]
fn max_counts_characters_not_bytes() {
    let c = Ctx::from_env();
    let t = c.login();
    let ok = store(
        &c,
        &t,
        &with("contract-mb", "section", json!("é".repeat(120))),
    );
    assert_eq!(ok.expect(201)["section"], "é".repeat(120));
    let too_long = store(
        &c,
        &t,
        &with("contract-mb", "section", json!("é".repeat(121))),
    );
    assert_validation(
        &too_long,
        &[(
            "section",
            &["The section field must not be greater than 120 characters."],
        )],
    );
}

#[test]
fn nested_rules_in_declaration_order() {
    let c = Ctx::from_env();
    let t = c.login();
    let p = json!({
        "id": "contract-nested", "number": 1, "section": "s", "title": "t",
        "lesson": {
            "intro": 1,
            "formation": [1],
            "irregulars": ["x", { "v": 1, "forms": [] }],
            "triggers": ["x", { "title": 1, "body": 2, "note": 3, "subcases": "x" }, { "subcases": ["y", { "label": 1, "example": 2 }] }]
        },
        "items": ["z", { "rule": 1, "consolidation": "c", "type": 1 }]
    });
    assert_validation(
        &store(&c, &t, &p),
        &[
            (
                "lesson.intro",
                &["The lesson.intro field must be a string."],
            ),
            (
                "lesson.formation.0",
                &["The lesson.formation.0 field must be a string."],
            ),
            (
                "lesson.irregulars.0",
                &["The lesson.irregulars.0 field must be an array."],
            ),
            (
                "lesson.irregulars.1.v",
                &["The lesson.irregulars.1.v field must be a string."],
            ),
            (
                "lesson.irregulars.1.forms",
                &["The lesson.irregulars.1.forms field must be a string."],
            ),
            (
                "lesson.triggers.0",
                &["The lesson.triggers.0 field must be an array."],
            ),
            (
                "lesson.triggers.1.title",
                &["The lesson.triggers.1.title field must be a string."],
            ),
            (
                "lesson.triggers.1.body",
                &["The lesson.triggers.1.body field must be a string."],
            ),
            (
                "lesson.triggers.1.note",
                &["The lesson.triggers.1.note field must be a string."],
            ),
            (
                "lesson.triggers.1.subcases",
                &["The lesson.triggers.1.subcases field must be an array."],
            ),
            (
                "lesson.triggers.2.subcases.0",
                &["The lesson.triggers.2.subcases.0 field must be an array."],
            ),
            (
                "lesson.triggers.2.subcases.1.label",
                &["The lesson.triggers.2.subcases.1.label field must be a string."],
            ),
            (
                "lesson.triggers.2.subcases.1.example",
                &["The lesson.triggers.2.subcases.1.example field must be a string."],
            ),
            ("items.0", &["The items.0 field must be an array."]),
            ("items.1.type", &["The selected items.1.type is invalid."]),
            (
                "items.1.rule",
                &["The items.1.rule field must be a string."],
            ),
            (
                "items.1.consolidation",
                &["The items.1.consolidation field must be an array."],
            ),
        ],
    );
}

#[test]
fn nullable_fields_accept_null() {
    let c = Ctx::from_env();
    let t = c.login();
    let p = json!({
        "id": "contract-nulls", "number": 1, "section": "s", "title": "t", "titleEm": null, "position": null,
        "lesson": { "intro": null, "formation": [null], "irregulars": [{ "v": null, "forms": null }],
                    "triggers": [{ "title": null, "body": null, "note": null, "subcases": null }] },
        "items": [{ "type": null, "rule": null, "consolidation": null }]
    });
    let b = store(&c, &t, &p).expect(201);
    assert_eq!(
        b["lesson"]["triggers"],
        json!([{ "title": null, "body": null, "note": null, "subcases": null }])
    );
    assert_eq!(
        b["items"],
        json!([{ "type": null, "rule": null, "consolidation": null }])
    );
}

const NUMBER_INT: &str = "The number field must be an integer.";

/// PHP `filter_var(FILTER_VALIDATE_INT)` semantics.
#[test]
fn number_integer_forms() {
    let c = Ctx::from_env();
    let t = c.login();
    let cases: &[(Value, Result<i64, &str>)] = &[
        (json!(3), Ok(3)),
        (json!("3"), Ok(3)),
        (json!(" 4 "), Ok(4)),
        (json!(1.0), Ok(1)),
        (json!(true), Ok(1)),
        (json!("03"), Err(NUMBER_INT)),
        (json!("3.0"), Err(NUMBER_INT)),
        (json!("1e3"), Err(NUMBER_INT)),
        (json!(3.5), Err(NUMBER_INT)),
        (json!("abc"), Err(NUMBER_INT)),
        (json!(-1), Err("The number field must be at least 1.")),
        (json!(0), Err("The number field must be at least 1.")),
        (json!(null), Err("The number field is required.")),
    ];
    for (input, want) in cases {
        let r = store(&c, &t, &with("contract-num", "number", input.clone()));
        match want {
            Ok(n) => assert_eq!(r.expect(201)["number"], *n, "number={input}"),
            Err(msg) => assert_validation(&r, &[("number", &[msg])]),
        }
    }
}

#[test]
fn position_integer_forms() {
    let c = Ctx::from_env();
    let t = c.login();
    let bad = "The position field must be an integer.";
    let cases: &[(Value, Result<i64, &str>)] = &[
        (json!("5"), Ok(5)),
        (json!(-3), Ok(-3)),
        (json!(true), Ok(1)),
        (json!(5.5), Err(bad)),
        (json!("x"), Err(bad)),
    ];
    for (input, want) in cases {
        let r = store(
            &c,
            &t,
            &with("contract-pos-forms", "position", input.clone()),
        );
        match want {
            Ok(n) => assert_eq!(r.expect(201)["position"], *n, "position={input}"),
            Err(msg) => assert_validation(&r, &[("position", &[msg])]),
        }
    }
}

#[test]
fn required_and_array_edge_cases() {
    let c = Ctx::from_env();
    let t = c.login();
    let id_required: &[&str] = &["The id field is required."];
    assert_validation(
        &store(&c, &t, &with("x", "id", json!(""))),
        &[("id", id_required)],
    );
    assert_validation(
        &store(&c, &t, &with("x", "id", json!([]))),
        &[("id", id_required)],
    );
    assert_validation(
        &store(&c, &t, &with("x", "id", json!(123))),
        &[("id", &["The id field must be a string."])],
    );

    let missing_lesson_keys: &[(&str, &[&str])] = &[
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
    ];
    // An empty JSON array passes `array` but has none of the keys.
    assert_validation(
        &store(&c, &t, &with("contract-l", "lesson", json!([]))),
        missing_lesson_keys,
    );
    let mut null_lesson = vec![("lesson", &["The lesson field must be an array."][..])];
    null_lesson.extend_from_slice(missing_lesson_keys);
    assert_validation(
        &store(&c, &t, &with("contract-l", "lesson", json!(null))),
        &null_lesson,
    );
    assert_validation(
        &store(
            &c,
            &t,
            &with(
                "contract-l",
                "lesson",
                json!({ "formation": null, "irregulars": [], "triggers": [] }),
            ),
        ),
        &[(
            "lesson.formation",
            &["The lesson.formation field must be an array."],
        )],
    );
    assert_validation(
        &store(&c, &t, &with("contract-l", "items", json!(null))),
        &[("items", &["The items field must be an array."])],
    );

    let order_required: &[&str] = &["The order field is required."];
    for order in [json!([]), json!(null)] {
        let r = c
            .post("/api/chapters/reorder")
            .bearer(&t)
            .json(&json!({ "order": order }))
            .send();
        assert_validation(&r, &[("order", order_required)]);
    }
}

/// Probe captures from slice 3.3: explicit rules before wildcard ones;
/// within a `*`-ending rule, scalars before non-empty arrays; within a rule
/// ending in a field (`items.*.rule`), plain data order.
#[test]
fn error_order_explicit_then_wildcards() {
    let c = Ctx::from_env();
    let t = c.login();
    let mut p = minimal("contract-order");
    p["lesson"]["formation"] = json!([["a"], 5]);
    p["items"] = json!(null);
    assert_validation(
        &store(&c, &t, &p),
        &[
            ("items", &["The items field must be an array."]),
            (
                "lesson.formation.1",
                &["The lesson.formation.1 field must be a string."],
            ),
            (
                "lesson.formation.0",
                &["The lesson.formation.0 field must be a string."],
            ),
        ],
    );
    let p = with(
        "contract-order",
        "items",
        json!([{ "rule": 1 }, "z", { "rule": ["x"] }]),
    );
    assert_validation(
        &store(&c, &t, &p),
        &[
            ("items.1", &["The items.1 field must be an array."]),
            (
                "items.0.rule",
                &["The items.0.rule field must be a string."],
            ),
            (
                "items.2.rule",
                &["The items.2.rule field must be a string."],
            ),
        ],
    );
    let mut p = minimal("contract-order");
    p["lesson"]["triggers"] =
        json!([{ "subcases": [{ "label": [1] }, "y", { "label": 2 }] }, "t", { "subcases": "s" }]);
    assert_validation(
        &store(&c, &t, &p),
        &[
            (
                "lesson.triggers.1",
                &["The lesson.triggers.1 field must be an array."],
            ),
            (
                "lesson.triggers.2.subcases",
                &["The lesson.triggers.2.subcases field must be an array."],
            ),
            (
                "lesson.triggers.0.subcases.1",
                &["The lesson.triggers.0.subcases.1 field must be an array."],
            ),
            (
                "lesson.triggers.0.subcases.0.label",
                &["The lesson.triggers.0.subcases.0.label field must be a string."],
            ),
            (
                "lesson.triggers.0.subcases.2.label",
                &["The lesson.triggers.0.subcases.2.label field must be a string."],
            ),
        ],
    );
    // Object keys are kept as they are, dots and stars included. In the
    // message a plain wildcard key is shown raw (`aB`); a key with a dot or
    // star goes through `Str::snake` (`x.Y` → `x. y`).
    let mut p = minimal("contract-order");
    p["lesson"]["formation"] = json!({ "a.b": 1, "c*": 2, "aB": 3, "x.Y": 4 });
    p["items"] = json!({ "1": 5, "": { "rule": 1 } });
    assert_validation(
        &store(&c, &t, &p),
        &[
            (
                "lesson.formation.a.b",
                &["The lesson.formation.a.b field must be a string."],
            ),
            (
                "lesson.formation.c*",
                &["The lesson.formation.c* field must be a string."],
            ),
            (
                "lesson.formation.aB",
                &["The lesson.formation.aB field must be a string."],
            ),
            (
                "lesson.formation.x.Y",
                &["The lesson.formation.x. y field must be a string."],
            ),
            ("items.1", &["The items.1 field must be an array."]),
            ("items..rule", &["The items..rule field must be a string."]),
        ],
    );
}

const ID_STRING: &str = "The id field must be a string.";
const ID_FORMAT: &str = "The id field format is invalid.";

/// Non-string values still go through every rule: `regex` sees PHP's
/// string form (`1e13` → "10000000000000" matches, `1e14` → "1.0E+14"
/// doesn't), `max` counts array items, `min` compares numbers exactly.
#[test]
fn rules_on_non_string_values() {
    let c = Ctx::from_env();
    let t = c.login();
    let cases: &[(Value, &[&str])] = &[
        (json!(-5), &[ID_STRING]),
        (json!(1e13), &[ID_STRING]),
        (json!(1e14), &[ID_STRING, ID_FORMAT]),
        (json!(1.5), &[ID_STRING, ID_FORMAT]),
        (json!(true), &[ID_STRING, ID_FORMAT]),
        (json!(false), &[ID_STRING, ID_FORMAT]),
        (json!(["a"]), &[ID_STRING, ID_FORMAT]),
        (
            json!(vec!["a"; 121]),
            &[
                ID_STRING,
                "The id field must not be greater than 120 characters.",
                ID_FORMAT,
            ],
        ),
    ];
    for (id, msgs) in cases {
        assert_validation(
            &store(&c, &t, &with("x", "id", id.clone())),
            &[("id", msgs)],
        );
    }
    let int = NUMBER_INT;
    let at_least = "The number field must be at least 1.";
    let cases: &[(Value, &[&str])] = &[
        (json!(false), &[int, at_least]),
        (json!(0.5), &[int, at_least]),
        (json!(1e-7), &[int, at_least]),
        (json!("1."), &[int]),
        (json!([1]), &[int]),
        (json!("9223372036854775808"), &[int]),
        (json!(1e20), &[int]),
        (json!(" 1e3 "), &[int]),
        (json!("-0"), &[at_least]),
    ];
    for (n, msgs) in cases {
        assert_validation(
            &store(&c, &t, &with("contract-n", "number", n.clone())),
            &[("number", msgs)],
        );
    }
    for n in [json!("+5"), json!("\u{b}5\n"), json!(0.999_999_999_999_999)] {
        let r = store(&c, &t, &with("contract-n", "number", n.clone()));
        // `(int) 0.999999999999999` is 0, though its string form "1" passes.
        let want = if n.is_f64() { 0 } else { 5 };
        assert_eq!(r.expect(201)["number"], want, "number={n}");
    }
    assert_validation(
        &store(&c, &t, &with("x", "section", json!(vec!["a"; 121]))),
        &[(
            "section",
            &[
                "The section field must be a string.",
                "The section field must not be greater than 120 characters.",
            ],
        )],
    );
    for ty in [json!(["fill"]), json!(true), json!(1)] {
        let p = with("x", "items", json!([{ "type": ty }]));
        assert_validation(
            &store(&c, &t, &p),
            &[("items.0.type", &["The selected items.0.type is invalid."])],
        );
    }
}

/// Laravel's middleware turns blank strings (spaces, but also no-break
/// spaces, soft hyphens and other invisible characters) into `null` before
/// validation. Rust validates the same way; what it stores is K2.
#[test]
fn blank_strings_count_as_null() {
    let c = Ctx::from_env();
    let t = c.login();
    for title in ["\u{a0}", "\u{ad}", "\u{200b}\u{3000}"] {
        assert_validation(
            &store(&c, &t, &with("x", "title", json!(title))),
            &[("title", &["The title field is required."])],
        );
    }
    // A blank position is no position: the chapter is appended.
    let max = c
        .get("/api/chapters")
        .bearer(&t)
        .send()
        .expect(200)
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["position"].as_i64().unwrap())
        .max()
        .unwrap();
    for pos in ["", "  ", "\u{a0}"] {
        let r = store(&c, &t, &with("contract-blank-pos", "position", json!(pos)));
        assert_eq!(r.expect(201)["position"], max + 1, "position={pos:?}");
    }
    // A blank type is null, which `nullable` accepts.
    let r = store(
        &c,
        &t,
        &with(
            "contract-blank-type",
            "items",
            json!([{ "type": "\u{a0}" }]),
        ),
    );
    assert_eq!(r.status, 201, "{}", r.text());
    // A blank order entry is null, which `string` rejects.
    for id in ["", " "] {
        let r = c
            .post("/api/chapters/reorder")
            .bearer(&t)
            .json(&json!({ "order": [id] }))
            .send();
        assert_validation(&r, &[("order.0", &["The order.0 field must be a string."])]);
    }
    let r = c
        .post("/api/chapters/reorder")
        .bearer(&t)
        .json(&json!({ "order": " " }))
        .send();
    assert_validation(&r, &[("order", &["The order field is required."])]);
}
