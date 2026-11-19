//! `grammaire-server`: loads config, opens the existing SQLite database,
//! serves the API, and shuts down gracefully on Ctrl-C, SIGTERM or idle
//! timeout. In-flight requests finish before the process exits.

use anyhow::Context;
use grammaire_server::config::{Config, LogFormat};
use grammaire_server::{AppState, db, idle, router};
use std::process::ExitCode;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("grammaire-server: {e:#}");
            ExitCode::FAILURE
        }
    }
}

fn load_dotenv() -> anyhow::Result<()> {
    match dotenvy::dotenv() {
        Ok(_) => Ok(()),
        Err(e) if e.not_found() => Ok(()),
        Err(e) => Err(e).context("cannot read .env"),
    }
}

fn init_tracing(format: LogFormat) {
    let filter = EnvFilter::try_from_env("RUST_LOG").unwrap_or_else(|_| EnvFilter::new("info"));
    let builder = tracing_subscriber::fmt().with_env_filter(filter);
    match format {
        LogFormat::Json => builder.json().init(),
        LogFormat::Pretty => builder.init(),
    }
}

async fn run() -> anyhow::Result<()> {
    load_dotenv()?;
    let config = Config::from_env()?;
    init_tracing(config.log_format);

    let pool = db::connect(&config.database_url).await?;
    db::verify_schema(&pool).await?;

    let addr = config.bind_addr;
    let idle_timeout = config.idle_timeout;
    let state = AppState::new(config, pool.clone());
    let activity = state.activity.clone();
    activity.touch();
    let app = router(state);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .with_context(|| format!("cannot listen on {addr}"))?;
    tracing::info!(%addr, "listening");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(activity, idle_timeout))
        .await
        .context("server error")?;

    pool.close().await;
    tracing::info!("stopped");
    Ok(())
}

async fn shutdown_signal(
    activity: std::sync::Arc<idle::Activity>,
    idle_timeout: Option<std::time::Duration>,
) {
    let ctrl_c = async {
        if tokio::signal::ctrl_c().await.is_err() {
            std::future::pending::<()>().await;
        }
    };
    #[cfg(unix)]
    let terminate = async {
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(mut s) => {
                s.recv().await;
            }
            Err(_) => std::future::pending::<()>().await,
        }
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    let idle = async {
        match idle_timeout {
            Some(t) => idle::wait_idle(activity, t).await,
            None => std::future::pending::<()>().await,
        }
    };
    tokio::select! {
        () = ctrl_c => tracing::info!("ctrl-c: shutting down"),
        () = terminate => tracing::info!("SIGTERM: shutting down"),
        () = idle => tracing::info!("idle timeout: shutting down"),
    }
}
