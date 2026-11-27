# Migration plan: Laravel API → Rust (Axum)

Status: **Phase 6 built (2026-11-26); the Debian 12 build and CI are
verified only after the first push.** Q3 decided: Debian 12 (2026-11-26).
Phase 5 accepted 2026-11-24 (`AUDIT_REPORT.md`). Install, publish, upgrade
and rollback: `CUTOVER.md`. Rust passes all 83
contract tests, Laravel is 83/83. The audit fixed one high-severity crash
(F1). Phase 6 follows the decisions. Q6–Q10, Q15 and Q16 were approved
on 2026-11-20. K14 and K15 were approved later the same day. K16 and K17
(slice 3.2) and K18 (slice 3.3) were approved as "accept" on 2026-11-22, and
K19 (slice 3.4) as a fix the same day (KNOWN_ISSUES). Q3, Q4, Q5 and Q11 still run on their defaults (local tool,
SQLite, keep tokens valid, systemd + tarball).

**Decided 2026-11-19 (Q1): option A.** Port today's 12-route local backoffice
API 1:1 to Rust. The "Under B" notes below are kept only as reference for a
later rebuild. They are not part of this plan.

Follow-on defaults from A. These hold unless you say otherwise:

- **Q3:** the server runs locally on this Mac, as `php artisan serve` does today.
- **Q4:** the database stays SQLite, with the existing schema.

Still open: `spec/SPEC.md` §6 and the project `CLAUDE.md` describe a Laravel
future. They are updated in Phase 6 to say that the backoffice API is Rust. The brief is
[INSTRUCTION.md](INSTRUCTION.md). The facts are in [INVENTORY.md](INVENTORY.md).

## 0. Decide first (blocking)

### Q1: what does "the Rust app" replace? (decided: A)

The repository already holds a **different** decision. `spec/SPEC.md` §6,
dated 2026-10-28, chose a fresh **Laravel** rebuild with:

- MariaDB and Redis on Debian 12;
- a new data model (`sections`, `chapter_versions`, roles, `chapter_runs`);
- a new `/api/v1` with learner accounts, publishing and GDPR export.

The project `CLAUDE.md` also mandates Laravel and Sanctum. A Rust migration
overrides both, so the user has to choose.

| | (A) 1:1 port of today's backoffice API | (B) Rust implements the SPEC §6 target |
|---|---|---|
| What gets built | The 12 routes in INVENTORY §1.1, same JSON, same SQLite schema | A new API: sections, versions, publish, learner auth (cookies + CSRF), runs, settings, GDPR |
| Parity oracle | Today's Laravel app (Phase 1 suite) | Nothing to compare against. SPEC §6 is the contract, so it is a **rebuild, not a migration**. |
| Phase 1 | Characterise Laravel | Write acceptance tests from SPEC §6 |
| Auth traps | Sanctum bearer compatibility | Cookie sessions, CSRF, password hashing (argon2), email verification. Mail and queue become real (lettre, a job runner). |
| DB | SQLite, existing schema | MariaDB (`sqlx` MySQL driver), new schema shipped as `.sql` files |
| Vite integration | None: the SPA stays a separate app | Single origin: nginx or Rust serves `dist/`, and the SPA moves to clean paths |
| Size | Small: about 12 handlers, one validator engine | Large: the whole of SPEC §6 phases 2 to 7 |
| Value | Low on its own. It ports a local tool with known data-loss bugs (D-03/D-04) that SPEC §1 says not to use. | It delivers the product SPEC §6 describes, in Rust instead of PHP |

**Recommendation:** decide between B and "stay with Laravel per SPEC §6".
Option A is cheap and safe, but on its own it only re-hosts a tool that is
being retired.

If A is chosen anyway (for example as a learning step, or to prove the
toolchain before B), the rest of this plan describes A. The plan is written
for A because that is what the brief asks for. Every phase below says what
changes under B.

### Other decisions the plan needs

