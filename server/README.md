# server/: Rust port of the backoffice API

This folder is a Cargo workspace for the 1:1 Rust port of `backend/`
(Laravel). The plan, decisions and status are in
[`docs/migration/`](../docs/migration/MIGRATION_PLAN.md).

| Path | What it is |
|---|---|
| `app/` | `grammaire-server`, the Axum server. Ported so far: auth (login, logout, Sanctum tokens, login rate limit), uploads (`uploads.rs`), all chapter routes (`chapters.rs`) with Laravel's validator (`validation.rs`, `php.rs`), and Laravel's path handling (`paths.rs`). Export returns 501 to signed-in users until slice 3.5 lands. |
| `app/migrations/` | Laravel's exact DDL as a sqlx baseline, for fresh and test databases **only**. The server never migrates the real file. |
| `contract/` | Black-box HTTP parity suite. It runs unchanged against Laravel and Rust, with a `TARGET` switch for the differences listed in `docs/migration/KNOWN_ISSUES.md`. |
| `contract/run.sh` | Builds a temp sandbox (copy of `backend/`, `src/chapters.json`, an empty `public/uploads/`, a fresh SQLite made by Laravel's `migrate --seed`), starts one server on `127.0.0.1:8791` with a 600 s hard deadline, then runs the suite. It never touches the real `src/` or `public/`. |
| `contract/run-laravel.sh`, `run-rust.sh` | `run.sh laravel` and `run.sh rust` |
| `contract/ported.txt` | Contract tests the Rust app must pass so far |
| `scripts/osv-audit.py` | OSV advisory check of `Cargo.lock`, plus the banned-crate list. Run it after every lockfile change. |
| `scripts/sqlx-prepare.sh` | Regenerate or `--check` the offline query data in `app/.sqlx` |

You need Rust (`. "$HOME/.cargo/env"`). The contract runs also need PHP,
`backend/vendor` (`composer install` in `backend/`), `sqlite3`, `rsync` and
`curl`.

## Run the server

```bash
cd server
DATABASE_URL=sqlite:/path/to/laravel.sqlite EDITOR_PASSWORD=change-me cargo run -p grammaire-server
```

Point `DATABASE_URL` at a database made by Laravel's migrations. Don't use
`backend/database/database.sqlite`: spec/SPEC.md §1 marks it damaged, and
editing through the backoffice is on hold until the port is done.

All settings, their defaults and their Laravel equivalents are in the
header of `app/src/config.rs`. A `.env` in the working directory is read if
it exists. The server refuses to start if `DATABASE_URL` is missing, if the
file doesn't exist, or if the Laravel tables are missing. It never creates
or migrates the database. Ctrl-C, SIGTERM or `IDLE_TIMEOUT_SECS` shut it
down gracefully.

## Tests

```bash
cd server && cargo test -p grammaire-server && cargo test -p contract --lib
```

This runs the app's unit and in-process tests and the contract helpers' own
tests (no server needed). Don't use `--workspace` with `--test auth`: the
contract crate also has an `auth` test, and it needs a running server.

Contract suite, whole, against Laravel:

```bash
server/contract/run-laravel.sh
```

Contract suite against Rust, limited to the tests in `ported.txt`:

```bash
server/contract/run-rust.sh
```

Contract suite, whole, against Rust (shows what is left to port):

```bash
server/contract/run-rust.sh --all
```

Other arguments go to the test harness as a name filter. `-- CMD` runs any
command against the sandbox instead; it gets `BASE_URL`, `EDITOR_PASSWORD`,
`TARGET`, `SANDBOX_ROOT` and `CONTRACT_DB`. Set `KEEP_SANDBOX=1` to keep the
temp dir, and `CONTRACT_PORT` or `CONTRACT_DEADLINE` to change the port or
the deadline.

### sqlx offline query data

App queries use `sqlx::query!`, which is checked against the schema at
compile time. `server/.cargo/config.toml` sets `SQLX_OFFLINE=true`, so
builds read `app/.sqlx` and never connect to a database. That only applies
to cargo runs from inside `server/`. From anywhere else, export
`SQLX_OFFLINE=true` first, as `contract/run.sh` does, or a `DATABASE_URL`
in your environment makes the compiler open that database. After adding or
changing a query, regenerate the data and commit `app/.sqlx`:

```bash
server/scripts/sqlx-prepare.sh
```

To fail when the data is stale (for CI):

```bash
server/scripts/sqlx-prepare.sh --check
```

The script builds a throwaway database from `app/migrations/`, Laravel's
exact schema, and compiles with `SQLX_OFFLINE_DIR` set, so `sqlx-cli` isn't
needed.

### Regenerate the schema fixture

Do this only after a change under `backend/database/migrations/`:

```bash
server/contract/run.sh laravel -- sh -c 'sqlite3 "$CONTRACT_DB" "select type||char(9)||name||char(9)||tbl_name||char(9)||ifnull(sql,\"\") from sqlite_master where name not in (\"migrations\",\"sqlite_sequence\",\"_sqlx_migrations\") and tbl_name not in (\"migrations\") order by rowid"' > server/app/tests/fixtures/laravel-schema.tsv
```

Then update `app/migrations/` to match. Schema changes also need a `.sql`
file for the user to apply and their approval (brief, rule 2).

## Writing contract tests

- Use `Ctx::from_env()`, `c.login()`, and `c.get/post/put/delete(path)…send()`.
- Use `Resp::expect(status)` for JSON bodies and `assert_validation` for 422 bodies.
  The 422 message suffix ("(and N more errors)") is built by `validation_body`.
- Compare key order explicitly with `keys()`. `serde_json` map equality
  ignores order.
- A behaviour that differs on purpose needs a K-number in `KNOWN_ISSUES.md`
  and `c.expect_by_target("K#", laravel, rust)`. No other per-target branches.
- Tests run serially (`--test-threads=1`) against one shared database.
  Delete what you create, and never edit seeded chapters. Work on copies, as
  `round_trip_of_every_seed_chapter` does.
- Use `c.sqlite()` only for states the API cannot create, such as expired
  tokens or a deleted user.
- When a slice lands, add its test names to `contract/ported.txt`.

## Dependencies

Every crate is vetted before it's added (MIGRATION_PLAN.md, "Crates"), and
exact versions are pinned in `Cargo.toml` and `Cargo.lock`. After any
lockfile change, run:

```bash
python3 -I server/scripts/osv-audit.py server/Cargo.lock
```
