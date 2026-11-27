//! Auth: R1 login, R2 logout, and the bearer-token guard, compatible with
//! Laravel Sanctum 4.3 (INVENTORY §2.4, `vendor/laravel/sanctum/src/
//! {HasApiTokens,PersonalAccessToken,Guard}.php`).
//!
//! - **Token string:** `SANCTUM_TOKEN_PREFIX` + 40 random `[A-Za-z0-9]` +
//!   the crc32b hex of those 40. The client gets `"<row id>|<token>"`. The
//!   row stores `sha256(<token>)` as hex.
//! - **Lookup** (`PersonalAccessToken::findToken`):
//!   - with a `|`: find by id, then compare hashes in constant time;
//!   - without: find by hash.
//! - **Valid when:** `expires_at` is null or not past, the tokenable is an
//!   existing `App\Models\User`, and no global expiry applies
//!   (`sanctum.expiration` is null).
//! - **Each use** sets `last_used_at` and `updated_at`, as Eloquent `save()`.
//! - **Login:**
//!   - validates `password: required|string`, then compares with
//!     `EDITOR_PASSWORD` like `hash_equals`;
//!   - `firstOrCreate`s the `editor@local` user with a bcrypt `$2y$` hash of
//!     random bytes, which is never checked;
//!   - creates a token named `admin-session` with abilities `["*"]` and no
//!     expiry.
//! - **Logout** deletes only the current token.
//!
//! Timestamps come from SQLite's `datetime('now')`, which gives UTC
//! `Y-m-d H:i:s`, the format Laravel writes and reads.

use crate::error::AppError;
use crate::http::AppState;
use crate::input::Input;
use crate::validation::{ValidationErrors, required_string};
use axum::Json;
use axum::extract::{ConnectInfo, FromRequestParts, State};
use axum::http::header;
use axum::http::request::Parts;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::convert::Infallible;
use std::net::IpAddr;
use std::net::SocketAddr;

/// `tokenable_type` of the editor user (Eloquent morph class).
pub const USER_MODEL: &str = "App\\Models\\User";
pub const EDITOR_EMAIL: &str = "editor@local";
pub const EDITOR_NAME: &str = "Editor";
pub const TOKEN_NAME: &str = "admin-session";
pub const TOKEN_ABILITIES: &str = "[\"*\"]";

/// The authenticated editor. Add it as a handler argument to protect a route.
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: i64,
    pub token_id: i64,
}

/// Laravel's `Request::bearerToken()`: the text after the last
/// case-insensitive `Bearer ` in `Authorization`, cut at the first comma.
/// Empty means no token. Not trimmed: `Bearer  x` yields ` x`, which never
/// matches, as in Laravel.
pub fn bearer_token(authorization: &str) -> Option<&str> {
    let lower = authorization.to_ascii_lowercase();
    let pos = lower.rfind("bearer ")?;
    let rest = &authorization[pos + "bearer ".len()..];
    let token = rest.split(',').next().unwrap_or("");
    (!token.is_empty()).then_some(token)
}

