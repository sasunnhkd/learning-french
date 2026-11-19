# server/: Rust port of the backoffice API

This folder is a Cargo workspace for the 1:1 Rust port of `backend/`
(Laravel). The plan, decisions and status are in
[`docs/migration/`](../docs/migration/MIGRATION_PLAN.md).

| Path | What it is |
|---|---|
| `contract/` | Black-box HTTP parity suite. It runs unchanged against Laravel and Rust, with a `TARGET` switch for the differences listed in `docs/migration/KNOWN_ISSUES.md`. |
| `contract/run-laravel.sh` | Builds a temp sandbox (copy of `backend/`, `src/chapters.json`, an empty `public/uploads/`, a fresh SQLite seeded from `src/chapters.json`), serves it on `127.0.0.1:8791` with a 600 s hard deadline, then runs the suite. It never touches the real `src/` or `public/`. |
| `scripts/osv-audit.py` | OSV advisory check of `Cargo.lock`, plus the banned-crate list. Run it after every lockfile change. |
| `app/` | The Axum server (Phase 2, not written yet). |

## Run the contract suite against Laravel

You need `backend/vendor` (`composer install` in `backend/`), PHP, `sqlite3`,
`rsync`, `curl`, and Rust (`. "$HOME/.cargo/env"`).

```bash
server/contract/run-laravel.sh
```

Extra arguments go to the test harness, for example a test name filter:

```bash
server/contract/run-laravel.sh upload
```

Run any command against the sandbox instead (it gets `BASE_URL`,
`EDITOR_PASSWORD`, `TARGET`, `SANDBOX_ROOT` and `CONTRACT_DB`):

```bash
server/contract/run-laravel.sh -- env
```

Set `KEEP_SANDBOX=1` to keep the temp dir, and `CONTRACT_PORT` or
`CONTRACT_DEADLINE` to change the port or the deadline.

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

## Dependencies

The suite's only crate is `serde_json`, pinned in `Cargo.lock`. Its HTTP
client is std-only, in `contract/src/lib.rs`. Vet every new crate under the
global package-safety rule, then run `scripts/osv-audit.py`.