| # | Question | Recommendation | Blocks |
|---|---|---|---|
| Q2 | D-03 (validation strips exercise fields) and D-04 (`TrimStrings`, `ConvertEmptyStringsToNull`): reproduce them for parity, or fix them as approved differences? | **Decided 2026-11-19: fix both.** Rust keeps every exercise field (D-03), and stops trimming and nulling exercise content (D-04). The scope of the D-04 fix is Q2a. The suite asserts Laravel's lossy value on the Laravel run and the exact value on the Rust run, and tags each case as an approved difference in `KNOWN_ISSUES.md`. | Phase 1 assertions, slice 3 |
| Q2a | Where does the D-04 fix apply? Inside `lesson` and `items` only, or also on the top-level fields? Fixing the top-level fields too changes two things: `titleEm: ""` would come back as `""` instead of `null` in index, show and export, and `title` and `section` would keep padding spaces. | **Decided 2026-11-19: everywhere.** Rust never trims a string and never turns one into `null`. The changes to top-level output (`titleEm: ""` stays `""`, `title` and `section` keep their padding) are approved differences in `KNOWN_ISSUES.md`. The `required` rule still fails whitespace-only strings, as Laravel's does. | Chapters-write contract cases |
| Q3 | Where does the Rust server run: on this Mac as a local tool, like `php artisan serve` today, or on the Debian 12 server? | A: local. B: Debian. | Phase 6, build target |
| Q4 | Which DB engine: SQLite (today) or MariaDB (SPEC §6, global "prod = MariaDB")? | A: SQLite. B: MariaDB. | Phase 2 |
| Q5 | Keep the 11 existing Sanctum tokens valid, or force re-login at cutover? | **Keep.** It costs one sha256 comparison, and rollback to Laravel then works both ways. | Slice 1 |
| Q6 | Framework routes. F1 `/` (Blade welcome), F3 `/sanctum/csrf-cookie`, F4/F5 `/storage/{path}`: drop? F2 `/up`: keep? | **Approved 2026-11-20.** Drop F1, F3, F4, F5. Keep `/up` (200, plain body). | Phase 2 |
| Q7 | K3 upload id mismatch: keep it, or use the filename ULID as the row id? | **Approved 2026-11-20.** **Fix for new rows.** Old rows keep working because delete goes through `path`. | Slice 4 |
| Q8 | K4 SVG uploads (XSS vector on the public origin): keep, drop SVG, or sanitise? | **Approved 2026-11-20.** Drop SVG, unless you use it. There are 0 uploads today. | Slice 4 |
| Q9 | K5: add a login rate limit, and optionally expire or prune tokens? | **Approved 2026-11-20.** Add a limit (5/min per IP), configurable through env and turned off by `run-rust.sh`, because the contract suite logs in about 60 times. Leave expiry as is. | Slice 1 |
| Q10 | K6: keep the absolute `path` and `written:false` + 200 in the export response? | **Approved 2026-11-20.** Keep `written`, return `path` relative to the repo. | Slice 5 |
| Q11 | Packaging: systemd + versioned tarball (global rules, SPEC §6), or Docker? | systemd + tarball. Docker only if you want it. | Phase 6 |
| Q12 | Contract suite language: Node `node:test` (Node 22 is already installed, no new deps), Rust integration tests, or Hurl (a new tool to vet)? | **Decided 2026-11-19: Rust integration tests.** The suite is the `server/contract` crate. It reads `BASE_URL` and uses a small std-only HTTP/1.1 client (`contract::Req`) instead of `reqwest`. That way its only dependency is `serde_json`. It is one suite for both apps, with a `TARGET=laravel|rust` switch. The switch changes only the cases tagged as approved differences in `KNOWN_ISSUES.md` (Q2). As a result, the toolchain install (Q14) moves to the start of Phase 1. | Phase 1 |
| Q13 | The repo is public, and `backend/`, `spec/`, the SPA changes and `CLAUDE.md` are all uncommitted. What goes into commits on `migration/rust`? | **Decided 2026-11-19: migration files only.** Phase commits contain only `docs/migration/` and `server/` (which holds the contract crate), after a secret scan of the staged diff. `backend/`, `spec/`, `CLAUDE.md` and the SPA changes stay uncommitted. Nothing is pushed without your OK. | Phase 0 commit |
| Q14 | Install the Rust toolchain (rustup) plus `cargo-audit`, `cargo-deny`, `sqlx-cli`, and later `oha`? Each one gets vetted under the package-safety rule first. | Yes, at the **start of Phase 1** (because Q12 chose Rust tests). **Phase 1 crates (vetted 2026-11-20):** only `serde_json` 1.0.154 with `preserve_order`. HTTP is hand-written on `std::net`, so `tokio`, `reqwest` and `tempfile` were not needed.

- Lock: 15 crates, from the dtolnay, BurntSushi, rust-lang, bluss/cuviper and orlp crates.io accounts.
- `serde_derive`, `syn`, `quote` and `proc-macro2` are locked through serde_core's `cfg(any())` version pin, but are never built.
- The build scripts only probe `rustc --version` and write to `OUT_DIR`.
- OSV check of `Cargo.lock`: no advisories, no banned crates. Re-run it with `python3 -I server/scripts/osv-audit.py server/Cargo.lock` until `cargo-audit` is installed. **Route decided 2026-11-19:** download `sh.rustup.rs` to the scratchpad,
read it, then run it with `--no-modify-path` so no shell profile is edited.
**Done 2026-11-19:**

