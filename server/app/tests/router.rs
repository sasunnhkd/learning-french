//! In-process checks of the Phase 2 foundation: routing, Laravel error
//! bodies, CORS headers, post size, request ids. The HTTP contract suite
//! (`server/contract`) checks the same things over the wire against both
//! apps; these tests keep the details close to the code.

mod common;

use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode, header};
use serde_json::{Value, json};
use tower::ServiceExt;

async fn send(
    app: &axum::Router,
    req: Request<Body>,
) -> (StatusCode, axum::http::HeaderMap, Vec<u8>) {
    let res = app.clone().oneshot(req).await.expect("infallible");
    let status = res.status();
    let headers = res.headers().clone();
    let body = to_bytes(res.into_body(), usize::MAX)
        .await
        .expect("body")
        .to_vec();
    (status, headers, body)
}

fn get(uri: &str) -> axum::http::request::Builder {
    Request::builder()
        .uri(uri)
        .header(header::ACCEPT, "application/json")
}

fn json_of(body: &[u8]) -> Value {
    serde_json::from_slice(body).expect("JSON body")
}

#[tokio::test]
async fn up_is_ok_with_laravel_cache_header() {
    let (app, _db) = common::app().await;
    let (status, headers, body) = send(&app, get("/up").body(Body::empty()).unwrap()).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, b"OK");
    assert_eq!(headers[header::CACHE_CONTROL], "no-cache, private");
    assert!(headers.contains_key("x-request-id"));
    assert!(!headers.contains_key(header::VARY), "no CORS outside api/*");
}

#[tokio::test]
async fn request_id_is_echoed() {
    let (app, _db) = common::app().await;
    let req = get("/up")
        .header("x-request-id", "abc-123")
        .body(Body::empty())
        .unwrap();
    let (_, headers, _) = send(&app, req).await;
    assert_eq!(headers["x-request-id"], "abc-123");
}

#[tokio::test]
async fn unknown_routes_and_dropped_scaffold_routes_are_404() {
    let (app, _db) = common::app().await;
    for (uri, path) in [
        ("/api/nope", "api/nope"),
        ("/", "/"),
        ("/sanctum/csrf-cookie", "sanctum/csrf-cookie"),
    ] {
        let (status, headers, body) = send(&app, get(uri).body(Body::empty()).unwrap()).await;
        assert_eq!(status, StatusCode::NOT_FOUND, "{uri}");
        assert!(
            headers[header::CONTENT_TYPE]
                .to_str()
                .unwrap()
                .starts_with("application/json")
        );
        assert_eq!(
            json_of(&body),
            json!({ "message": format!("The route {path} could not be found.") })
        );
    }
}

#[tokio::test]
async fn guests_get_401_json_even_without_accept() {
    let (app, _db) = common::app().await;
    let unauth = json!({ "message": "Unauthenticated." });
    let routes = [
        ("POST", "/api/auth/logout"),
        ("GET", "/api/chapters"),
        ("POST", "/api/chapters"),
        ("GET", "/api/chapters/x"),
        ("PUT", "/api/chapters/x"),
        ("DELETE", "/api/chapters/x"),
        ("POST", "/api/chapters/reorder"),
        ("GET", "/api/chapters/reorder"),
        ("GET", "/api/uploads"),
        ("POST", "/api/uploads"),
        ("DELETE", "/api/uploads/x"),
        ("POST", "/api/export"),
    ];
    for (method, uri) in routes {
        for accept in [true, false] {
            let mut b = Request::builder().method(method).uri(uri);
            if accept {
                b = b.header(header::ACCEPT, "application/json");
            }
            let (status, _, body) = send(&app, b.body(Body::empty()).unwrap()).await;
            assert_eq!(
                status,
                StatusCode::UNAUTHORIZED,
                "{method} {uri} accept={accept}"
            );
            assert_eq!(json_of(&body), unauth);
        }
        // Phase 2 guard fails closed: any token is rejected until slice 3.1.
        let req = Request::builder()
            .method(method)
            .uri(uri)
            .header(header::AUTHORIZATION, "Bearer 1|whatever")
            .body(Body::empty())
            .unwrap();
        assert_eq!(
            send(&app, req).await.0,
            StatusCode::UNAUTHORIZED,
            "{method} {uri} token"
        );
    }
}

