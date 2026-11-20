//! Router and middleware stack.
//!
//! Routes mirror `backend/routes/api.php` (INVENTORY §1.1), plus `/up`.
//! The scaffold routes `/`, `/sanctum/csrf-cookie` and `/storage/*` are
//! dropped on purpose (KNOWN_ISSUES K12, plan Q6).
//!
//! Middleware, outermost first:
//! 1. request id (`x-request-id`, generated if absent, echoed back)
//! 2. tracing (method, path, status, latency; never headers or bodies)
//! 3. idle tracking (`IDLE_TIMEOUT_SECS`)
//! 4. Laravel default headers (`Cache-Control: no-cache, private`)
//! 5. path handling (`paths.rs`): 400 "Malformed URL." for undecodable
//!    paths, then route on the decoded path without trailing slashes, as
//!    Laravel's `ValidatePathEncoding` and `UriValidator` do
//! 6. CORS (`cors.rs`, the same as Laravel's HandleCors)
//! 7. post size: 413 above 8 MiB, as PHP `post_max_size=8M` / ValidatePostSize
//! 8. request timeout (`REQUEST_TIMEOUT_SECS`, answered with 408)
//! 9. 405 bodies in Laravel's format
//!
//! Unknown paths get Laravel's 404 JSON from the fallback. Both the 404 and
//! the 405 messages quote the raw request path (`paths::raw_path`), not the
//! rewritten one.
//!
//! To port a slice, replace its `protected_stub` handler below
//! with the real handler module, and add its contract tests to
//! `server/contract/ported.txt`.

use crate::auth::AuthUser;
use crate::config::Config;
use crate::cors::{self, Cors};
use crate::db;
use crate::error::{AppError, laravel_path};
use crate::idle::Activity;
use crate::limiter::LoginLimiter;
use crate::{auth, paths, uploads};
use axum::Router;
use axum::extract::{DefaultBodyLimit, Request, State};
use axum::http::{HeaderValue, StatusCode, header};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use sqlx::SqlitePool;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;

/// PHP `post_max_size=8M` (Laravel's ValidatePostSize compares Content-Length).
pub const POST_MAX_BYTES: u64 = 8 * 1024 * 1024;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db: SqlitePool,
    pub activity: Arc<Activity>,
    pub login_limiter: Arc<LoginLimiter>,
}

impl AppState {
    pub fn new(config: Config, db: SqlitePool) -> Self {
        AppState {
            db,
            activity: Activity::new(),
            login_limiter: Arc::new(LoginLimiter::new(config.login_rate_limit)),
            config: Arc::new(config),
        }
    }
}

pub fn router(state: AppState) -> Router {
    let cors = Cors::new(state.config.allowed_origins.clone());
    let activity = state.activity.clone();
    let timeout = state.config.request_timeout;

    // `reorder` is a static segment, so the router prefers it over `{id}`.
    // Laravel matches GET/PUT/DELETE `/api/chapters/reorder` against
    // `{id}` (id = "reorder"), so those methods are registered here too and
    // must call the `{id}` handlers.
    let routes = Router::new()
        .route("/up", get(up))
        .route("/api/auth/login", post(auth::login))
        .route("/api/auth/logout", post(auth::logout))
        .route("/api/chapters", get(protected_stub).post(protected_stub))
        .route(
            "/api/chapters/reorder",
            post(protected_stub)
                .get(protected_stub)
                .put(protected_stub)
                .delete(protected_stub),
        )
        .route(
            "/api/chapters/{id}",
            get(protected_stub)
                .put(protected_stub)
                .delete(protected_stub),
        )
        .route("/api/uploads", get(uploads::index).post(protected_stub))
        .route("/api/uploads/{id}", axum::routing::delete(protected_stub))
        .route("/api/export", post(protected_stub))
        .fallback(route_not_found)
        .with_state(state);

    // Wrap the finished inner router as ONE service. `Router::layer` would
    // wrap each route separately, and axum adds the `Allow` header to a 405
    // only after those per-route layers have run, so
    // `method_not_allowed_body` would never see it.
    Router::new().fallback_service(routes).layer(
        ServiceBuilder::new()
            .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                    .on_response(DefaultOnResponse::new().level(Level::INFO)),
            )
            .layer(PropagateRequestIdLayer::x_request_id())
            .layer(middleware::from_fn_with_state(activity, touch_activity))
            .layer(middleware::from_fn(laravel_headers))
            .layer(middleware::from_fn(paths::middleware))
            .layer(middleware::from_fn_with_state(cors, cors::middleware))
            .layer(middleware::from_fn(limit_post_size))
            .layer(DefaultBodyLimit::max(
                usize::try_from(POST_MAX_BYTES).unwrap_or(usize::MAX),
            ))
            .layer(TimeoutLayer::with_status_code(
                StatusCode::REQUEST_TIMEOUT,
                timeout,
            ))
            .layer(middleware::from_fn(method_not_allowed_body)),
    )
}

async fn up(State(state): State<AppState>) -> Response {
    match db::ping(&state.db).await {
        Ok(()) => (StatusCode::OK, "OK").into_response(),
        Err(e) => AppError::from(e).into_response(),
    }
}

async fn route_not_found(req: Request) -> AppError {
    AppError::RouteNotFound {
        path: laravel_path(&paths::raw_path(&req)),
    }
}

/// Protected route whose slice is not ported yet: guests get 401 first.
async fn protected_stub(_user: AuthUser) -> AppError {
    AppError::NotImplemented
}

async fn touch_activity(
    State(activity): State<Arc<Activity>>,
    req: Request,
    next: Next,
) -> Response {
    activity.touch();
    let res = next.run(req).await;
    activity.touch();
    res
}

async fn laravel_headers(req: Request, next: Next) -> Response {
    let mut res = next.run(req).await;
    res.headers_mut()
        .entry(header::CACHE_CONTROL)
        .or_insert(HeaderValue::from_static("no-cache, private"));
    res
}

async fn limit_post_size(req: Request, next: Next) -> Response {
    let too_large = req
        .headers()
        .get(header::CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.trim().parse::<u64>().ok())
        .is_some_and(|len| len > POST_MAX_BYTES);
    if too_large {
        return AppError::PostTooLarge.into_response();
    }
    next.run(req).await
}

/// Give the router's bare 405 Laravel's JSON body and `Allow` format.
async fn method_not_allowed_body(req: Request, next: Next) -> Response {
    let method = req.method().to_string();
    let path = laravel_path(&paths::raw_path(&req));
    let res = next.run(req).await;
    if res.status() != StatusCode::METHOD_NOT_ALLOWED {
        return res;
    }
    let allowed = res
        .headers()
        .get(header::ALLOW)
        .and_then(|v| v.to_str().ok())
        .map(|v| {
            v.split(',')
                .map(str::trim)
                .filter(|m| !m.is_empty())
                .collect::<Vec<_>>()
                .join(", ")
        })
        .unwrap_or_default();
    AppError::MethodNotAllowed {
        method,
        path,
        allowed,
    }
    .into_response()
}