- Installer: `rustup-init.sh`, sha256 `d890e2fe…048de6`.
- What the script does: it fetches `rustup-init` from
  `static.rust-lang.org` over HTTPS with TLS 1.2 or later. Its only base64
  blob is a LoongArch probe, which never runs on macOS.
- Installed: `rustc`/`cargo` 1.100.0 stable, `clippy`, `rustfmt`, minimal
  profile, `aarch64-apple-darwin`. Cargo lives in `~/.cargo/bin` and is not
  on `PATH`.
- No incident against rustup was found.
- `deny.toml` must ban the crates from the August 2026 crates.io compromise
  (RUSTSEC-2026-0260): `proc-macro1` (a typosquat), and `arrayref` 0.3.10,
  `internment` 0.8.7 and `append-only-vec` 0.1.9. | Phase 1 |
| Q15 | Upload size: keep Laravel's effective 2M cap (from PHP `upload_max_filesize`), or honour the 5120 KB rule? | **Approved 2026-11-20.** 5120 KB. That is what the rule says. | Slice 4 |
| Q16 | K11 (found in Phase 1): a guest request without `Accept: application/json` gets a 500 HTML page from Laravel. Should Rust answer 401 JSON instead? | **Approved 2026-11-20.** **Default: yes**, always 401 `{"message":"Unauthenticated."}`. The SPA always sends the header, so it sees no change. | Slice 1 |

## 1. Architecture (option A)

```
SPA (Vite, :5173, unchanged) ──fetch, Bearer──▶ grammaire-server (Axum, 127.0.0.1:8000)
                                                   ├─ SQLite (same file and schema as Laravel)
                                                   ├─ ../public/uploads/   (image files)
                                                   └─ ../src/chapters.json (export target)
```

- One binary, `grammaire-server`.
- `VITE_API_URL` keeps pointing at `:8000`, so the SPA doesn't change.
- **Paths become config.** `REPO_ROOT` defaults to the parent of the working
  directory, which matches Laravel's `realpath(base_path('..'))`. Uploads and
  export resolve under it. This also lets the contract suite use a sandbox.
- **Rollback in both directions.** Rust writes rows that Laravel reads
  unchanged: the same timestamp text, JSON text and Sanctum token format.
- The Rust server uses no `migrations` table. A sqlx baseline exists for
  fresh/test databases only, and is never run against an existing file
  without approval.

### Folder layout

As built through Phase 2. `(3.x)` marks modules that later slices add.

```
server/
  Cargo.toml  Cargo.lock      # workspace (app, contract); rust-version 1.94
  .gitignore                  # /target/
  README.md                   # how to run, how to write contract tests
  .cargo/config.toml          # SQLX_OFFLINE=true: macros never touch a live database
  scripts/osv-audit.py        # OSV advisory check of Cargo.lock + banned-crate list
  scripts/sqlx-prepare.sh     # regenerate / --check app/.sqlx from a throwaway baseline DB (no sqlx-cli)
  app/                        # package grammaire-server 0.1.0
    migrations/0001_laravel_baseline.sql  # Laravel's exact DDL; fresh/test DBs only
    src/
      main.rs                 # .env, config, tracing, pool, schema check, serve, graceful shutdown
      lib.rs                  # module list; re-exports AppState, router
      config.rs               # typed env (table in the file header); Secret hides EDITOR_PASSWORD
      error.rs                # AppError -> Laravel JSON bodies (401/404/405/413/500), laravel_path()
      http.rs                 # routes + middleware stack (order documented in the file header)
      cors.rs                 # Laravel HandleCors + php-cors, hand-written
      auth.rs                 # bearer_token() (Laravel parsing), AuthUser guard (fails closed until 3.1)
      db.rs                   # pool on the existing file, verify_schema, MIGRATOR, fresh_for_tests
      idle.rs                 # IDLE_TIMEOUT_SECS activity tracking
      input.rs                # request input like $request->all(): JSON/form/multipart, PHP a[] keys; no query string (K14)
      validation.rs           # 422 body, display names, required/string (3.1); rule engine: validate, validation_view (3.3)
      php.rs                  # PHP rules the validator needs: blank chars, trim, float strings, filter_var int, is_numeric, brick compare (3.3)
      limiter.rs              # login rate limit (K5/Q9), LOGIN_RATE_LIMIT
      uploads.rs              # R9 list (3.1); store/delete, sniff, new_ulid (3.4)
      chapters.rs             # R3-R8: reads (3.2), writes, chapter_rules, reorder (3.3)
      export.rs               # R12 export: Eloquent's query verbatim, php::json_pretty (3.5)
    .sqlx/                    # offline query data for sqlx macros (scripts/sqlx-prepare.sh)
    tests/
      auth.rs                 # in-process login/logout/guard/limiter checks
      router.rs               # in-process routing, error, CORS, 413, request-id checks
      schema_baseline.rs      # baseline == fixtures/laravel-schema.tsv
      fixtures/laravel-schema.tsv  # sqlite_master after Laravel's migrate (regen: server/README.md)
      common/mod.rs           # temp DBs, test config, app()
  contract/                   # HTTP parity suite (std-only client + serde_json), BASE_URL-driven (Q12)
    src/lib.rs                # Ctx (env), Req/Resp HTTP client, validation_body, php_pretty, crc32b, TINY_PNG
    tests/*.rs                # one file per route group, plus validation.rs
    fixtures/                 # tiny.jpg / tiny.gif / tiny.webp
    run.sh                    # sandbox + Laravel-made DB + server (laravel|rust) + suite
    run-laravel.sh run-rust.sh  # wrappers for run.sh
    ported.txt                # contract tests the Rust app must pass so far
```