/// PHP `hash('crc32b', ...)`, as lowercase hex.
pub fn crc32b_hex(data: &[u8]) -> String {
    let mut crc = 0xFFFF_FFFFu32;
    for &b in data {
        crc ^= u32::from(b);
        for _ in 0..8 {
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    format!("{:08x}", !crc)
}

pub fn sha256_hex(data: &str) -> String {
    Sha256::digest(data.as_bytes())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

/// PHP `hash_equals`: constant time for equal lengths.
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let diff = a.iter().zip(b).fold(0u8, |acc, (x, y)| acc | (x ^ y));
    std::hint::black_box(diff) == 0
}

const ALPHANUM: &[u8; 62] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

/// 40 uniformly random `[A-Za-z0-9]` characters from the OS CSPRNG
/// (rejection sampling, so no modulo bias).
pub fn random_alnum(len: usize) -> Result<String, getrandom::Error> {
    let mut out = String::with_capacity(len);
    let mut buf = [0u8; 64];
    while out.len() < len {
        getrandom::fill(&mut buf)?;
        for &b in &buf {
            // 248 = 4 * 62: accept 0..248 so every symbol is equally likely.
            if b < 248 && out.len() < len {
                out.push(char::from(ALPHANUM[usize::from(b % 62)]));
            }
        }
    }
    Ok(out)
}

/// Sanctum's `generateTokenString()`.
pub fn generate_token_string(prefix: &str) -> Result<String, getrandom::Error> {
    let entropy = random_alnum(40)?;
    let check = crc32b_hex(entropy.as_bytes());
    Ok(format!("{prefix}{entropy}{check}"))
}

/// Resolve a bearer token to a valid token row, and mark it used.
pub async fn authenticate(state: &AppState, raw: &str) -> Result<Option<AuthUser>, AppError> {
    let found = match raw.split_once('|') {
        Some((id, secret)) => {
            let Ok(id) = id.parse::<i64>() else {
                return Ok(None);
            };
            let row = sqlx::query!(
                r#"select t.id as "id!", t.tokenable_id, t.token
                   from personal_access_tokens t
                   join users u on u.id = t.tokenable_id
                   where t.id = ? and t.tokenable_type = ?
                     and (t.expires_at is null or t.expires_at >= datetime('now'))"#,
                id,
                USER_MODEL
            )
            .fetch_optional(&state.db)
            .await?;
            row.filter(|r| constant_time_eq(r.token.as_bytes(), sha256_hex(secret).as_bytes()))
                .map(|r| (r.id, r.tokenable_id))
        }
        None => {
            let hash = sha256_hex(raw);
            sqlx::query!(
                r#"select t.id as "id!", t.tokenable_id
                   from personal_access_tokens t
                   join users u on u.id = t.tokenable_id
                   where t.token = ? and t.tokenable_type = ?
                     and (t.expires_at is null or t.expires_at >= datetime('now'))"#,
                hash,
                USER_MODEL
            )
            .fetch_optional(&state.db)
            .await?
            .map(|r| (r.id, r.tokenable_id))
        }
    };
    let Some((token_id, user_id)) = found else {
        return Ok(None);
    };
    sqlx::query!(
        "update personal_access_tokens set last_used_at = datetime('now'), updated_at = datetime('now') where id = ?",
        token_id
    )
    .execute(&state.db)
    .await?;
    Ok(Some(AuthUser { user_id, token_id }))
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(bearer_token)
            .map(str::to_string);
        let Some(token) = token else {
            return Err(AppError::Unauthenticated);
        };
        authenticate(state, &token)
            .await?
            .ok_or(AppError::Unauthenticated)
    }
}

/// `User::firstOrCreate(['email' => 'editor@local'], [...])`.
async fn editor_user_id(state: &AppState) -> Result<i64, AppError> {
    if let Some(id) =
        sqlx::query_scalar!("select id from users where email = ? limit 1", EDITOR_EMAIL)
            .fetch_optional(&state.db)
            .await?
    {
        return Ok(id);
    }
    let rounds = state.config.bcrypt_rounds;
    let hash = tokio::task::spawn_blocking(move || -> anyhow::Result<String> {
        let mut bytes = [0u8; 16];
        getrandom::fill(&mut bytes).map_err(|e| anyhow::anyhow!("getrandom: {e}"))?;
        let secret: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        Ok(bcrypt::hash_with_result(secret, rounds)?.format_for_version(bcrypt::Version::TwoY))
    })
    .await
    .map_err(|e| AppError::Internal(e.into()))??;
    let inserted = sqlx::query_scalar!(
        r#"insert into users (name, email, password, created_at, updated_at)
           values (?, ?, ?, datetime('now'), datetime('now')) returning id as "id!""#,
        EDITOR_NAME,
        EDITOR_EMAIL,
        hash
    )
    .fetch_one(&state.db)
    .await;
    match inserted {
        Ok(id) => Ok(id),
        // Lost a race with a concurrent first login: the row exists now.
        Err(sqlx::Error::Database(e)) if e.is_unique_violation() => Ok(sqlx::query_scalar!(
            "select id from users where email = ? limit 1",
            EDITOR_EMAIL
        )
        .fetch_one(&state.db)
        .await?),
        Err(e) => Err(e.into()),
    }
}

/// `$user->createToken('admin-session')->plainTextToken`.
async fn create_token(state: &AppState, user_id: i64) -> Result<String, AppError> {
    let plain = generate_token_string(&state.config.sanctum_token_prefix)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("getrandom: {e}")))?;
    let hash = sha256_hex(&plain);
    let id = sqlx::query_scalar!(
        r#"insert into personal_access_tokens
             (tokenable_type, tokenable_id, name, token, abilities, expires_at, created_at, updated_at)
           values (?, ?, ?, ?, ?, null, datetime('now'), datetime('now')) returning id as "id!""#,
        USER_MODEL,
        user_id,
        TOKEN_NAME,
        hash,
        TOKEN_ABILITIES
    )
    .fetch_one(&state.db)
    .await?;
    Ok(format!("{id}|{plain}"))
}

/// The address the login limiter keys on (Phase 5 audit F3). Behind nginx
/// on the same host every TCP peer is loopback, so one bucket would cover
/// every client. When the peer is loopback, nginx's `X-Real-IP` (see
/// `server/packaging/nginx-grammaire.conf`) is used if it parses as an IP;
/// otherwise the peer. Only a process on the host itself can set the
/// header, and it could reach the server directly anyway.
pub fn client_ip(
    peer: Option<IpAddr>,
    real_ip: Option<&axum::http::HeaderValue>,
) -> Option<IpAddr> {
    match peer {
        Some(p) if p.is_loopback() => real_ip
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.trim().parse::<IpAddr>().ok())
            .or(peer),
        other => other,
    }
}

