//! R1 login, R2 logout, and the Sanctum bearer-token guard on every
//! protected route (INVENTORY §1.1, §2.4).

use contract::{Ctx, assert_validation, crc32b};
use serde_json::json;

const PROTECTED: &[(&str, &str)] = &[
    ("POST", "/api/auth/logout"),
    ("GET", "/api/chapters"),
    ("GET", "/api/chapters/subjonctif-present"),
    ("POST", "/api/chapters"),
    ("PUT", "/api/chapters/subjonctif-present"),
    ("DELETE", "/api/chapters/subjonctif-present"),
    ("POST", "/api/chapters/reorder"),
    ("GET", "/api/uploads"),
    ("POST", "/api/uploads"),
    ("DELETE", "/api/uploads/01aaaaaaaaaaaaaaaaaaaaaaaa"),
    ("POST", "/api/export"),
];

fn unauthenticated() -> serde_json::Value {
    json!({ "message": "Unauthenticated." })
}

#[test]
fn login_requires_password() {
    let c = Ctx::from_env();
    let r = c.post("/api/auth/login").json(&json!({})).send();
    assert_validation(&r, &[("password", &["The password field is required."])]);
}

#[test]
fn login_password_must_be_string() {
    let c = Ctx::from_env();
    let r = c
        .post("/api/auth/login")
        .json(&json!({ "password": 123 }))
        .send();
    assert_validation(
        &r,
        &[("password", &["The password field must be a string."])],
    );
}

#[test]
fn login_wrong_password() {
    let c = Ctx::from_env();
    let r = c
        .post("/api/auth/login")
        .json(&json!({ "password": "nope" }))
        .send();
    assert_validation(&r, &[("password", &["Invalid password."])]);
}

/// `password` is on TrimStrings' except list: padding is not stripped.
#[test]
fn login_padded_password_is_not_trimmed() {
    let c = Ctx::from_env();
    let padded = format!(" {} ", c.password);
    let r = c
        .post("/api/auth/login")
        .json(&json!({ "password": padded }))
        .send();
    assert_validation(&r, &[("password", &["Invalid password."])]);
}

/// Malformed JSON is treated as an empty body.
#[test]
fn login_malformed_json() {
    let c = Ctx::from_env();
    let r = c
        .post("/api/auth/login")
        .raw("application/json", b"{bad".to_vec())
        .send();
    assert_validation(&r, &[("password", &["The password field is required."])]);
}

#[test]
fn login_accepts_form_encoding() {
    let c = Ctx::from_env();
    let body = format!("password={}", c.password); // hex password, no escaping needed
    let r = c
        .post("/api/auth/login")
        .raw("application/x-www-form-urlencoded", body.into_bytes())
        .send();
    let b = r.expect(200);
    assert!(b["token"].is_string());
}

/// Sanctum 4: `<row id>|<40 alnum><crc32b hex of those 40>`, stored as sha256 hex.
#[test]
fn login_token_format_and_row() {
    let c = Ctx::from_env();
    let r = c
        .post("/api/auth/login")
        .json(&json!({ "password": c.password }))
        .send();
    let body = r.expect(200);
    assert_eq!(contract::keys(&body), ["token"]);
    let token = body["token"].as_str().unwrap();
    let (id, secret) = token.split_once('|').expect("id|secret");
    assert!(id.parse::<u64>().is_ok(), "id is numeric: {id}");
    assert_eq!(secret.len(), 48);
    let (entropy, check) = secret.split_at(40);
    assert!(entropy.chars().all(|ch| ch.is_ascii_alphanumeric()));
    assert_eq!(check, format!("{:08x}", crc32b(entropy.as_bytes())));

    let row = c.sqlite(&format!(
        "select tokenable_type, name, abilities, length(token), expires_at is null, \
         token glob '[0-9a-f]*' from personal_access_tokens where id = {id}"
    ));
    assert_eq!(row, "App\\Models\\User|admin-session|[\"*\"]|64|1|1");
}

