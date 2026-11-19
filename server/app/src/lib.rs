//! Rust port of the grammaire backoffice API (`backend/`, Laravel 13).
//! Behaviour contract: `server/contract/` and `docs/migration/`.

pub mod auth;
pub mod config;
pub mod cors;
pub mod db;
pub mod error;
pub mod http;
pub mod idle;

pub use http::{AppState, router};
