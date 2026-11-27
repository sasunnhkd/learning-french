//! In-process checks of upload store and delete (slice 3.4). The wire-level
//! contract is `server/contract/tests/uploads.rs`, run against both apps.

mod common;

use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode, header};
use serde_json::{Value, json};
use tower::ServiceExt;

const PNG: &[u8] = b"\x89PNG\r\n\x1a\n\x00\x00\x00\x0dIHDR\x00\x00\x00\x01";

async fn call(app: &axum::Router, req: Request<Body>) -> (StatusCode, Value) {
    let res = app.clone().oneshot(req).await.expect("infallible");
    let status = res.status();
    let bytes = to_bytes(res.into_body(), usize::MAX).await.expect("body");
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

async fn login(app: &axum::Router) -> String {
    let req = Request::builder()
        .method("POST")
        .uri("/api/auth/login")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(r#"{"password":"s3cret"}"#))
        .unwrap();
    call(app, req).await.1["token"]
        .as_str()
        .unwrap()
        .to_string()
}

/// One multipart part: (Content-Disposition params, bytes).
fn multipart(token: &str, parts: &[(&str, &[u8])]) -> Request<Body> {
    let mut body = Vec::new();
    for (disp, data) in parts {
        body.extend_from_slice(b"--XYZ\r\nContent-Disposition: form-data; ");
        body.extend_from_slice(disp.as_bytes());
        body.extend_from_slice(b"\r\n\r\n");
        body.extend_from_slice(data);
        body.extend_from_slice(b"\r\n");
    }
    body.extend_from_slice(b"--XYZ--\r\n");
    Request::builder()
        .method("POST")
        .uri("/api/uploads")
        .header(header::ACCEPT, "application/json")
        .header(header::CONTENT_TYPE, "multipart/form-data; boundary=XYZ")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::from(body))
        .unwrap()
}

fn delete(token: &str, id: &str) -> Request<Body> {
    Request::builder()
        .method("DELETE")
        .uri(format!("/api/uploads/{id}"))
        .header(header::ACCEPT, "application/json")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap()
}

/// A repo root of its own, removed on drop.
struct Root(std::path::PathBuf);

impl Drop for Root {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[tokio::test]
async fn store_list_delete() {
    let root = Root(common::temp_path("uploads-root").with_extension("d"));
    std::fs::create_dir_all(&root.0).unwrap();
    let root_str = root.0.display().to_string();
    let (app, db) = common::app_with(&[
        ("EDITOR_PASSWORD", "s3cret"),
        ("LOGIN_RATE_LIMIT", "0"),
        ("REPO_ROOT", &root_str),
    ])
    .await;
    let t = login(&app).await;

    // public/uploads is created on first use.
    let (status, b) = call(
        &app,
        multipart(&t, &[(r#"name="file"; filename="dir\a b.PNG""#, PNG)]),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{b}");
    let id = b["id"].as_str().unwrap().to_string();
    let path = b["path"].as_str().unwrap().to_string();
    assert_eq!(
        path,
        format!("uploads/{}.png", id.to_uppercase()),
        "K3: id = file ULID"
    );
    assert_eq!(
        b,
        json!({
            "id": id, "path": path,
            "url": format!("http://localhost:5173/learning-french/{path}"),
            "mime": "image/png", "size": PNG.len()
        })
    );
    let on_disk = root.0.join("public").join(&path);
    assert_eq!(std::fs::read(&on_disk).unwrap(), PNG);
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = std::fs::metadata(root.0.join("public/uploads"))
            .unwrap()
            .permissions()
            .mode();
        // 0755 minus the umask, as PHP's mkdir: never group/other-writable.
        assert_eq!(mode & 0o022, 0, "{mode:o}");
    }
    let (name, created): (String, String) =
        sqlx::query_as("select original_name, created_at from uploads where id = ?")
            .bind(&id)
            .fetch_one(&db.pool)
            .await
            .unwrap();
    assert_eq!(name, "a b.PNG");
    assert_eq!(created.len(), 19, "Y-m-d H:i:s: {created}");

    // Rejected uploads write nothing.
    let svg = br#"<svg xmlns="http://www.w3.org/2000/svg"/>"#;
    let (status, b) = call(
        &app,
        multipart(&t, &[(r#"name="file"; filename="x.svg""#, svg)]),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(
        b["errors"]["file"],
        json!([
            "The file field must be a file of type: image/png, image/jpeg, image/webp, image/gif."
        ])
    );
    let (status, _) = call(
        &app,
        multipart(&t, &[(r#"name="file"; filename="x.php""#, PNG)]),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(
        std::fs::read_dir(root.0.join("public/uploads"))
            .unwrap()
            .count(),
        1
    );

    // Delete removes the file and the row; again is 404.
    let (status, b) = call(&app, delete(&t, &id)).await;
    assert_eq!((status, b), (StatusCode::OK, json!({ "ok": true })));
    assert!(!on_disk.exists());
    let (status, b) = call(&app, delete(&t, &id)).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(
        b,
        json!({ "message": format!("No query results for model [App\\Models\\Upload] {id}") })
    );

    // A Laravel-made row (K3: id unrelated to the file name) still deletes
    // its file through `path`; a missing file is not an error.
    std::fs::write(root.0.join("public/uploads/01ABC.png"), PNG).unwrap();
    sqlx::query(
        "insert into uploads (id, path, original_name, mime, size, created_at, updated_at) \
         values ('01zzz', 'uploads/01ABC.png', 'o.png', 'image/png', 16, '2026-01-01 00:00:00', '2026-01-01 00:00:00'), \
                ('01yyy', 'uploads/GONE.png', 'g.png', 'image/png', 16, '2026-01-01 00:00:00', '2026-01-01 00:00:00')",
    )
    .execute(&db.pool)
    .await
    .unwrap();
    assert_eq!(call(&app, delete(&t, "01zzz")).await.0, StatusCode::OK);
    assert!(!root.0.join("public/uploads/01ABC.png").exists());
    assert_eq!(call(&app, delete(&t, "01yyy")).await.0, StatusCode::OK);
}
