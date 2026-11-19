//! Shared helpers for the app's in-process tests. Each test binary uses a
//! subset, so unused-helper warnings are expected per binary.
#![allow(dead_code)]

use grammaire_server::config::Config;
use grammaire_server::{AppState, db};
use sqlx::SqlitePool;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

/// A unique path in the system temp dir (no `tempfile` crate needed).
pub fn temp_path(label: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_nanos());
    std::env::temp_dir().join(format!(
        "grammaire-{label}-{}-{nanos}.sqlite",
        std::process::id()
    ))
}

/// Deletes the database file (and SQLite side files) on drop.
pub struct TempDb {
    pub path: PathBuf,
    pub pool: SqlitePool,
}

impl Drop for TempDb {
    fn drop(&mut self) {
        for suffix in ["", "-wal", "-shm", "-journal"] {
            let mut p = self.path.clone().into_os_string();
            p.push(suffix);
            let _ = std::fs::remove_file(p);
        }
    }
}

pub async fn fresh_db(label: &str) -> TempDb {
    let path = temp_path(label);
    let pool = db::fresh_for_tests(&path).await.expect("fresh db");
    TempDb { path, pool }
}

pub fn test_config(db_path: &std::path::Path) -> Config {
    let url = format!("sqlite:{}", db_path.display());
    let repo = std::env::temp_dir();
    Config::from_lookup(
        |k| match k {
            "DATABASE_URL" => Some(url.clone()),
            "REPO_ROOT" => Some(repo.display().to_string()),
            _ => None,
        },
        &repo,
    )
    .expect("config")
}

pub async fn app() -> (axum::Router, TempDb) {
    let db = fresh_db("router").await;
    let state = AppState::new(test_config(&db.path), db.pool.clone());
    (grammaire_server::router(state), db)
}
