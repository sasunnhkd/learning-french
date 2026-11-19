//! Bearer-token guard, compatible with Laravel Sanctum 4 (INVENTORY §2.4).
//!
//! Phase 2 status: the guard **fails closed**. Every request is
//! unauthenticated until slice 3.1 adds the `personal_access_tokens` lookup
//! (sha256 compare, `expires_at`, tokenable user, `last_used_at`).

use crate::error::AppError;
use axum::extract::FromRequestParts;
use axum::http::header;
use axum::http::request::Parts;

/// The authenticated editor. Add it as a handler argument to protect a route.
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: i64,
    pub token_id: i64,
}

/// Laravel's `Request::bearerToken()`: the text after the last
/// case-insensitive `Bearer ` in `Authorization`, cut at the first comma.
/// Empty means no token.
pub fn bearer_token(authorization: &str) -> Option<&str> {
    let lower = authorization.to_ascii_lowercase();
    let pos = lower.rfind("bearer ")?;
    let rest = &authorization[pos + "bearer ".len()..];
    let token = rest.split(',').next().unwrap_or("");
    (!token.is_empty()).then_some(token)
}

impl<S: Send + Sync> FromRequestParts<S> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(bearer_token);
        // Slice 3.1: look `token` up. Until then nothing authenticates,
        // with or without a token (fail closed).
        let _ = token;
        Err(AppError::Unauthenticated)
    }
}

#[cfg(test)]
mod tests {
    use super::bearer_token;

    #[test]
    fn bearer_parsing_matches_laravel() {
        assert_eq!(bearer_token("Bearer abc"), Some("abc"));
        assert_eq!(bearer_token("bearer abc"), Some("abc"));
        assert_eq!(bearer_token("BEARER 1|x"), Some("1|x"));
        assert_eq!(bearer_token("Bearer a,b"), Some("a"));
        assert_eq!(bearer_token("Bearer x Bearer y"), Some("y"));
        assert_eq!(bearer_token("Bearer "), None);
        assert_eq!(bearer_token("Basic abc"), None);
        assert_eq!(bearer_token(""), None);
    }
}
