//! Typed configuration from environment variables (plus an optional `.env`
//! in the working directory). Invalid values stop the server at startup.
//!
//! | Variable | Default | Laravel equivalent |
//! |---|---|---|
//! | `DATABASE_URL` | required, `sqlite:<path>` | `DB_CONNECTION` + `DB_DATABASE` |
//! | `BIND_ADDR` | `127.0.0.1:8000` | `php artisan serve` default |
//! | `EDITOR_PASSWORD` | unset, empty or `0`: every login fails (PHP `empty()`) | `EDITOR_PASSWORD` |
//! | `FRONTEND_BASE_URL` | `http://localhost:5173/learning-french` | `FRONTEND_BASE_URL` |
//! | `FRONTEND_ORIGIN` | unset | `FRONTEND_ORIGIN` (extra CORS origin) |
//! | `SANCTUM_TOKEN_PREFIX` | empty | `SANCTUM_TOKEN_PREFIX` |
//! | `REPO_ROOT` | parent of the working directory | `realpath(base_path('..'))` |
//! | `LOG_FORMAT` | `pretty` (`json` for machines) | `LOG_CHANNEL` |
//! | `REQUEST_TIMEOUT_SECS` | `30` | PHP `max_execution_time` |
//! | `IDLE_TIMEOUT_SECS` | unset: never | none (local-server rule: exit when idle) |
//! | `LOGIN_RATE_LIMIT` | `5` login attempts per minute per IP; `0` turns it off | none (K5, plan Q9) |
//! | `BCRYPT_ROUNDS` | `12` (4 to 31) | `BCRYPT_ROUNDS` |
//!
//! To add a variable: add a field here, parse it in [`Config::from_lookup`],
//! document it in the table above and in `server/README.md`.

use std::fmt;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::Duration;

/// Origins the Laravel app allows in `config/cors.php` (before `FRONTEND_ORIGIN`).
pub const DEFAULT_ORIGINS: [&str; 3] = [
    "http://localhost:5173",
    "http://127.0.0.1:5173",
    "http://localhost:4173",
];

/// A string that never appears in logs or `Debug` output.
#[derive(Clone, PartialEq, Eq)]
pub struct Secret(String);

impl Secret {
    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Secret(***)")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LogFormat {
    Pretty,
    Json,
}

