//! Framework-level behaviour: health, unknown routes, wrong methods, CORS,
//! and the scaffold routes Laravel exposes (INVENTORY §1.2, plan Q6).

use contract::Ctx;
use serde_json::json;

#[test]
fn up_returns_200() {
    let c = Ctx::from_env();
    let r = c.get("/up").without_header("Accept").send();
    assert_eq!(r.status, 200, "{}", r.text());
}

#[test]
fn unknown_api_route_is_404_json() {
    let c = Ctx::from_env();
    let body = c.get("/api/nope").send().expect(404);
    assert_eq!(
        body,
        json!({ "message": "The route api/nope could not be found." })
    );
}

/// Routing runs before the auth guard, so a guest gets the 405 too.
#[test]
fn wrong_method_is_405_with_allow_header() {
    let c = Ctx::from_env();
    let r = c.post("/api/chapters/some-id").send();
    let body = r.expect(405);
    assert_eq!(
        body,
        json!({ "message": "The POST method is not supported for route api/chapters/some-id. Supported methods: GET, HEAD, PUT, DELETE." })
    );
    assert_eq!(r.header("allow"), Some("GET, HEAD, PUT, DELETE"));
}

#[test]
fn cors_preflight_allowed_origins_echo_request() {
    let c = Ctx::from_env();
    for origin in [
        "http://localhost:5173",
        "http://127.0.0.1:5173",
        "http://localhost:4173",
    ] {
        let r = c
            .req("OPTIONS", "/api/chapters/x")
            .without_header("Accept")
            .header("Origin", origin)
            .header("Access-Control-Request-Method", "PUT")
            .header(
                "Access-Control-Request-Headers",
                "authorization,content-type",
            )
            .send();
        assert_eq!(r.status, 204, "preflight from {origin}");
        assert_eq!(r.header("access-control-allow-origin"), Some(origin));
        assert_eq!(r.header("access-control-allow-methods"), Some("PUT"));
        assert_eq!(
            r.header("access-control-allow-headers"),
            Some("authorization,content-type")
        );
        assert_eq!(r.header("access-control-max-age"), Some("0"));
        assert_eq!(
            r.header("access-control-allow-credentials"),
            None,
            "supports_credentials is false"
        );
    }
}

#[test]
fn cors_preflight_unknown_origin_gets_no_allow_origin() {
    let c = Ctx::from_env();
    let r = c
        .req("OPTIONS", "/api/chapters")
        .without_header("Accept")
        .header("Origin", "https://evil.example")
        .header("Access-Control-Request-Method", "GET")
        .send();
    assert_eq!(r.status, 204);
    assert_eq!(r.header("access-control-allow-origin"), None);
}

#[test]
fn cors_simple_request_echoes_allowed_origin() {
    let c = Ctx::from_env();
    let t = c.login();
    let r = c
        .get("/api/chapters")
        .bearer(&t)
        .header("Origin", "http://localhost:5173")
        .send();
    assert_eq!(r.status, 200);
    assert_eq!(
        r.header("access-control-allow-origin"),
        Some("http://localhost:5173")
    );
    assert!(
        r.header("vary").is_some_and(|v| v.contains("Origin")),
        "Vary: Origin"
    );
}

/// K12 / plan Q6 (default: drop). Laravel serves its scaffold welcome page on
/// `/` and Sanctum's CSRF cookie route; the SPA uses neither.
#[test]
fn scaffold_routes() {
    let c = Ctx::from_env();
    let root = c.get("/").without_header("Accept").send();
    assert_eq!(root.status, c.expect_by_target("K12", 200, 404));
    let csrf = c
        .get("/sanctum/csrf-cookie")
        .without_header("Accept")
        .send();
    assert_eq!(csrf.status, c.expect_by_target("K12", 204, 404));
}

/// K11: without `Accept: application/json`, Laravel tries to redirect a guest
/// to a `login` route that does not exist and answers 500 HTML. The SPA always
/// sends the header. Rust default: always 401 JSON.
#[test]
fn guest_without_accept_header() {
    let c = Ctx::from_env();
    let r = c.get("/api/chapters").without_header("Accept").send();
    if c.is_laravel() {
        assert_eq!(r.status, 500);
    } else {
        assert_eq!(r.expect(401), json!({ "message": "Unauthenticated." }));
    }
}