#[tokio::test]
async fn method_not_allowed_matches_laravel() {
    let (app, _db) = common::app().await;
    let cases = [
        ("POST", "/api/chapters/foo", "GET, HEAD, PUT, DELETE"),
        ("PUT", "/api/chapters", "GET, HEAD, POST"),
        ("GET", "/api/export", "POST"),
    ];
    for (method, uri, allowed) in cases {
        let req = Request::builder()
            .method(method)
            .uri(uri)
            .body(Body::empty())
            .unwrap();
        let (status, headers, body) = send(&app, req).await;
        assert_eq!(status, StatusCode::METHOD_NOT_ALLOWED, "{method} {uri}");
        assert_eq!(headers[header::ALLOW], allowed, "{method} {uri}");
        let path = uri.trim_start_matches('/');
        assert_eq!(
            json_of(&body),
            json!({ "message": format!("The {method} method is not supported for route {path}. Supported methods: {allowed}.") })
        );
    }
}

#[tokio::test]
async fn post_size_limit_is_413() {
    let (app, _db) = common::app().await;
    let req = Request::builder()
        .method("POST")
        .uri("/api/uploads")
        .header(header::CONTENT_LENGTH, "9000000")
        .body(Body::empty())
        .unwrap();
    let (status, _, body) = send(&app, req).await;
    assert_eq!(status, StatusCode::PAYLOAD_TOO_LARGE);
    assert_eq!(
        json_of(&body),
        json!({ "message": "The POST data is too large." })
    );
}

#[tokio::test]
async fn cors_actual_requests() {
    let (app, _db) = common::app().await;
    // Allowed origin: echoed, on errors too.
    let req = get("/api/chapters")
        .header(header::ORIGIN, "http://localhost:5173")
        .body(Body::empty())
        .unwrap();
    let (status, h, _) = send(&app, req).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(
        h[header::ACCESS_CONTROL_ALLOW_ORIGIN],
        "http://localhost:5173"
    );
    assert_eq!(h[header::VARY], "Origin");
    // Unknown origin: no allow-origin, still Vary.
    let req = get("/api/chapters")
        .header(header::ORIGIN, "https://evil.example")
        .body(Body::empty())
        .unwrap();
    let (_, h, _) = send(&app, req).await;
    assert!(!h.contains_key(header::ACCESS_CONTROL_ALLOW_ORIGIN));
    assert_eq!(h[header::VARY], "Origin");
    // No Origin at all: Vary only.
    let (_, h, _) = send(&app, get("/api/nope").body(Body::empty()).unwrap()).await;
    assert_eq!(h[header::VARY], "Origin");
    assert!(!h.contains_key(header::ACCESS_CONTROL_ALLOW_ORIGIN));
}

#[tokio::test]
async fn cors_preflight() {
    let (app, _db) = common::app().await;
    let req = Request::builder()
        .method("OPTIONS")
        .uri("/api/chapters/x")
        .header(header::ORIGIN, "http://127.0.0.1:5173")
        .header(header::ACCESS_CONTROL_REQUEST_METHOD, "put")
        .header(
            header::ACCESS_CONTROL_REQUEST_HEADERS,
            "authorization,content-type",
        )
        .body(Body::empty())
        .unwrap();
    let (status, h, body) = send(&app, req).await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    assert_eq!(body, b"");
    assert_eq!(
        h[header::ACCESS_CONTROL_ALLOW_ORIGIN],
        "http://127.0.0.1:5173"
    );
    assert_eq!(h[header::ACCESS_CONTROL_ALLOW_METHODS], "PUT");
    assert_eq!(
        h[header::ACCESS_CONTROL_ALLOW_HEADERS],
        "authorization,content-type"
    );
    assert_eq!(h[header::ACCESS_CONTROL_MAX_AGE], "0");
    assert_eq!(
        h[header::VARY],
        "Origin, Access-Control-Request-Method, Access-Control-Request-Headers"
    );
    assert!(!h.contains_key(header::ACCESS_CONTROL_ALLOW_CREDENTIALS));

    let req = Request::builder()
        .method("OPTIONS")
        .uri("/api/chapters")
        .header(header::ORIGIN, "https://evil.example")
        .header(header::ACCESS_CONTROL_REQUEST_METHOD, "GET")
        .body(Body::empty())
        .unwrap();
    let (status, h, _) = send(&app, req).await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    assert!(!h.contains_key(header::ACCESS_CONTROL_ALLOW_ORIGIN));
    assert!(!h.contains_key(header::ACCESS_CONTROL_ALLOW_METHODS));
    assert_eq!(h[header::VARY], "Origin, Access-Control-Request-Method");
}

