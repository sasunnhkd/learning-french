//! Laravel-compatible error responses. Bodies are `{"message": "..."}` with
//! the exact strings Laravel 13 produces with `APP_DEBUG=false`
//! (captured in Phase 1; see docs/migration/INVENTORY.md §1.4). The 422
//! validation shape is added with the validation engine in Phase 3.
//!
//! Internal errors are logged with their cause and answered with
//! `Server Error`, so nothing internal leaks to the client.

use axum::Json;
use axum::http::{HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use serde_json::json;

/// Eloquent model names used in `findOrFail` messages.
pub const CHAPTER_MODEL: &str = "App\\Models\\Chapter";
pub const UPLOAD_MODEL: &str = "App\\Models\\Upload";

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Unauthenticated.")]
    Unauthenticated,
    #[error("No query results for model [{model}] {id}")]
    ModelNotFound { model: &'static str, id: String },
    /// `path` is Laravel's `$request->path()`: no leading slash, `/` for root.
    #[error("The route {path} could not be found.")]
    RouteNotFound { path: String },
    #[error("The {method} method is not supported for route {path}. Supported methods: {allowed}.")]
    MethodNotAllowed {
        method: String,
        path: String,
        /// Comma-and-space separated, as in the `Allow` header Laravel sends.
        allowed: String,
    },
    #[error("The POST data is too large.")]
    PostTooLarge,
    /// Route exists but its slice is not ported yet (removed by end of Phase 3).
    #[error("Not implemented yet.")]
    NotImplemented,
    #[error("Server Error")]
    Internal(#[from] anyhow::Error),
}

impl AppError {
    pub fn status(&self) -> StatusCode {
        match self {
            AppError::Unauthenticated => StatusCode::UNAUTHORIZED,
            AppError::ModelNotFound { .. } | AppError::RouteNotFound { .. } => {
                StatusCode::NOT_FOUND
            }
            AppError::MethodNotAllowed { .. } => StatusCode::METHOD_NOT_ALLOWED,
            AppError::PostTooLarge => StatusCode::PAYLOAD_TOO_LARGE,
            AppError::NotImplemented => StatusCode::NOT_IMPLEMENTED,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        AppError::Internal(e.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        if let AppError::Internal(cause) = &self {
            tracing::error!(error = %format!("{cause:#}"), "internal error");
        }
        let status = self.status();
        let mut res = (status, Json(json!({ "message": self.to_string() }))).into_response();
        if let AppError::MethodNotAllowed { allowed, .. } = &self
            && let Ok(v) = HeaderValue::from_str(allowed)
        {
            res.headers_mut().insert(header::ALLOW, v);
        }
        res
    }
}

/// Laravel's `$request->path()`: the path without surrounding slashes, or `/`.
pub fn laravel_path(uri_path: &str) -> String {
    let trimmed = uri_path.trim_matches('/');
    if trimmed.is_empty() {
        "/".to_string()
    } else {
        trimmed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn messages_match_laravel() {
        assert_eq!(AppError::Unauthenticated.to_string(), "Unauthenticated.");
        assert_eq!(
            AppError::ModelNotFound {
                model: CHAPTER_MODEL,
                id: "x".into()
            }
            .to_string(),
            "No query results for model [App\\Models\\Chapter] x"
        );
        assert_eq!(
            AppError::RouteNotFound {
                path: laravel_path("/api/nope")
            }
            .to_string(),
            "The route api/nope could not be found."
        );
        assert_eq!(
            AppError::MethodNotAllowed {
                method: "POST".into(),
                path: "api/chapters/foo".into(),
                allowed: "GET, HEAD, PUT, DELETE".into()
            }
            .to_string(),
            "The POST method is not supported for route api/chapters/foo. Supported methods: GET, HEAD, PUT, DELETE."
        );
        assert_eq!(
            AppError::Internal(anyhow::anyhow!("secret detail")).to_string(),
            "Server Error"
        );
    }

    #[test]
    fn laravel_path_rules() {
        assert_eq!(laravel_path("/"), "/");
        assert_eq!(laravel_path(""), "/");
        assert_eq!(laravel_path("/api/x/"), "api/x");
    }
}
