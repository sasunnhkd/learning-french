//! In-process check of export (slice 3.5). The wire-level contract,
//! including byte equality with PHP's output, is
//! `server/contract/tests/export.rs`.

mod common;

use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode, header};
use serde_json::{Value, json};
use tower::ServiceExt;

async fn call(app: &axum::Router, req: Request<Body>) -> (StatusCode, Value) {
    let res = app.clone().oneshot(req).await.expect("infallible");
    let status = res.status();
    let bytes = to_bytes(res.into_body(), usize::MAX).await.expect("body");
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

fn post(uri: &str, token: Option<&str>, body: &str) -> Request<Body> {
    let mut b = Request::builder()
        .method("POST")
        .uri(uri)
        .header(header::ACCEPT, "application/json")
        .header(header::CONTENT_TYPE, "application/json");
    if let Some(t) = token {
        b = b.header(header::AUTHORIZATION, format!("Bearer {t}"));
    }
    b.body(Body::from(body.to_string())).unwrap()
}

#[tokio::test]
async fn export_writes_pretty_json_or_reports_failure() {
    let root = common::temp_path("export-root").with_extension("d");
    std::fs::create_dir_all(&root).unwrap();
    let root_str = root.display().to_string();
    let (app, db) = common::app_with(&[
        ("EDITOR_PASSWORD", "s3cret"),
        ("LOGIN_RATE_LIMIT", "0"),
        ("REPO_ROOT", &root_str),
    ])
    .await;
    let (_, b) = call(
        &app,
        post("/api/auth/login", None, r#"{"password":"s3cret"}"#),
    )
    .await;
    let t = b["token"].as_str().unwrap().to_string();

    // Guests first.
    assert_eq!(
        call(&app, post("/api/export", None, "")).await.0,
        StatusCode::UNAUTHORIZED
    );

    sqlx::query(
        "insert into chapters (id, number, section, title, title_em, payload, position, created_at, updated_at) \
         values ('a', 1, 'S', 'T', null, '{\"lesson\":{},\"items\":[1.0]}', 0, '2026-01-01 00:00:00', '2026-01-01 00:00:00')",
    )
    .execute(&db.pool)
    .await
    .unwrap();

    // No src/ directory: still 200, written false.
    let (status, b) = call(&app, post("/api/export", Some(&t), "")).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        (&b["written"], &b["path"], &b["count"]),
        (&json!(false), &json!("src/chapters.json"), &json!(1))
    );

    std::fs::create_dir_all(root.join("src")).unwrap();
    let (_, b) = call(&app, post("/api/export", Some(&t), "")).await;
    assert_eq!(b["written"], true);
    let text = std::fs::read_to_string(root.join("src/chapters.json")).unwrap();
    assert_eq!(
        text,
        "[\n    {\n        \"id\": \"a\",\n        \"number\": 1,\n        \"section\": \"S\",\n        \"title\": \"T\",\n        \"titleEm\": null,\n        \"position\": 0,\n        \"lesson\": {},\n        \"items\": [\n            1.0\n        ]\n    }\n]\n"
    );
    let _ = std::fs::remove_dir_all(&root);
}
