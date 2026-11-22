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
