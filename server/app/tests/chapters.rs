//! In-process checks of the chapter reads (slice 3.2). The wire-level
//! contract is `server/contract/tests/chapters_read.rs`, run against both
//! apps on the real seed.

mod common;

use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode, header};
use serde_json::{Value, json};
use tower::ServiceExt;

async fn call(app: &axum::Router, req: Request<Body>) -> (StatusCode, Vec<u8>) {
    let res = app.clone().oneshot(req).await.expect("infallible");
    let status = res.status();
    let bytes = to_bytes(res.into_body(), usize::MAX).await.expect("body");
    (status, bytes.to_vec())
}

fn get(uri: &str, token: Option<&str>) -> Request<Body> {
    let mut b = Request::builder()
        .uri(uri)
        .header(header::ACCEPT, "application/json");
    if let Some(t) = token {
        b = b.header(header::AUTHORIZATION, format!("Bearer {t}"));
    }
    b.body(Body::empty()).unwrap()
}

async fn login(app: &axum::Router) -> String {
    let req = Request::builder()
        .method("POST")
        .uri("/api/auth/login")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(r#"{"password":"s3cret"}"#))
        .unwrap();
    let (status, body) = call(app, req).await;
    assert_eq!(status, StatusCode::OK);
    let v: Value = serde_json::from_slice(&body).unwrap();
    v["token"].as_str().unwrap().to_string()
}

async fn insert(pool: &sqlx::SqlitePool, id: &str, number: i64, position: i64, payload: &str) {
    sqlx::query(
        "insert into chapters (id, number, section, title, title_em, payload, position, created_at, updated_at) \
         values (?, ?, 'S', 'T', ?, ?, ?, '2026-01-01 00:00:00', '2026-01-01 00:00:00')",
    )
    .bind(id)
    .bind(number)
    .bind((number == 2).then_some("em"))
    .bind(payload)
    .bind(position)
    .execute(pool)
    .await
    .unwrap();
}

fn json_of(body: &[u8]) -> Value {
    serde_json::from_slice(body).expect("JSON body")
}

#[tokio::test]
async fn index_and_show() {
    let (app, db) =
        common::app_with(&[("EDITOR_PASSWORD", "s3cret"), ("LOGIN_RATE_LIMIT", "0")]).await;
    let t = login(&app).await;

    let (status, body) = call(&app, get("/api/chapters", Some(&t))).await;
    assert_eq!((status, json_of(&body)), (StatusCode::OK, json!([])));

    insert(&db.pool, "c", 3, 1, "{}").await;
    insert(
        &db.pool,
        "b",
        2,
        0,
        r#"{"lesson":{"intro":"x"},"items":[{"a":1.0}]}"#,
    )
    .await;
    insert(&db.pool, "a", 9, 0, "not json").await;
    insert(&db.pool, "a b", 1, 5, "{}").await;

    let (status, body) = call(&app, get("/api/chapters", Some(&t))).await;
    assert_eq!(status, StatusCode::OK);
    let ids: Vec<String> = json_of(&body)
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["id"].as_str().unwrap().to_string())
        .collect();
    assert_eq!(ids, ["b", "a", "c", "a b"], "position, then number");
    assert_eq!(
        json_of(&body)[0],
        json!({ "id": "b", "number": 2, "section": "S", "title": "T", "titleEm": "em", "position": 0 })
    );

    // Payload bytes come back as stored, key order and `1.0` included.
    let (status, body) = call(&app, get("/api/chapters/b", Some(&t))).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        String::from_utf8(body).unwrap(),
        r#"{"id":"b","number":2,"section":"S","title":"T","titleEm":"em","position":0,"lesson":{"intro":"x"},"items":[{"a":1.0}]}"#
    );

    let (_, body) = call(&app, get("/api/chapters/a", Some(&t))).await;
    let v = json_of(&body);
    assert_eq!((&v["lesson"], &v["items"]), (&Value::Null, &json!([])));

    // Route parameters are decoded once (paths.rs).
    let (status, body) = call(&app, get("/api/chapters/a%20b/", Some(&t))).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json_of(&body)["id"], "a b");
}

