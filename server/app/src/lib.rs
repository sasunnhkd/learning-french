//! Rust port of the grammaire backoffice API (`backend/`, Laravel 13).
//! Behaviour contract: `server/contract/` and `docs/migration/`.

pub mod auth;
pub mod chapters;
pub mod config;
pub mod cors;
pub mod db;
pub mod error;
pub mod http;
pub mod idle;
pub mod input;
pub mod limiter;
pub mod paths;
pub mod php;
pub mod uploads;
pub mod validation;

pub use http::{AppState, router};
