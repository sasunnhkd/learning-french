//! Shared helpers for the app's in-process tests. Each test binary uses a
//! subset, so unused-helper warnings are expected per binary.
#![allow(dead_code)]

use grammaire_server::config::Config;
use grammaire_server::{AppState, db};
use sqlx::SqlitePool;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// A unique path in the system temp dir (no `tempfile` crate needed). The
/// counter matters: tests run in parallel and macOS clocks tick in
/// microseconds, so time alone can collide.
pub fn temp_path(label: &str) -> PathBuf {
    static SEQ: AtomicU64 = AtomicU64::new(0);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_nanos());
    let seq = SEQ.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "grammaire-{label}-{}-{nanos}-{seq}.sqlite",
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
    test_config_with(db_path, &[])
}

/// Test config plus extra env pairs (e.g. `EDITOR_PASSWORD`).
pub fn test_config_with(db_path: &std::path::Path, extra: &[(&str, &str)]) -> Config {
    let url = format!("sqlite:{}", db_path.display());
    let repo = std::env::temp_dir();
    let extra: Vec<(String, String)> = extra
        .iter()
        .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
        .collect();
    Config::from_lookup(
        |k| {
            // Extra pairs win, so a test can point REPO_ROOT elsewhere.
            if let Some((_, v)) = extra.iter().find(|(ek, _)| ek == k) {
                return Some(v.clone());
            }
            match k {
                "DATABASE_URL" => Some(url.clone()),
                "REPO_ROOT" => Some(repo.display().to_string()),
                "BCRYPT_ROUNDS" => Some("4".to_string()),
                _ => None,
            }
        },
        &repo,
    )
    .expect("config")
}

pub async fn app_with(extra: &[(&str, &str)]) -> (axum::Router, TempDb) {
    let db = fresh_db("router").await;
    let state = AppState::new(test_config_with(&db.path, extra), db.pool.clone());
    (grammaire_server::router(state), db)
}

pub async fn app() -> (axum::Router, TempDb) {
    let db = fresh_db("router").await;
    let state = AppState::new(test_config(&db.path), db.pool.clone());
    (grammaire_server::router(state), db)
}