### Crates

Each crate is vetted before it is added, and `Cargo.lock` pins every version.

**`grammaire-server`, vetted 2026-11-20** (versions are exact pins):

| Crate | Use |
|---|---|
| `tokio` 1.54.1 | Runtime, signals, timers |
| `axum` 0.8.9 | Router. Default features off: `http1`, `json`, `form`, `query`, `tokio`, `matched-path`, `original-uri` and `tracing` on. |
| `tower` 0.5.3 (`util`) | Service composition |
| `tower-http` 0.7.1 | `trace`, `request-id` and `timeout` features. **No `cors`**: CORS is hand-written in `cors.rs`, because Laravel's 204 preflights and `Vary` rules differ from tower-http's. |
| `sqlx` 0.9.1 | SQLite with the bundled library, `runtime-tokio`, `macros` and `migrate`. Default features off: no `load-extension`, no `any`, no `json`. |
| `serde` 1.0.229, `serde_json` 1.0.154 (`preserve_order`) | JSON with PHP key order |
| `thiserror` 2.0.21, `anyhow` 1.0.104 | Errors (`anyhow` only at the edges) |
| `tracing` 0.1.44, `tracing-subscriber` 0.3.23 | Logs (`fmt`, `env-filter`, `json`) |
| `dotenvy` 0.15.7 | Optional `.env` |

The vetting covered the whole tree:

- The lock has 187 crates, of which 147 are built.
- Owners were checked through the crates.io API. All are known ecosystem
  publishers: rust-lang, dtolnay, tokio-rs, hyperium, rustcrypto, servo,
  unicode-org/icu4x, the sqlx maintainers, smol-rs, and others.
- Build scripts:
  - Most only probe `rustc --version` or write to `OUT_DIR`.
  - `libsqlite3-sys` compiles the bundled SQLite with `cc`. Its bindgen path
    belongs to a feature that is not enabled.
- Proc macros: `sqlx-macros-core` runs `cargo metadata --no-deps` to find
  the workspace root. Nothing touches the network.
- `scripts/osv-audit.py` finds no advisories and no banned crates.

**Added in slice 3.1, vetted 2026-11-20:**

| Crate | Use |
|---|---|
| `sha2` 0.11.1 | Token hashes (already in the tree through sqlx) |
| `getrandom` 0.4.3 | CSPRNG for token entropy (already in the tree) |
| `form_urlencoded` 1.2.3 | Form bodies (already in the tree; 1.2.2 until the Phase 5 `url` fix, D1) |
| `bcrypt` 0.19.3 | `$2y$` hash for a newly created editor user (Keats) |
| axum `multipart` feature (`multer` 3.1.0) | Multipart input: login fields now, uploads in 3.4 |

The 13 new crates in the tree come from known publishers:

| Crates | Publisher |
|---|---|
| `bcrypt` | Keats |
| `blowfish`, `cipher`, `inout`, `zeroize`, `const-oid` | RustCrypto |
| `subtle` | dalek |
| `multer` | rwf2 |
| `encoding_rs`, `multiversion_no_op` | hsivonen |
| `simdutf8` | rusticstuff |
| `core_detect` | thomcc |
| `base64` | marshallpierce |
| `byteorder` | BurntSushi |

`multiversion_no_op` is a proc macro that is only four months old, so it was
read in full: a 3-line pass-through attribute. The only new build scripts
are two 5-line version probes. OSV is clean.

No `hex`, `subtle` or `rand` dependency of our own: hex encoding, the
constant-time compare and token sampling are a few lines each in `auth.rs`,
and each is unit-tested.

**Planned for later slices, not needed after all.** Each was replaced by a
few lines of own, unit-tested code, so no new crate had to be vetted:

| Crate once planned | Replaced by |
|---|---|
| `ulid`, `time` | `uploads::new_ulid` (48-bit ms + 80 bits from `getrandom`); timestamps from SQLite's `datetime('now')` (3.4) |
| `regex` | `validation::Pattern::Slug`, a byte check (3.3) |
| `infer` plus an own SVG check | `uploads::sniff`: the libmagic 5.46 rules for the four accepted types; SVG is refused (K4), so it needs no detection (3.4) |