#[tokio::test]
async fn extra_frontend_origin_is_allowed() {
    let db = common::fresh_db("origin").await;
    let mut config = common::test_config(&db.path);
    config.allowed_origins.push("https://admin.example".into());
    let app = grammaire_server::router(grammaire_server::AppState::new(config, db.pool.clone()));
    let req = get("/api/chapters")
        .header(header::ORIGIN, "https://admin.example")
        .body(Body::empty())
        .unwrap();
    let (_, h, _) = send(&app, req).await;
    assert_eq!(
        h[header::ACCESS_CONTROL_ALLOW_ORIGIN],
        "https://admin.example"
    );
}

/// `paths.rs`: Laravel's `ValidatePathEncoding` and decoded-path routing.
#[tokio::test]
async fn paths_are_decoded_like_laravel() {
    let (app, _db) = common::app().await;

    for uri in ["/api/chapters/%FF", "/nope/%E9/"] {
        let req = get(uri)
            .header(header::ORIGIN, "http://localhost:5173")
            .body(Body::empty())
            .unwrap();
        let (status, headers, body) = send(&app, req).await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{uri}");
        assert_eq!(json_of(&body), json!({ "message": "Malformed URL." }));
        assert_eq!(headers[header::CACHE_CONTROL], "no-cache, private");
        assert!(!headers.contains_key(header::ACCESS_CONTROL_ALLOW_ORIGIN));
        assert!(!headers.contains_key(header::VARY));
    }

    let (status, _, body) = send(&app, get("/up/").body(Body::empty()).unwrap()).await;
    assert_eq!((status, body.as_slice()), (StatusCode::OK, &b"OK"[..]));

    for (uri, path) in [
        ("/api/nope/", "api/nope"),
        ("/api/chapters/a%2Fb", "api/chapters/a%2Fb"),
        ("/api/chapters/x%2F", "api/chapters/x%2F"),
        ("/api//chapters", "api//chapters"),
    ] {
        let (status, _, body) = send(&app, get(uri).body(Body::empty()).unwrap()).await;
        assert_eq!(status, StatusCode::NOT_FOUND, "{uri}");
        assert_eq!(
            json_of(&body),
            json!({ "message": format!("The route {path} could not be found.") }),
            "{uri}"
        );
    }

    // Routing runs before auth: the 405 quotes the raw path, trimmed.
    let req = get("/api/chapters/foo/")
        .method("POST")
        .body(Body::empty())
        .unwrap();
    let (status, headers, body) = send(&app, req).await;
    assert_eq!(status, StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(headers[header::ALLOW], "GET, HEAD, PUT, DELETE");
    assert_eq!(
        json_of(&body)["message"],
        "The POST method is not supported for route api/chapters/foo. Supported methods: GET, HEAD, PUT, DELETE."
    );

    // The decoded path picks the route: POST /api/chapters/reorder → guard.
    for uri in ["/api/chapters%2Freorder", "/api/auth/logout/"] {
        let req = get(uri).method("POST").body(Body::empty()).unwrap();
        let (status, _, _) = send(&app, req).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED, "{uri}");
    }
}