#[tokio::test]
async fn missing_chapters_and_guests() {
    let (app, db) =
        common::app_with(&[("EDITOR_PASSWORD", "s3cret"), ("LOGIN_RATE_LIMIT", "0")]).await;
    let t = login(&app).await;

    for (uri, id) in [
        ("/api/chapters/nope", "nope"),
        ("/api/chapters/reorder", "reorder"),
        ("/api/chapters/a%2541", "a%41"),
    ] {
        let (status, body) = call(&app, get(uri, Some(&t))).await;
        assert_eq!(status, StatusCode::NOT_FOUND, "{uri}");
        assert_eq!(
            json_of(&body),
            json!({ "message": format!("No query results for model [App\\Models\\Chapter] {id}") })
        );
    }

    // A chapter whose id is "reorder" is what GET /api/chapters/reorder shows.
    insert(&db.pool, "reorder", 1, 0, "{}").await;
    let (status, body) = call(&app, get("/api/chapters/reorder", Some(&t))).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json_of(&body)["id"], "reorder");

    for uri in ["/api/chapters", "/api/chapters/reorder", "/api/chapters/x"] {
        let (status, body) = call(&app, get(uri, None)).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED, "{uri}");
        assert_eq!(json_of(&body), json!({ "message": "Unauthenticated." }));
    }
}

fn send(method: &str, uri: &str, token: &str, body: &Value) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri)
        .header(header::ACCEPT, "application/json")
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::from(body.to_string()))
        .unwrap()
}

fn minimal(id: &str) -> Value {
    json!({
        "id": id, "number": 3, "section": "S", "title": "T",
        "lesson": { "formation": [], "irregulars": [], "triggers": [] },
        "items": []
    })
}

fn not_found(id: &str) -> Value {
    json!({ "message": format!("No query results for model [App\\Models\\Chapter] {id}") })
}

#[tokio::test]
async fn store_update_destroy() {
    let (app, db) =
        common::app_with(&[("EDITOR_PASSWORD", "s3cret"), ("LOGIN_RATE_LIMIT", "0")]).await;
    let t = login(&app).await;

    // Empty table: position 0. Payload kept exactly, blank titleEm kept.
    let mut p = minimal("a");
    p["number"] = json!(" 4 ");
    p["titleEm"] = json!("");
    p["section"] = json!("  S  ");
    p["items"] = json!([{ "answers": ["sois", ""], "x": 1.0, "consolidation": {} }]);
    p["extra"] = json!("dropped");
    let (status, body) = call(&app, send("POST", "/api/chapters", &t, &p)).await;
    assert_eq!(
        status,
        StatusCode::CREATED,
        "{}",
        String::from_utf8_lossy(&body)
    );
    assert_eq!(
        String::from_utf8(body).unwrap(),
        r#"{"id":"a","number":4,"section":"  S  ","title":"T","titleEm":"","position":0,"lesson":{"formation":[],"irregulars":[],"triggers":[]},"items":[{"answers":["sois",""],"x":1.0,"consolidation":{}}]}"#
    );
    let (_, shown) = call(&app, get("/api/chapters/a", Some(&t))).await;
    let stored: String = sqlx::query_scalar("select payload from chapters where id = 'a'")
        .fetch_one(&db.pool)
        .await
        .unwrap();
    assert_eq!(
        stored,
        r#"{"id":"a","number":" 4 ","section":"  S  ","title":"T","titleEm":"","lesson":{"formation":[],"irregulars":[],"triggers":[]},"items":[{"answers":["sois",""],"x":1.0,"consolidation":{}}]}"#,
        "payload keeps the raw number (K10)"
    );
    assert_eq!(json_of(&shown)["section"], "  S  ");

    // Next store appends; a blank position counts as none.
    let mut b = minimal("b");
    b["position"] = json!(" ");
    let (_, body) = call(&app, send("POST", "/api/chapters", &t, &b)).await;
    assert_eq!(json_of(&body)["position"], 1);

    // Duplicate id.
    let (status, body) = call(&app, send("POST", "/api/chapters", &t, &minimal("a"))).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(
        json_of(&body),
        json!({ "message": "The id has already been taken.", "errors": { "id": ["The id has already been taken."] } })
    );

    // Update: 404 first, then validation, then the id check.
    let (status, body) = call(&app, send("PUT", "/api/chapters/zz", &t, &json!({}))).await;
    assert_eq!(
        (status, json_of(&body)),
        (StatusCode::NOT_FOUND, not_found("zz"))
    );
    let (status, body) = call(&app, send("PUT", "/api/chapters/a", &t, &json!({}))).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(
        json_of(&body)["message"],
        "The id field is required. (and 8 more errors)"
    );
    let (_, body) = call(&app, send("PUT", "/api/chapters/a", &t, &minimal("b"))).await;
    assert_eq!(
        json_of(&body)["errors"],
        json!({ "id": ["The id of an existing chapter cannot be changed."] })
    );
    let mut u = minimal("a");
    u["title"] = json!("New");
    let (status, body) = call(&app, send("PUT", "/api/chapters/a", &t, &u)).await;
    assert_eq!(status, StatusCode::OK);
    let v = json_of(&body);
    assert_eq!(
        (&v["title"], &v["position"], &v["titleEm"]),
        (&json!("New"), &json!(0), &Value::Null)
    );
    u["position"] = json!(9);
    let (_, body) = call(&app, send("PUT", "/api/chapters/a", &t, &u)).await;
    assert_eq!(json_of(&body)["position"], 9);

    // Destroy, then 404.
    let del = |id: &str| {
        Request::builder()
            .method("DELETE")
            .uri(format!("/api/chapters/{id}"))
            .header(header::ACCEPT, "application/json")
            .header(header::AUTHORIZATION, format!("Bearer {t}"))
            .body(Body::empty())
            .unwrap()
    };
    let (status, body) = call(&app, del("a")).await;
    assert_eq!(
        (status, json_of(&body)),
        (StatusCode::OK, json!({ "ok": true }))
    );
    let (status, body) = call(&app, del("a")).await;
    assert_eq!(
        (status, json_of(&body)),
        (StatusCode::NOT_FOUND, not_found("a"))
    );
}