Not needed under A: `bcrypt`/`argon2` (no password is verified), sessions,
`lettre`, `apalis`, templates.

## 2. Phases and commits

Each phase ends with:

- the tests passing, with the command output shown;
- the docs updated;
- an advisor review of the diff;
- a secret scan of the staged files;
- one commit on `migration/rust`.

| Phase | Work | Done when | Under B |
|---|---|---|---|
| 0 | INSTRUCTION, INVENTORY, this plan | Q1, Q2, Q2a, Q12, Q13 and Q14 are decided (2026-11-19). The other questions run on the defaults stated here until you override them. | same |
| 1 | Toolchain install (Q14, each tool and crate vetted). A `server/` Cargo workspace holding only the `contract` crate. The suite runs against sandboxed Laravel. It covers every R1–R12 route: guest, bad or deleted token, happy path, each validation rule with its message, 404, 405, 413, CORS preflight. Fixtures come from `src/chapters.json` (26 chapters plus the placeholder). Deliverable: `KNOWN_ISSUES.md` (K1–K13). | **Done 2026-11-20:** 57/57 green against Laravel in about 20 s. With `TARGET=rust`, exactly the 10 tagged tests fail (KNOWN_ISSUES; corrected from 11). | Acceptance tests from SPEC §6 instead |
| 2 | `server/app` crate skeleton, config, tracing, request ids, `/up`, error type, CORS (exact origin list), post-size limit, timeout, idle exit, SQLite pool on the existing file, sqlx baseline proved against Laravel's `sqlite_master`. All 12 routes are registered behind a guard that fails closed. `run.sh` with a `rust` target and `ported.txt`. | **Done 2026-11-20:** 20 app tests (10 unit, 9 router, 1 schema baseline) plus the 4 contract helper tests. 9 contract tests pass against Rust, including the 9 MB 413 over a real socket (5/5 repeats). Laravel is still 57/57, and the `TARGET=rust` negative run still fails exactly the 10 tagged tests (corrected from 11). `cargo sqlx prepare` moves to 3.1, the first `query!` macro: Phase 2 has only two runtime queries, both with bound parameters. | MariaDB pool, new schema as `.sql` |
| 3.1 | Auth slice: Sanctum token module (vectors from real Laravel-made tokens), guard extractor, `last_used_at`, login and logout. **Notes from the Phase 2 review:** <ul><li>Read request input with one extractor that turns a JSON **or** form body into a `serde_json::Value`, and treats a body that won't parse as `{}`. axum's `Json`/`Form` rejections are plain-text 400/415/422 errors, and they would break `login_malformed_json` and `login_accepts_form_encoding`.</li><li>Before writing that extractor, capture whether Laravel merges query-string input: does `POST /api/auth/login?password=…` with an empty body log in? If it does, that is a new keep-or-fix item (K14) for you. The recommendation would be to fix it, because passwords would land in access logs.</li><li>Add captured contract tests for GET, PUT and DELETE on `/api/chapters/reorder`. They should get 404 `… [App\\Models\\Chapter] reorder`, which pins the `{id}` delegation in `http.rs`.</li></ul> | **Done 2026-11-20:** Rust passes 30/30 ported contract tests (21 new auth tests), including tokens crossing both ways: Laravel-minted tokens work on Rust, and Rust-minted tokens pass Laravel's `findToken`. There are 40 app tests: 25 unit, 5 auth, 9 router, 1 schema. Laravel is 64/64, and the negative run fails exactly 12 tagged tests. The limiter was checked over a real socket: the third login got 429 with `Retry-After`. Queries are compile-time checked with `query!`. Offline data comes from `scripts/sqlx-prepare.sh` instead of `sqlx-cli`, so there is one tool fewer to install. `GET /api/uploads` (R9) was pulled forward as the authenticated probe the auth tests use. New items: K14 (query-string input) and K15 (trailing header space), both approved on 2026-11-20. | Cookie auth, CSRF, register, verify, reset |
| 3.2 | Chapters read: R3, R4 (ordering, shape, null `lesson`, `items` default `[]`) | **Done 2026-11-22:** Rust passes 38/38 ported contract tests (8 new). `index` runs Eloquent's SQL verbatim, and the contract test now pins the exact id sequence. `show` returns the stored payload JSON as it is (K2, plus the new K16 numbers and K17 nesting depth, both approved as "accept" on 2026-11-22). `GET /api/chapters/reorder` is `show` with id "reorder". PUT and DELETE follow in 3.3, so `reorder_path_falls_back_to_id_routes` waits until then. **Phase 2 gap fixed** (`paths.rs`, commit `7a7399e`): Rust now routes on the decoded path without trailing slashes and answers 400 "Malformed URL." like Laravel; before, `/api/auth/login/` was a 404. Tests: 49 app tests (31 unit, 5 auth, 10 router, 2 chapters, 1 schema). Laravel is 67/67, and the negative run fails exactly 14 tagged tests. | Public read endpoints + ETag |
| 3.3 | Chapters write: R5–R8, the validation engine, `position` logic, transaction for reorder, Q2 outcome | **Done 2026-11-22:** Rust passes 65/65 ported contract tests (27 new: 26 write/validation tests, incl. `reorder_path_falls_back_to_id_routes`, and 1 login test). The round trip of the 26 seed chapters is now compared as JSON text, so key order counts. The validator is a literal port of Laravel's rule expansion and checks (`validation.rs`, `php.rs`), checked against 38 error bodies captured from Laravel. Store keeps the raw input (K1, K2). Reorder binds positions as text so SQLite stores what PDO does (K8, object keys). Such a text `position` reads fine as a plain column, but a typed or computed read fails, so `max(position)` and update's read use `cast(… as integer)` (PHP's `(int)`); export (3.5) must too. New: **K18**, approved as "accept" on 2026-11-22: Rust validates padded strings untrimmed. **Slice 3.1 gap fixed:** login's `required` used Rust's Unicode `trim`, so a password of a no-break space or form feed got "required" instead of Laravel's "Invalid password." (`login_unicode_blank_password_is_checked`). Tests: 61 app tests (39 unit, 5 auth, 10 router, 6 chapters, 1 schema). Laravel is 73/73, and the negative run fails exactly 15 tagged tests. | Drafts, versions, publish |
| 3.4 | Uploads: R9–R11, sniffing, path guard, Q7/Q8/Q15 outcomes | **Done 2026-11-22:** Rust passes 77/77 ported contract tests (12 new: the 6 upload tests from Phase 1, 5 new shared ones and the K19 test). The edge cases were probed on Laravel first: which multipart part counts as `file` (a text field wins, the last file part wins, an empty file name is no file, `file[]` is an array), the libmagic 5.46 signatures for the four types, PHP-extension names refused (`x.php`), `original_name` after the last `/` or `\`, and delete when the file is already gone. **Multipart gap fixed** (found in review): multer matches `name`/`filename` case-sensitively and drops non-UTF-8 file names, while PHP matches in any case and keeps the bytes; `input::php_disposition` now reads the header with PHP's rules (`content_disposition_parsing`). New: **K19**, approved as a fix on 2026-11-22: Laravel's upload list breaks (500) after a non-UTF-8 file name; Rust stores the name with U+FFFD. K3 (id = file ULID), K4 (no SVG) and K7 (5120 KB) are in. No new crates: the ULID and the sniffing are own code. The file is written with `create_new` and removed again if the row insert fails (Laravel would leave it). Tests: 68 app tests (45 unit, 5 auth, 6 chapters, 10 router, 1 uploads, 1 schema). Laravel is 79/79, and the negative run fails exactly 16 tagged tests. | same |
| 3.5 | Export: R12, byte-identical file against the Laravel output | **Done 2026-11-23:** `cmp` of the files Laravel and Rust export from the same seed = 0 (342,024 bytes); rerun with `server/contract/export-cmp.sh`. Rust passes 80/80 contract tests (3 new: the 2 export tests from Phase 1 and `export_escaping_and_text_position`, which checks control characters, U+2028/U+2029, `/`, é, emoji and DEL against PHP's bytes, plus a text `position`). Export runs Eloquent's `select *` query verbatim as a runtime query and reads `number`/`position` unchecked, so a text position reads as 0 (the 3.3 trap). The file is written in place like `@file_put_contents`; `written: false` on failure (K6 kept), `path` is relative (K6 fixed). The 501 stub (`protected_stub`, `AppError::NotImplemented`) is gone. Tests: 70 app tests (46 unit, 5 auth, 6 chapters, 1 export, 10 router, 1 uploads, 1 schema). Laravel is 80/80, and the negative run fails exactly 16 tagged tests. | Replaced by publish |
| 4 | Background work: **none to port** (INVENTORY §5). Checkpoint commit only, adding the parity matrix rows. | Matrix lists every route and command | **Done 2026-11-23:** all 17 routes from `route:list` are in §4, plus rows for the shared behaviour (auth guard, input, validation, CORS, post size, errors, rate limit) and the operational pieces (commands, schema, seeding). Nothing to port: no jobs, schedule, listeners or mail exist. | Mail queue, scheduler |
| 5 | Audit loop: fmt, clippy (pedantic where sensible), tests, the full contract suite, `cargo audit`, `cargo deny`, `sqlx prepare --check`, a route-enumeration auth test, OWASP pass, `oha` p50/p95/p99 against Laravel, `npm run build`, an admin smoke test in the browser pane. Deliverable: `AUDIT_REPORT.md`. | All clean, or the findings approved. **Audit complete 2026-11-23 (`AUDIT_REPORT.md`); accepted 2026-11-24:** all gates green. F1 (unauthenticated crash through deeply nested form names) fixed by copying PHP's input limits; F2, F5 and F9 fixed. New tests: `every_api_route_is_guarded_like_laravel`, `form_input_limits_like_php`, `file_part_cap_like_php`. No new tools: OSV instead of `cargo audit`, `cargo metadata` instead of `cargo deny`, `ab` instead of `oha` (`server/contract/bench.sh`). Decided: F3 → Phase 6 nginx, D1 → fixed (`url` 2.5.9, `form_urlencoded` 1.2.3), P1 → keep the journal, A → no. | same, larger |
| 6 | CI workflow (fmt, clippy, test, audit, deny, sqlx check, contract suite). Packaging per Q3/Q11. `CUTOVER.md`. Swap README, project `CLAUDE.md` and `spec/SPEC.md` §6. From the Phase 5 audit: F3 (nginx `limit_req` on login; trust `X-Real-IP` only from loopback) and F8 (nginx `nosniff`, CSP on `/uploads/`, header timeouts, whether `/up` is public; sanitise `x-request-id`). | CI green. Rollback tested once (Laravel reads the Rust-written DB). **Built 2026-11-26.** Q3 = Debian 12. Done: `.github/workflows/server.yml` (fmt, clippy, tests, sqlx check, OSV audit, release tarball in `rust:1.100.0-bookworm` with a glibc ≤ 2.36 check, `nginx -t`, `systemd-analyze verify`; actions pinned to commits; the contract suite stays local because `backend/` isn't in git); `server/packaging/` (release script, systemd unit with hardening, nginx site with F3/F8, env example); version 0.2.0 with `--version`; `scripts/initial-db-sql.sh`; F3 (`X-Real-IP` from loopback) and F8 (request-id filter, bind warning) in code; `CUTOVER.md`; `DOC_SWAP.md` (applied at cutover). Rollback **tested** with `server/contract/rollback-check.sh`: passed. **Not yet verified:** CI and the Linux tarball, which run only after a push. | Debian systemd, nginx, tarball, SQL files |

**Cutover under A:**

- the editor runs `server` instead of `php artisan serve`;
- there is no queue to drain;
- tokens stay valid (Q5);
- rollback means starting artisan again on the same DB file.

Strangler or route-by-route routing isn't worth it for 12 routes and one user.

## 3. Risks

| Risk | Mitigation |
|---|---|
| Validation messages drift from Laravel's (the SPA shows `message` verbatim) | Phase 1 records every rule's message from the real app, and the Rust tests compare strings exactly |
| JSON round-trip differs: key order, `{}` → `[]`, number types, Unicode escaping | `preserve_order`. Tests compare parsed JSON for responses and bytes for the export file. Each PHP quirk is decided in KNOWN_ISSUES. |
| The contract suite against Laravel overwrites the real `src/chapters.json` or `public/uploads/` | It always runs from a temp sandbox copy, and the script refuses to run inside the repo |
| The damaged `database.sqlite` gets used as a fixture | Fixtures are seeded only from `src/chapters.json`. The suite never opens `backend/database/database.sqlite`. |
| A crate supply-chain issue | Package-safety rule: vet before each add, then `cargo audit`/`deny` in CI, with a pinned lockfile |
| Scope creep from A to B without a decision | Q1 is answered before Phase 1 |
| Publishing secrets: the repo is public | A staged-diff scan before every commit. `.env` values are never copied into docs or tests. Test passwords are generated, never real ones. |
| The SPEC §6 and project `CLAUDE.md` contradictions linger | Phase 6 updates them. Under B, update them in Phase 0 after the decision. |

## 4. Parity matrix (filled in Phases 1 to 5)

**Phase 4 checkpoint (2026-11-23).** `php artisan route:list --json` lists 17
routes: R1–R12, F1–F5. Each has a row below. Laravel has no jobs, scheduled
tasks, listeners, notifications, mail or broadcasts to port (INVENTORY §5),
and `AppServiceProvider` is empty. The rows after the routes cover the
behaviour every route shares and the operational pieces.

| Laravel | Rust handler | Contract test | Status |
|---|---|---|---|
| R1 `POST /api/auth/login` | `auth::login` | `server/contract/tests/auth.rs` | Laravel ✓, Rust ✓ |
| R2 `POST /api/auth/logout` | `auth::logout` | `server/contract/tests/auth.rs` | Laravel ✓, Rust ✓ |
| R3 `GET /api/chapters` | `chapters::index` | `server/contract/tests/chapters_read.rs` | Laravel ✓, Rust ✓ |
| R4 `GET /api/chapters/{id}` | `chapters::show` (and `show_reorder`) | `server/contract/tests/chapters_read.rs` | Laravel ✓, Rust ✓. Payload read as stored (K2, K16, K17). |
| R5 `POST /api/chapters` | `chapters::store` | `server/contract/tests/chapters_write.rs`, `validation.rs` | Laravel ✓, Rust ✓. Raw input stored (K1, K2); K18. |
| R6 `PUT /api/chapters/{id}` | `chapters::update` | `server/contract/tests/chapters_write.rs`, `validation.rs` | Laravel ✓, Rust ✓. Also `PUT /api/chapters/reorder` (`update_reorder`). |
| R7 `DELETE /api/chapters/{id}` | `chapters::destroy` | `server/contract/tests/chapters_write.rs` | Laravel ✓, Rust ✓. Also `DELETE /api/chapters/reorder` (`destroy_reorder`). |
| R8 `POST /api/chapters/reorder` | `chapters::reorder` | `server/contract/tests/chapters_write.rs` | Laravel ✓, Rust ✓. One transaction; object keys as positions (K8). |
| R9 `GET /api/uploads` | `uploads::index` | `server/contract/tests/uploads.rs`, auth probe | Laravel ✓, Rust ✓ |
| R10 `POST /api/uploads` | `uploads::store` | `server/contract/tests/uploads.rs` | Laravel ✓, Rust ✓. Sniffed type, K3, K4, K7, K19. |
| R11 `DELETE /api/uploads/{id}` | `uploads::destroy` | `server/contract/tests/uploads.rs` | Laravel ✓, Rust ✓. File found through `path`. |
| R12 `POST /api/export` | `export::run` | `server/contract/tests/export.rs` | Laravel ✓, Rust ✓. Same file bytes (K13); relative `path` (K6). |
| F2 `GET /up` | `http::up` | `server/contract/tests/framework.rs` | Laravel ✓, Rust ✓ |
| Path handling (all routes: Malformed URL, decoded-path routing, trailing slash) | `paths::middleware` | `framework::paths_are_decoded_like_laravel` | Laravel ✓, Rust ✓ |
| F1, F3, F4, F5 | dropped (Q6) | `server/contract/tests/framework.rs` asserts 404 | Laravel ✓, Rust ✓ (404, Q6) |
| Auth guard (Sanctum bearer tokens, `last_used_at`, 401 body) | `auth::AuthUser` | `auth.rs` (`protected_routes_reject_*`, `bearer_*`, `authenticated_request_sets_last_used_at`, token-crossing tests) | Laravel ✓, Rust ✓ (K11, K15) |
| Request input (`$request->all()`: JSON, form, multipart, `Content-Disposition`) | `input::Input`, `input::php_disposition` | `auth::login_input_formats`, `login_password_in_query_string`, `uploads::content_disposition_parsing` | Laravel ✓, Rust ✓ (K14, K19) |
| Validation engine (rule order, messages, blank strings) | `validation::validate`, `php.rs` | `validation.rs`, `chapters_write.rs` | Laravel ✓, Rust ✓ (K18) |
| CORS (`HandleCors`) | `cors::middleware` | `framework::cors_*` | Laravel ✓, Rust ✓ |
| Post size (`ValidatePostSize`, 8 MB → 413) | `http::limit_post_size` | `uploads::oversized_body_is_413` | Laravel ✓, Rust ✓ |
| Error bodies (404 route, 405 + `Allow`, 404 model, 500) | `error::AppError`, `http::route_not_found`, `method_not_allowed_body` | `framework::unknown_api_route_is_404_json`, `wrong_method_is_405_with_allow_header`, `show_unknown_chapter_is_404` | Laravel ✓, Rust ✓ |
| Login rate limit | `limiter.rs` | `server/app/tests/auth.rs` (off in contract runs) | Rust only (K5, Q9) |
| Jobs, scheduler, listeners, notifications, mail, broadcasting | none exist | — | N/A (Phase 4: nothing to port) |
| Artisan commands | only the scaffold `inspire`, plus package dev tooling (`dev`, `reload`, `dev:list`) | — | N/A |
| Schema (`php artisan migrate`) | `app/migrations/` baseline for fresh and test databases only; the server checks the real file with `db::verify_schema` and never migrates it | `server/app/tests/schema_baseline.rs` | Rust ✓. Schema changes ship as SQL files you apply. |
| Seeding (`ImportChaptersSeeder` from `chapters.json`) | not ported: the contract sandbox seeds with Laravel's own `migrate --seed` (`contract/run.sh`) | every contract test | N/A |
