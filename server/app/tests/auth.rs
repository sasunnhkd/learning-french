//! In-process auth checks (slice 3.1). The wire-level contract for the same
//! behaviour is `server/contract/tests/auth.rs`, run against both apps.

mod common;

use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode, header};
use serde_json::{Value, json};
use tower::ServiceExt;

async fn call(
    app: &axum::Router,
    req: Request<Body>,
) -> (StatusCode, axum::http::HeaderMap, Value) {
    let res = app.clone().oneshot(req).await.expect("infallible");
    let status = res.status();
    let headers = res.headers().clone();
    let bytes = to_bytes(res.into_body(), usize::MAX).await.expect("body");
    let body = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    (status, headers, body)
}

fn login_req(body: &str, content_type: &str) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri("/api/auth/login")
        .header(header::ACCEPT, "application/json")
        .header(header::CONTENT_TYPE, content_type)
        .body(Body::from(body.to_string()))
        .unwrap()
}

fn get_uploads(token: &str) -> Request<Body> {
    Request::builder()
        .uri("/api/uploads")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap()
}

#[tokio::test]
async fn login_use_logout() {
    let (app, db) =
        common::app_with(&[("EDITOR_PASSWORD", "s3cret"), ("LOGIN_RATE_LIMIT", "0")]).await;
    let (status, _, body) = call(
        &app,
        login_req(r#"{"password":"s3cret"}"#, "application/json"),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let token = body["token"].as_str().unwrap().to_string();
    let (id, secret) = token.split_once('|').unwrap();

    // Row written like Sanctum's.
    let row: (String, String, String, Option<String>, Option<String>) = sqlx::query_as(
        "select tokenable_type, name, abilities, expires_at, last_used_at from personal_access_tokens where id = ?",
    )
    .bind(id.parse::<i64>().unwrap())
    .fetch_one(&db.pool)
    .await
    .unwrap();
    assert_eq!(
        row,
        (
            "App\\Models\\User".into(),
            "admin-session".into(),
            "[\"*\"]".into(),
            None,
            None
        )
    );
    let user: (String, String, String) =
        sqlx::query_as("select name, email, substr(password, 1, 7) from users")
            .fetch_one(&db.pool)
            .await
            .unwrap();
    assert_eq!(
        user,
        ("Editor".into(), "editor@local".into(), "$2y$04$".into())
    );

    // Both token forms authenticate and mark the token used.
    for t in [token.as_str(), secret] {
        let (status, _, body) = call(&app, get_uploads(t)).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, json!([]));
    }
    let used: Option<String> =
        sqlx::query_scalar("select last_used_at from personal_access_tokens where id = ?")
            .bind(id.parse::<i64>().unwrap())
            .fetch_one(&db.pool)
            .await
            .unwrap();
    assert_eq!(used.map(|u| u.len()), Some(19));

    // A second login reuses the user and makes a new token.
    let (_, _, body2) = call(
        &app,
        login_req("password=s3cret", "application/x-www-form-urlencoded"),
    )
    .await;
    let token2 = body2["token"].as_str().unwrap().to_string();
    assert_ne!(token, token2);
    let users: i64 = sqlx::query_scalar("select count(*) from users")
        .fetch_one(&db.pool)
        .await
        .unwrap();
    assert_eq!(users, 1);

    // Logout revokes only the current token.
    let logout = Request::builder()
        .method("POST")
        .uri("/api/auth/logout")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();
    let (status, _, body) = call(&app, logout).await;
    assert_eq!((status, body), (StatusCode::OK, json!({ "ok": true })));
    assert_eq!(
        call(&app, get_uploads(&token)).await.0,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(call(&app, get_uploads(&token2)).await.0, StatusCode::OK);
}

#[tokio::test]
async fn login_validation_and_k14_query_string() {
    let (app, _db) =
        common::app_with(&[("EDITOR_PASSWORD", "s3cret"), ("LOGIN_RATE_LIMIT", "0")]).await;
    let required = json!({ "message": "The password field is required.", "errors": { "password": ["The password field is required."] } });
    let string = json!({ "message": "The password field must be a string.", "errors": { "password": ["The password field must be a string."] } });
    let invalid =
        json!({ "message": "Invalid password.", "errors": { "password": ["Invalid password."] } });
    let cases = [
        (r#"{}"#, "application/json", &required),
        (r#"{bad"#, "application/json", &required),
        (r#"{"password":"s3cret"}"#, "text/plain", &required),
        (r#"{"password":"   "}"#, "application/json", &required),
        (r#"{"password":["x"]}"#, "application/json", &string),
        (
            "password[]=s3cret",
            "application/x-www-form-urlencoded",
            &string,
        ),
        (r#"{"password":" s3cret "}"#, "application/json", &invalid),
        (r#"{"password":"nope"}"#, "application/json", &invalid),
    ];
    for (body, ct, want) in cases {
        let (status, _, got) = call(&app, login_req(body, ct)).await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body} as {ct}");
        assert_eq!(&got, want, "{body} as {ct}");
    }
    // K14: the query string is not input.
    let req = Request::builder()
        .method("POST")
        .uri("/api/auth/login?password=s3cret")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from("{}"))
        .unwrap();
    let (status, _, got) = call(&app, req).await;
    assert_eq!((status, got), (StatusCode::UNPROCESSABLE_ENTITY, required));
    // vnd+json and charset parameters still count as JSON.
    for ct in [
        "application/vnd.api+json",
        "application/json; charset=utf-8",
    ] {
        assert_eq!(
            call(&app, login_req(r#"{"password":"s3cret"}"#, ct))
                .await
                .0,
            StatusCode::OK,
            "{ct}"
        );
    }
}

#[tokio::test]
async fn unset_password_rejects_every_login() {
    let (app, _db) = common::app_with(&[("LOGIN_RATE_LIMIT", "0")]).await;
    let (status, _, body) = call(
        &app,
        login_req(r#"{"password":"anything"}"#, "application/json"),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(body["errors"]["password"], json!(["Invalid password."]));
}

#[tokio::test]
async fn login_rate_limit() {
    let (app, _db) =
        common::app_with(&[("EDITOR_PASSWORD", "s3cret"), ("LOGIN_RATE_LIMIT", "2")]).await;
    for _ in 0..2 {
        assert_eq!(
            call(
                &app,
                login_req(r#"{"password":"nope"}"#, "application/json")
            )
            .await
            .0,
            StatusCode::UNPROCESSABLE_ENTITY
        );
    }
    let (status, headers, body) = call(
        &app,
        login_req(r#"{"password":"s3cret"}"#, "application/json"),
    )
    .await;
    assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
    assert_eq!(body, json!({ "message": "Too Many Attempts." }));
    let retry: u64 = headers[header::RETRY_AFTER]
        .to_str()
        .unwrap()
        .parse()
        .unwrap();
    assert!((1..=60).contains(&retry));
}

#[tokio::test]
async fn expired_deleted_and_malformed_tokens() {
    let (app, db) =
        common::app_with(&[("EDITOR_PASSWORD", "s3cret"), ("LOGIN_RATE_LIMIT", "0")]).await;
    let (_, _, body) = call(
        &app,
        login_req(r#"{"password":"s3cret"}"#, "application/json"),
    )
    .await;
    let token = body["token"].as_str().unwrap().to_string();
    let id: i64 = token.split_once('|').unwrap().0.parse().unwrap();

    for bad in [
        "x|abc",
        " 1|abc",
        &format!("{id}|wrong"),
        &format!("{token} "),
    ] {
        assert_eq!(
            call(&app, get_uploads(bad)).await.0,
            StatusCode::UNAUTHORIZED,
            "{bad:?}"
        );
    }
    sqlx::query(
        "update personal_access_tokens set expires_at = '2000-01-01 00:00:00' where id = ?",
    )
    .bind(id)
    .execute(&db.pool)
    .await
    .unwrap();
    assert_eq!(
        call(&app, get_uploads(&token)).await.0,
        StatusCode::UNAUTHORIZED
    );
    sqlx::query(
        "update personal_access_tokens set expires_at = '2999-01-01 00:00:00' where id = ?",
    )
    .bind(id)
    .execute(&db.pool)
    .await
    .unwrap();
    assert_eq!(call(&app, get_uploads(&token)).await.0, StatusCode::OK);
    sqlx::query(
        "update personal_access_tokens set tokenable_type = 'App\\Models\\Other' where id = ?",
    )
    .bind(id)
    .execute(&db.pool)
    .await
    .unwrap();
    assert_eq!(
        call(&app, get_uploads(&token)).await.0,
        StatusCode::UNAUTHORIZED,
        "wrong tokenable type"
    );
    sqlx::query(
        "update personal_access_tokens set tokenable_type = 'App\\Models\\User' where id = ?",
    )
    .bind(id)
    .execute(&db.pool)
    .await
    .unwrap();
    sqlx::query("delete from users")
        .execute(&db.pool)
        .await
        .unwrap();
    assert_eq!(
        call(&app, get_uploads(&token)).await.0,
        StatusCode::UNAUTHORIZED,
        "deleted user"
    );
}
