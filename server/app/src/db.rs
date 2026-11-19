//! SQLite pool against the **existing** Laravel database file.
//!
//! - Never creates the file and never migrates it. The baseline in
//!   `migrations/` is only for fresh and test databases (`fresh_for_tests`).
//! - Leaves `journal_mode` unchanged (sqlx's default), so the file stays
//!   readable by Laravel for rollback. `foreign_keys` is on, as in Laravel.
//! - Phase 2 queries use sqlx's runtime API with bound parameters. Slice 3.1
//!   switches app queries to compile-time-checked `query!` macros, once
//!   `sqlx-cli` is vetted for `cargo sqlx prepare`.

use anyhow::Context;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::str::FromStr;
use std::time::Duration;

/// Tables the API reads and writes. Startup fails if any is missing.
pub const REQUIRED_TABLES: [&str; 4] = ["chapters", "uploads", "personal_access_tokens", "users"];

pub async fn connect(url: &str) -> anyhow::Result<SqlitePool> {
    let options = SqliteConnectOptions::from_str(url)
        .context("DATABASE_URL is not a valid sqlite URL")?
        .create_if_missing(false)
        .busy_timeout(Duration::from_secs(5));
    SqlitePoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect_with(options)
        .await
        .with_context(|| format!("cannot open the database ({url})"))
}

pub async fn verify_schema(pool: &SqlitePool) -> anyhow::Result<()> {
    for table in REQUIRED_TABLES {
        let found: Option<String> =
            sqlx::query_scalar("select name from sqlite_master where type = 'table' and name = ?")
                .bind(table)
                .fetch_optional(pool)
                .await
                .context("cannot read sqlite_master")?;
        if found.is_none() {
            anyhow::bail!("table `{table}` is missing: is DATABASE_URL the Laravel database?");
        }
    }
    Ok(())
}

pub async fn ping(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query("select 1").execute(pool).await.map(|_| ())
}

/// The Laravel baseline schema (`migrations/0001_laravel_baseline.sql`).
pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

/// Create a new database file with the baseline schema. Tests only: refuses
/// to touch an existing file.
pub async fn fresh_for_tests(path: &std::path::Path) -> anyhow::Result<SqlitePool> {
    anyhow::ensure!(!path.exists(), "{} already exists", path.display());
    let options = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await?;
    MIGRATOR.run(&pool).await?;
    Ok(pool)
}
