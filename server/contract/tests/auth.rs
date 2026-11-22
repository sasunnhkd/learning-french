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

/// `required` uses PHP's `trim` (`" \t\n\r\0\x0B"`), and `password` skips
/// TrimStrings, so a password of other blank characters is checked against
/// the real one (found in slice 3.3).
#[test]
fn login_unicode_blank_password_is_checked() {
    let c = Ctx::from_env();
    for pw in ["\u{a0}", "\u{c}", "\u{200b}"] {
        let r = c
            .post("/api/auth/login")
            .json(&json!({ "password": pw }))
            .send();
        assert_validation(&r, &[("password", &["Invalid password."])]);
    }
    let r = c
        .post("/api/auth/login")
        .json(&json!({ "password": " \t" }))
        .send();
    assert_validation(&r, &[("password", &["The password field is required."])]);
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

/// K14 (default fix, pending confirmation): Laravel merges the query string
/// into the input, so a password in the URL logs in. Rust reads the body only.
#[test]
fn login_password_in_query_string() {
    let c = Ctx::from_env();
    let r = c
        .post(&format!("/api/auth/login?password={}", c.password))
        .json(&json!({}))
        .send();
    if c.is_laravel() {
        assert!(r.expect(200)["token"].is_string());
    } else {
        assert_validation(&r, &[("password", &["The password field is required."])]);
    }
}

/// Input formats Laravel reads: multipart fields, `+json` and charset
/// parameters, PHP `a[]` arrays; JSON that is not an object; non-JSON types.
#[test]
fn login_input_formats() {
    let c = Ctx::from_env();
    let pw = c.password.clone();
    let ok = |r: contract::Resp| assert!(r.expect(200)["token"].is_string());
    ok(c.post("/api/auth/login")
        .multipart("password", None, "", pw.as_bytes())
        .send());
    for ct in [
        "application/vnd.api+json",
        "application/json; charset=utf-8",
    ] {
        ok(c.post("/api/auth/login")
            .raw(ct, serde_json::to_vec(&json!({ "password": pw })).unwrap())
            .send());
    }
    let dup = format!("password=nope&password={pw}");
    ok(c.post("/api/auth/login")
        .raw("application/x-www-form-urlencoded", dup.into_bytes())
        .send());

    let required: &[&str] = &["The password field is required."];
    let string: &[&str] = &["The password field must be a string."];
    let cases: Vec<(&str, Vec<u8>, &[&str])> = vec![
        (
            "text/plain",
            serde_json::to_vec(&json!({ "password": pw })).unwrap(),
            required,
        ),
        ("application/json", b"[\"x\"]".to_vec(), required),
        ("application/json", b"\"x\"".to_vec(), required),
        (
            "application/json",
            br#"{"password":null}"#.to_vec(),
            required,
        ),
        (
            "application/json",
            br#"{"password":"   "}"#.to_vec(),
            required,
        ),
        ("application/json", br#"{"password":true}"#.to_vec(), string),
        (
            "application/x-www-form-urlencoded",
            format!("password[]={pw}").into_bytes(),
            string,
        ),
    ];
    for (ct, body, msgs) in cases {
        let r = c.post("/api/auth/login").raw(ct, body).send();
        assert_validation(&r, &[("password", msgs)]);
    }
}

#[test]
fn login_get_is_405() {
    let c = Ctx::from_env();
    let body = c.get("/api/auth/login").send().expect(405);
    assert_eq!(
        body,
        json!({ "message": "The GET method is not supported for route api/auth/login. Supported methods: POST." })
    );
}

/// `bearerToken()` is not trimmed, so an extra space after `Bearer` makes the
/// token invalid on both apps. K15: a trailing space differs by HTTP stack.
/// PHP's server keeps it (token invalid); hyper strips surrounding header
/// whitespace as RFC 9110 requires (token valid).
#[test]
fn bearer_with_extra_spaces() {
    let c = Ctx::from_env();
    let t = c.login();
    let r = c
        .get("/api/uploads")
        .header("Authorization", &format!("Bearer  {t}"))
        .send();
    assert_eq!(r.expect(401), unauthenticated(), "double space");
    let r = c
        .get("/api/uploads")
        .header("Authorization", &format!("Bearer {t} "))
        .send();
    assert_eq!(
        r.status,
        c.expect_by_target("K15", 401, 200),
        "trailing space"
    );
}

/// Q5 / rollback: a token Laravel's own Sanctum mints is accepted, in both
/// the `id|secret` and the secret-only forms.
#[test]
fn laravel_minted_token_is_accepted() {
    let c = Ctx::from_env();
    let token = c.php(
        "$u = App\\Models\\User::firstOrCreate(['email' => 'editor@local'], \
         ['name' => 'Editor', 'password' => bcrypt(bin2hex(random_bytes(16)))]); \
         echo $u->createToken('admin-session')->plainTextToken;",
        &[],
    );
    let (_, secret) = token.split_once('|').expect("id|secret");
    for t in [token.as_str(), secret] {
        assert_eq!(c.get("/api/uploads").bearer(t).send().status, 200, "{t}");
    }
}

/// Rollback: a token the server under test mints is valid for Laravel's
/// Sanctum (`findToken` + tokenable user) on the same database.
#[test]
fn server_minted_token_is_valid_in_laravel() {
    let c = Ctx::from_env();
    let token = c.login();
    let verdict = c.php(
        "$t = Laravel\\Sanctum\\PersonalAccessToken::findToken($argv[1]); \
         echo ($t && $t->tokenable instanceof App\\Models\\User && $t->name === 'admin-session' \
               && $t->can('anything')) ? 'valid' : 'invalid';",
        &[&token],
    );
    assert_eq!(verdict, "valid");
}