/// The client address for the login limiter: [`client_ip`] of the peer
/// (present when the server runs with connect info, always in `main`;
/// absent in in-process tests) and `X-Real-IP`.
#[derive(Debug, Clone, Copy)]
pub struct ClientIp(pub Option<IpAddr>);

impl<S: Send + Sync> FromRequestParts<S> for ClientIp {
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let peer = parts
            .extensions
            .get::<ConnectInfo<SocketAddr>>()
            .map(|c| c.0.ip());
        Ok(ClientIp(client_ip(peer, parts.headers.get("x-real-ip"))))
    }
}

/// R1 `POST /api/auth/login`.
pub async fn login(
    State(state): State<AppState>,
    ClientIp(ip): ClientIp,
    input: Input,
) -> Result<Json<Value>, AppError> {
    state
        .login_limiter
        .hit(ip)
        .map_err(|retry_after_secs| AppError::TooManyAttempts { retry_after_secs })?;

    let mut errors = ValidationErrors::new();
    let password = required_string(&input.fields, "password", &mut errors).map(str::to_string);
    errors.into_result()?;
    let password = password.unwrap_or_default();

    let ok = state
        .config
        .editor_password
        .as_ref()
        .is_some_and(|expected| {
            // Compare digests: fixed 32 bytes, so timing doesn't reveal the
            // password's length (PHP's hash_equals does; Phase 5 audit).
            constant_time_eq(
                &Sha256::digest(expected.expose().as_bytes()),
                &Sha256::digest(password.as_bytes()),
            )
        });
    if !ok {
        return Err(AppError::Validation(ValidationErrors::single(
            "password",
            "Invalid password.",
        )));
    }

    let user_id = editor_user_id(&state).await?;
    let token = create_token(&state, user_id).await?;
    Ok(Json(json!({ "token": token })))
}

/// R2 `POST /api/auth/logout`.
pub async fn logout(
    State(state): State<AppState>,
    user: AuthUser,
) -> Result<Json<Value>, AppError> {
    sqlx::query!(
        "delete from personal_access_tokens where id = ?",
        user.token_id
    )
    .execute(&state.db)
    .await?;
    Ok(Json(json!({ "ok": true })))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bearer_parsing_matches_laravel() {
        assert_eq!(bearer_token("Bearer abc"), Some("abc"));
        assert_eq!(bearer_token("bearer abc"), Some("abc"));
        assert_eq!(bearer_token("BEARER 1|x"), Some("1|x"));
        assert_eq!(bearer_token("Bearer a,b"), Some("a"));
        assert_eq!(bearer_token("Bearer x Bearer y"), Some("y"));
        assert_eq!(bearer_token("Bearer  x"), Some(" x"));
        assert_eq!(bearer_token("Bearer "), None);
        assert_eq!(bearer_token("Basic abc"), None);
        assert_eq!(bearer_token(""), None);
    }

    #[test]
    fn crc32b_matches_php() {
        // php -r 'echo hash("crc32b", "hello");'
        assert_eq!(crc32b_hex(b"hello"), "3610a686");
        assert_eq!(crc32b_hex(b""), "00000000");
    }

    #[test]
    fn sha256_matches_php() {
        // php -r 'echo hash("sha256", "abc");'
        assert_eq!(
            sha256_hex("abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn token_string_shape() {
        let t = generate_token_string("pfx_").unwrap();
        let body = t.strip_prefix("pfx_").unwrap();
        assert_eq!(body.len(), 48);
        let (entropy, check) = body.split_at(40);
        assert!(entropy.bytes().all(|b| b.is_ascii_alphanumeric()));
        assert_eq!(check, crc32b_hex(entropy.as_bytes()));
        assert_ne!(
            generate_token_string("").unwrap(),
            generate_token_string("").unwrap()
        );
    }

    #[test]
    fn client_ip_trusts_x_real_ip_only_from_loopback() {
        use axum::http::HeaderValue;
        let lo: IpAddr = "127.0.0.1".parse().unwrap();
        let ext: IpAddr = "203.0.113.9".parse().unwrap();
        let hdr = HeaderValue::from_static("198.51.100.7");
        let want: IpAddr = "198.51.100.7".parse().unwrap();
        assert_eq!(client_ip(Some(lo), Some(&hdr)), Some(want));
        assert_eq!(
            client_ip(Some(ext), Some(&hdr)),
            Some(ext),
            "not from a proxy"
        );
        assert_eq!(client_ip(Some(lo), None), Some(lo));
        let bad = HeaderValue::from_static("not-an-ip");
        assert_eq!(client_ip(Some(lo), Some(&bad)), Some(lo));
        assert_eq!(client_ip(None, Some(&hdr)), None);
    }

    #[test]
    fn constant_time_eq_basics() {
        assert!(constant_time_eq(b"abc", b"abc"));
        assert!(!constant_time_eq(b"abc", b"abd"));
        assert!(!constant_time_eq(b"abc", b"abcd"));
        assert!(constant_time_eq(b"", b""));
    }
}