/// Error responses carry CORS headers too. Without them the browser hides the
/// status from `fetch`, and the SPA's 401 and 422 handling would break.
#[test]
fn cors_headers_on_error_responses() {
    let c = Ctx::from_env();
    let t = c.login();
    let origin = "http://localhost:5173";
    let cases = [
        (
            "401",
            c.get("/api/chapters").header("Origin", origin).send(),
            401,
        ),
        (
            "422",
            c.post("/api/auth/login")
                .header("Origin", origin)
                .json(&json!({}))
                .send(),
            422,
        ),
        (
            "404 model",
            c.get("/api/chapters/nope")
                .bearer(&t)
                .header("Origin", origin)
                .send(),
            404,
        ),
        (
            "405",
            c.post("/api/chapters/foo")
                .bearer(&t)
                .header("Origin", origin)
                .send(),
            405,
        ),
        (
            "404 route",
            c.get("/api/nope").header("Origin", origin).send(),
            404,
        ),
    ];
    for (label, r, status) in cases {
        assert_eq!(r.status, status, "{label}");
        assert_eq!(
            r.header("access-control-allow-origin"),
            Some(origin),
            "{label}"
        );
        assert!(
            r.header("vary").is_some_and(|v| v.contains("Origin")),
            "{label} Vary"
        );
    }
}

/// Laravel matches routes against the **decoded** path with trailing
/// slashes removed (`UriValidator`: `rawurldecode(rtrim(pathInfo, '/'))`),
/// and the global `ValidatePathEncoding` middleware answers 400 when the
/// decoded path isn't UTF-8. Error messages still quote the raw path.
#[test]
fn paths_are_decoded_like_laravel() {
    let c = Ctx::from_env();
    let t = c.login();
    let model = |id: &str| json!({ "message": format!("No query results for model [App\\Models\\Chapter] {id}") });
    let route = |p: &str| json!({ "message": format!("The route {p} could not be found.") });

    let index = c.get("/api/chapters").bearer(&t).send().expect(200);
    for p in ["/api/chapters/", "/api/chapters//", "/api/%63hapters"] {
        assert_eq!(c.get(p).bearer(&t).send().expect(200), index, "{p}");
    }

    let cases = [
        (
            "/api/chapters/does-not-exist/",
            404,
            model("does-not-exist"),
        ),
        ("/api/chapters/a%20b", 404, model("a b")),
        ("/api/chapters/a+b", 404, model("a+b")),
        ("/api/chapters/A%41", 404, model("AA")),
        ("/api/chapters/a%2541", 404, model("a%41")),
        ("/api/chapters/%C3%A9t%C3%A9", 404, model("\u{e9}t\u{e9}")),
        ("/api/chapters/%", 404, model("%")),
        ("/api/chapters%2Freorder", 404, model("reorder")),
        ("/api/chapters/x%2F", 404, route("api/chapters/x%2F")),
        ("/api/chapters/a%2Fb", 404, route("api/chapters/a%2Fb")),
        ("/api//chapters", 404, route("api//chapters")),
        ("/api/nope/", 404, route("api/nope")),
    ];
    for (p, status, want) in cases {
        assert_eq!(c.get(p).bearer(&t).send().expect(status), want, "{p}");
    }

    // The decoded path picks the route, so this is POST /api/chapters/reorder
    // (a guest gets 401 from its auth guard, not a 404).
    c.post("/api/chapters%2Freorder").send().expect(401);

    // 405 messages quote the raw path, trimmed of slashes.
    let body = c.post("/api/chapters/foo/").bearer(&t).send().expect(405);
    assert_eq!(
        body,
        json!({ "message": "The POST method is not supported for route api/chapters/foo. Supported methods: GET, HEAD, PUT, DELETE." })
    );

    // Trailing slash on a ported POST route.
    let login = c
        .post("/api/auth/login/")
        .json(&json!({ "password": c.password }))
        .send();
    assert_eq!(login.status, 200, "{}", login.text());

    // Undecodable path: 400 before CORS, auth and routing.
    for p in ["/api/chapters/%FF", "/api/auth/login%C3", "/nope/%E9"] {
        let r = c.get(p).header("Origin", "http://localhost:5173").send();
        assert_eq!(r.expect(400), json!({ "message": "Malformed URL." }), "{p}");
        assert_eq!(r.header("Cache-Control"), Some("no-cache, private"), "{p}");
        assert_eq!(r.header("Access-Control-Allow-Origin"), None, "{p}");
    }
}