#[derive(Clone, Debug)]
pub struct Config {
    pub bind_addr: SocketAddr,
    pub database_url: String,
    /// `None` when unset or empty: Laravel then rejects every login.
    pub editor_password: Option<Secret>,
    pub frontend_base_url: String,
    pub allowed_origins: Vec<String>,
    pub sanctum_token_prefix: String,
    pub repo_root: PathBuf,
    pub log_format: LogFormat,
    pub request_timeout: Duration,
    pub idle_timeout: Option<Duration>,
    /// Login attempts per minute per client IP; 0 = no limit.
    pub login_rate_limit: u32,
    /// Cost for the bcrypt hash stored when the editor user is created.
    pub bcrypt_rounds: u32,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ConfigError {
    #[error("{0} is required")]
    Missing(&'static str),
    #[error("{name} is invalid: {reason}")]
    Invalid { name: &'static str, reason: String },
}

fn invalid(name: &'static str, reason: impl Into<String>) -> ConfigError {
    ConfigError::Invalid {
        name,
        reason: reason.into(),
    }
}

fn positive_secs(name: &'static str, raw: &str) -> Result<Duration, ConfigError> {
    match raw.trim().parse::<u64>() {
        Ok(0) | Err(_) => Err(invalid(
            name,
            format!("expected a positive integer, got {raw:?}"),
        )),
        Ok(n) => Ok(Duration::from_secs(n)),
    }
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let cwd = std::env::current_dir().map_err(|e| {
            invalid(
                "REPO_ROOT",
                format!("cannot read the working directory: {e}"),
            )
        })?;
        Self::from_lookup(|k| std::env::var(k).ok(), &cwd)
    }

    /// Build from any key lookup. `cwd` is used for the `REPO_ROOT` default.
    pub fn from_lookup(
        get: impl Fn(&str) -> Option<String>,
        cwd: &std::path::Path,
    ) -> Result<Self, ConfigError> {
        let non_empty = |k: &str| get(k).filter(|v| !v.is_empty());

        let database_url = non_empty("DATABASE_URL").ok_or(ConfigError::Missing("DATABASE_URL"))?;
        if !database_url.starts_with("sqlite:") {
            return Err(invalid("DATABASE_URL", "only sqlite: URLs are supported"));
        }

        let bind_raw = non_empty("BIND_ADDR").unwrap_or_else(|| "127.0.0.1:8000".to_string());
        let bind_addr = bind_raw
            .parse()
            .map_err(|e| invalid("BIND_ADDR", format!("{bind_raw:?}: {e}")))?;

        let frontend_base_url = non_empty("FRONTEND_BASE_URL")
            .unwrap_or_else(|| "http://localhost:5173/learning-french".to_string());

        let mut allowed_origins: Vec<String> =
            DEFAULT_ORIGINS.iter().map(|o| (*o).to_string()).collect();
        if let Some(extra) = non_empty("FRONTEND_ORIGIN") {
            allowed_origins.push(extra);
        }

        let repo_root = match non_empty("REPO_ROOT") {
            Some(p) => PathBuf::from(p),
            None => cwd
                .parent()
                .ok_or_else(|| {
                    invalid(
                        "REPO_ROOT",
                        "working directory has no parent; set REPO_ROOT",
                    )
                })?
                .to_path_buf(),
        };
        if !repo_root.is_dir() {
            return Err(invalid(
                "REPO_ROOT",
                format!("{} is not a directory", repo_root.display()),
            ));
        }

        let log_format = match non_empty("LOG_FORMAT").as_deref() {
            None | Some("pretty") => LogFormat::Pretty,
            Some("json") => LogFormat::Json,
            Some(other) => {
                return Err(invalid(
                    "LOG_FORMAT",
                    format!("expected pretty or json, got {other:?}"),
                ));
            }
        };

        let request_timeout = match non_empty("REQUEST_TIMEOUT_SECS") {
            Some(raw) => positive_secs("REQUEST_TIMEOUT_SECS", &raw)?,
            None => Duration::from_secs(30),
        };
        let idle_timeout = non_empty("IDLE_TIMEOUT_SECS")
            .map(|raw| positive_secs("IDLE_TIMEOUT_SECS", &raw))
            .transpose()?;

        let login_rate_limit = match non_empty("LOGIN_RATE_LIMIT") {
            Some(raw) => raw.trim().parse::<u32>().map_err(|_| {
                invalid(
                    "LOGIN_RATE_LIMIT",
                    format!("expected a non-negative integer, got {raw:?}"),
                )
            })?,
            None => 5,
        };
        let bcrypt_rounds = match non_empty("BCRYPT_ROUNDS") {
            Some(raw) => match raw.trim().parse::<u32>() {
                Ok(n) if (4..=31).contains(&n) => n,
                _ => {
                    return Err(invalid(
                        "BCRYPT_ROUNDS",
                        format!("expected 4 to 31, got {raw:?}"),
                    ));
                }
            },
            None => 12,
        };

        Ok(Config {
            bind_addr,
            database_url,
            // Laravel: `if (empty($expected) || ...)`, and PHP's empty("0") is true.
            editor_password: non_empty("EDITOR_PASSWORD")
                .filter(|p| p != "0")
                .map(Secret),
            frontend_base_url,
            allowed_origins,
            sanctum_token_prefix: get("SANCTUM_TOKEN_PREFIX").unwrap_or_default(),
            repo_root,
            log_format,
            request_timeout,
            idle_timeout,
            login_rate_limit,
            bcrypt_rounds,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn build(pairs: &[(&str, &str)]) -> Result<Config, ConfigError> {
        let map: HashMap<String, String> = pairs
            .iter()
            .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
            .collect();
        let cwd = std::env::temp_dir().join("x");
        Config::from_lookup(|k| map.get(k).cloned(), &cwd)
    }

    #[test]
    fn defaults() {
        let c = build(&[("DATABASE_URL", "sqlite:/tmp/x.sqlite")]).unwrap();
        assert_eq!(c.bind_addr.to_string(), "127.0.0.1:8000");
        assert_eq!(c.editor_password, None);
        assert_eq!(c.frontend_base_url, "http://localhost:5173/learning-french");
        assert_eq!(c.allowed_origins, DEFAULT_ORIGINS);
        assert_eq!(c.sanctum_token_prefix, "");
        assert_eq!(c.repo_root, std::env::temp_dir());
        assert_eq!(c.log_format, LogFormat::Pretty);
        assert_eq!(c.request_timeout, Duration::from_secs(30));
        assert_eq!(c.idle_timeout, None);
        assert_eq!(c.login_rate_limit, 5);
        assert_eq!(c.bcrypt_rounds, 12);
    }

    #[test]
    fn php_empty_password_zero_is_unset() {
        let c = build(&[("DATABASE_URL", "sqlite:x"), ("EDITOR_PASSWORD", "0")]).unwrap();
        assert_eq!(c.editor_password, None);
        let c = build(&[("DATABASE_URL", "sqlite:x"), ("LOGIN_RATE_LIMIT", "0")]).unwrap();
        assert_eq!(c.login_rate_limit, 0);
    }

    #[test]
    fn database_url_required_and_sqlite_only() {
        assert_eq!(
            build(&[]).unwrap_err(),
            ConfigError::Missing("DATABASE_URL")
        );
        assert!(matches!(
            build(&[("DATABASE_URL", "mysql://x")]),
            Err(ConfigError::Invalid {
                name: "DATABASE_URL",
                ..
            })
        ));
    }

    #[test]
    fn empty_password_means_unset_and_is_not_logged() {
        let c = build(&[("DATABASE_URL", "sqlite:x"), ("EDITOR_PASSWORD", "")]).unwrap();
        assert_eq!(c.editor_password, None);
        let c = build(&[("DATABASE_URL", "sqlite:x"), ("EDITOR_PASSWORD", "hunter2")]).unwrap();
        assert_eq!(
            c.editor_password.as_ref().map(Secret::expose),
            Some("hunter2")
        );
        assert!(!format!("{c:?}").contains("hunter2"));
    }

    #[test]
    fn extra_origin_and_bad_values() {
        let c = build(&[
            ("DATABASE_URL", "sqlite:x"),
            ("FRONTEND_ORIGIN", "https://a.example"),
        ])
        .unwrap();
        assert_eq!(
            c.allowed_origins.last().map(String::as_str),
            Some("https://a.example")
        );
        for (k, v) in [
            ("BIND_ADDR", "nope"),
            ("LOG_FORMAT", "xml"),
            ("REQUEST_TIMEOUT_SECS", "0"),
            ("IDLE_TIMEOUT_SECS", "-1"),
            ("REPO_ROOT", "/definitely/not/here"),
            ("LOGIN_RATE_LIMIT", "-1"),
            ("BCRYPT_ROUNDS", "3"),
            ("BCRYPT_ROUNDS", "32"),
        ] {
            assert!(
                matches!(
                    build(&[("DATABASE_URL", "sqlite:x"), (k, v)]),
                    Err(ConfigError::Invalid { .. })
                ),
                "{k}={v}"
            );
        }
    }
}