#[tokio::test]
async fn reorder_and_the_reorder_path() {
    let (app, db) =
        common::app_with(&[("EDITOR_PASSWORD", "s3cret"), ("LOGIN_RATE_LIMIT", "0")]).await;
    let t = login(&app).await;
    for id in ["a", "b", "c", "d"] {
        let (status, _) = call(&app, send("POST", "/api/chapters", &t, &minimal(id))).await;
        assert_eq!(status, StatusCode::CREATED);
    }

    // List: index = position; unknown ids ignored; the last duplicate wins.
    let order = json!({ "order": ["c", "nope", "a", "b", "b"] });
    let (status, body) = call(&app, send("POST", "/api/chapters/reorder", &t, &order)).await;
    assert_eq!(
        (status, json_of(&body)),
        (StatusCode::OK, json!({ "ok": true }))
    );
    let pos = |id: &'static str| {
        let app = app.clone();
        let t = t.clone();
        async move {
            let (_, body) = call(&app, get(&format!("/api/chapters/{id}"), Some(&t))).await;
            json_of(&body)["position"].clone()
        }
    };
    assert_eq!(
        (
            pos("c").await,
            pos("a").await,
            pos("b").await,
            pos("d").await
        ),
        (json!(0), json!(2), json!(4), json!(3))
    );

    // Object: keys are stored with SQLite integer affinity, as PDO does.
    let order = json!({ "order": { "x": "a", "7": "b", "05": "c", "1.5": "d" } });
    let (status, _) = call(&app, send("POST", "/api/chapters/reorder", &t, &order)).await;
    assert_eq!(status, StatusCode::OK);
    let stored: Vec<(String, String)> =
        sqlx::query_as("select id, typeof(position) || ':' || position from chapters order by id")
            .fetch_all(&db.pool)
            .await
            .unwrap();
    assert_eq!(
        stored,
        [
            ("a".into(), "text:x".into()),
            ("b".into(), "integer:7".into()),
            ("c".into(), "integer:5".into()),
            ("d".into(), "real:1.5".into()),
        ]
    );
    // Reads give PHP's (int): 'x' → 0, 1.5 → 1. Text sorts last.
    assert_eq!(
        (
            pos("a").await,
            pos("b").await,
            pos("c").await,
            pos("d").await
        ),
        (json!(0), json!(7), json!(5), json!(1))
    );
    let (_, body) = call(&app, get("/api/chapters", Some(&t))).await;
    let ids: Vec<String> = json_of(&body)
        .as_array()
        .unwrap()
        .iter()
        .map(|r| format!("{}={}", r["id"].as_str().unwrap(), r["position"]))
        .collect();
    assert_eq!(ids, ["d=1", "c=5", "b=7", "a=0"]);
    // Update without a position keeps the stored one, read as (int)'x'.
    let (status, body) = call(&app, send("PUT", "/api/chapters/a", &t, &minimal("a"))).await;
    assert_eq!(
        (status, json_of(&body)["position"].clone()),
        (StatusCode::OK, json!(0))
    );
    let restore = json!({ "order": { "x": "a" } });
    call(&app, send("POST", "/api/chapters/reorder", &t, &restore)).await;
    // max('position') is the text 'x', so a new chapter gets (int)'x' + 1.
    let (_, body) = call(&app, send("POST", "/api/chapters", &t, &minimal("e"))).await;
    assert_eq!(json_of(&body)["position"], 1);

    // Validation.
    for (body, field, msg) in [
        (json!({}), "order", "The order field is required."),
        (
            json!({ "order": [] }),
            "order",
            "The order field is required.",
        ),
        (
            json!({ "order": " " }),
            "order",
            "The order field is required.",
        ),
        (
            json!({ "order": 0 }),
            "order",
            "The order field must be an array.",
        ),
        (
            json!({ "order": [""] }),
            "order.0",
            "The order.0 field must be a string.",
        ),
        (
            json!({ "order": [[]] }),
            "order.0",
            "The order.0 field must be a string.",
        ),
    ] {
        let (status, got) = call(&app, send("POST", "/api/chapters/reorder", &t, &body)).await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
        assert_eq!(json_of(&got)["errors"], json!({ field: [msg] }), "{body}");
    }

    // PUT and DELETE /api/chapters/reorder are the {id} routes.
    for method in ["PUT", "DELETE"] {
        let (status, body) =
            call(&app, send(method, "/api/chapters/reorder", &t, &json!({}))).await;
        assert_eq!(
            (status, json_of(&body)),
            (StatusCode::NOT_FOUND, not_found("reorder")),
            "{method}"
        );
    }
    let (status, _) = call(&app, send("POST", "/api/chapters", &t, &minimal("reorder"))).await;
    assert_eq!(status, StatusCode::CREATED);
    let mut u = minimal("reorder");
    u["title"] = json!("R");
    let (status, body) = call(&app, send("PUT", "/api/chapters/reorder", &t, &u)).await;
    assert_eq!(
        (status, json_of(&body)["title"].clone()),
        (StatusCode::OK, json!("R"))
    );
    let (status, body) = call(
        &app,
        send("DELETE", "/api/chapters/reorder", &t, &json!({})),
    )
    .await;
    assert_eq!(
        (status, json_of(&body)),
        (StatusCode::OK, json!({ "ok": true }))
    );
}