#[test]
fn every_login_makes_a_new_token() {
    let c = Ctx::from_env();
    assert_ne!(c.login(), c.login());
}

#[test]
fn protected_routes_reject_guests() {
    let c = Ctx::from_env();
    for (method, path) in PROTECTED {
        let r = c.req(method, path).send();
        assert_eq!(r.expect(401), unauthenticated(), "{method} {path}");
    }
}

#[test]
fn protected_routes_reject_bad_tokens() {
    let c = Ctx::from_env();
    let good = c.login();
    let secret = good.split_once('|').unwrap().1;
    for bad in ["nope", "1|nope", &format!("999999|{secret}"), ""] {
        for (method, path) in PROTECTED {
            let r = c.req(method, path).bearer(bad).send();
            assert_eq!(
                r.expect(401),
                unauthenticated(),
                "{method} {path} with {bad:?}"
            );
        }
    }
}

/// `findToken` without a pipe looks the token up by hash alone.
#[test]
fn token_without_id_prefix_is_accepted() {
    let c = Ctx::from_env();
    let t = c.login();
    let secret = t.split_once('|').unwrap().1;
    assert_eq!(c.get("/api/uploads").bearer(secret).send().status, 200);
}

#[test]
fn bearer_scheme_is_case_insensitive() {
    let c = Ctx::from_env();
    let t = c.login();
    let r = c
        .get("/api/uploads")
        .header("Authorization", &format!("bearer {t}"))
        .send();
    assert_eq!(r.status, 200);
}

#[test]
fn expired_token_is_rejected() {
    let c = Ctx::from_env();
    let t = c.login();
    let id = t.split_once('|').unwrap().0;
    c.sqlite(&format!(
        "update personal_access_tokens set expires_at = '2000-01-01 00:00:00' where id = {id}"
    ));
    assert_eq!(
        c.get("/api/uploads").bearer(&t).send().expect(401),
        unauthenticated()
    );
    c.sqlite(&format!(
        "update personal_access_tokens set expires_at = '2999-01-01 00:00:00' where id = {id}"
    ));
    assert_eq!(
        c.get("/api/uploads").bearer(&t).send().status,
        200,
        "future expiry is valid"
    );
}

#[test]
fn authenticated_request_sets_last_used_at() {
    let c = Ctx::from_env();
    let t = c.login();
    let id = t.split_once('|').unwrap().0;
    c.sqlite(&format!(
        "update personal_access_tokens set last_used_at = null where id = {id}"
    ));
    assert_eq!(c.get("/api/uploads").bearer(&t).send().status, 200);
    let used = c.sqlite(&format!(
        "select last_used_at from personal_access_tokens where id = {id}"
    ));
    assert_eq!(used.len(), 19, "Y-m-d H:i:s text, got {used:?}");
    assert_eq!(&used[4..5], "-");
    assert_eq!(&used[10..11], " ");
}

#[test]
fn logout_revokes_only_the_current_token() {
    let c = Ctx::from_env();
    let a = c.login();
    let b = c.login();
    let body = c.post("/api/auth/logout").bearer(&a).send().expect(200);
    assert_eq!(body, json!({ "ok": true }));
    assert_eq!(
        c.get("/api/uploads").bearer(&a).send().expect(401),
        unauthenticated()
    );
    assert_eq!(c.get("/api/uploads").bearer(&b).send().status, 200);
}

/// A token whose user row is gone is invalid; the next login recreates the
/// `editor@local` user (firstOrCreate) with a bcrypt `$2y$` password hash.
#[test]
fn deleted_user_invalidates_tokens_and_login_recreates_it() {
    let c = Ctx::from_env();
    let t = c.login();
    c.sqlite("delete from users");
    assert_eq!(
        c.get("/api/uploads").bearer(&t).send().expect(401),
        unauthenticated()
    );
    let t2 = c.login();
    assert_eq!(c.get("/api/uploads").bearer(&t2).send().status, 200);
    let user = c.sqlite("select name, email, substr(password, 1, 4) from users");
    assert_eq!(user, "Editor|editor@local|$2y$");
}