/// K18: Rust doesn't trim, so values Laravel trimmed into validity fail.
#[tokio::test]
async fn untrimmed_values_are_judged_as_sent() {
    let (app, _db) =
        common::app_with(&[("EDITOR_PASSWORD", "s3cret"), ("LOGIN_RATE_LIMIT", "0")]).await;
    let t = login(&app).await;
    let mut p = minimal(" a ");
    p["items"] = json!([{ "type": " fill " }, { "type": "\u{a0}" }]);
    let (status, body) = call(&app, send("POST", "/api/chapters", &t, &p)).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(
        json_of(&body)["errors"],
        json!({
            "id": ["The id field format is invalid."],
            "items.0.type": ["The selected items.0.type is invalid."],
        })
    );
    let (status, _) = call(&app, send("POST", "/api/chapters", &t, &minimal("a\n"))).await;
    assert_eq!(
        status,
        StatusCode::UNPROCESSABLE_ENTITY,
        "PCRE's $ quirk is not copied"
    );
}

/// Laravel's error order: explicit attributes first, then each wildcard
/// rule; within a `*`-ending rule, scalars before non-empty arrays.
#[tokio::test]
async fn validation_error_order() {
    let (app, _db) =
        common::app_with(&[("EDITOR_PASSWORD", "s3cret"), ("LOGIN_RATE_LIMIT", "0")]).await;
    let t = login(&app).await;
    let mut p = minimal("a");
    p["lesson"]["formation"] = json!([["x"], 5]);
    p["items"] = json!(null);
    let (_, body) = call(&app, send("POST", "/api/chapters", &t, &p)).await;
    let v = json_of(&body);
    let keys: Vec<&String> = v["errors"].as_object().unwrap().keys().collect();
    assert_eq!(keys, ["items", "lesson.formation.1", "lesson.formation.0"]);
    assert_eq!(
        v["message"],
        "The items field must be an array. (and 2 more errors)"
    );
}
