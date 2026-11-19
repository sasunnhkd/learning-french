# Migration brief: Laravel API → Rust (refined for this repo)

This is the generic "Laravel + Vite → Rust + Vite" brief, rewritten with the
facts of this repository (Phase 0 discovery, 2026-11-19). Where the generic
brief assumed something this repo doesn't have, the line says **N/A** and
gives the evidence. Evidence and details: [INVENTORY.md](INVENTORY.md).
Plan and open questions: [MIGRATION_PLAN.md](MIGRATION_PLAN.md).

## Role

You are a senior Rust engineer porting this repository's Laravel JSON API to
Rust. Behaviour stays the same for the SPA and any API client unless the user
approves a change. Read the code. Don't guess.

## Context (facts, not placeholders)

| Item | Value |
|---|---|
| Backend | Laravel 13.35.0 (`backend/`), PHP `^8.3` (local CLI is 8.5.6), Sanctum 4.3.2, no other runtime packages |
| What the backend is | A **local-only authoring backoffice** with 12 JSON routes (R1–R12 in INVENTORY §1.1). It has never been committed, deployed or used in production. |
| Frontend | A separate Vite 6.4.5 + React 18 SPA at the repo root. `base: "/learning-french/"`, with hash routes (`#/admin`). The public site is static on GitHub Pages and **never calls the API**. Only the lazy-loaded admin (`src/admin/`) does. |
| How the SPA calls the API | `src/admin/api.js`. Base URL is `VITE_API_URL`, default `http://localhost:8000`. It sends `Authorization: Bearer <token>`, with the token kept in `localStorage["grammaire.admin.token"]`. It reads `message` from error bodies, and a 401 clears the token. |
| Database | SQLite (`backend/database/database.sqlite`). The local copy is **damaged** (spec/SPEC.md §1, D-03/D-04). The authoritative content is `src/chapters.json`, with `spec/content/` derived from it. |
| Cache / queue / sessions | Configured as `database`. **Nothing uses them**: 0 jobs, no Job, Mail or Notification classes, the API group has no session middleware. |
| Files | Uploads go to `../public/uploads/` (the SPA's folder). Export writes `../src/chapters.json`. Both paths are relative to `backend/`. |
| Existing tests | 2 scaffold tests (`ExampleTest` ×2). `composer test` passes. No test covers any API route. |
| Other plans in the repo | `spec/SPEC.md` §6 (decided 2026-10-28) plans a **fresh Laravel rebuild** on Debian 12 with MariaDB and Redis. The project `CLAUDE.md` mandates Laravel + Sanctum. **This brief conflicts with both.** Plan Q1 was decided on 2026-11-19 as **option A**: a 1:1 port of today's API, run locally on SQLite. |
| Deploy target | Global rule: Debian 12 (bookworm), x86_64, systemd. No production exists yet for this API. |
| Toolchain | Rust is **not installed** on the authoring machine. |

### Target stack (adjusted)

| Concern | Choice | Why it differs from the generic brief |
|---|---|---|
| Runtime / web | Rust stable (≥ 1.94, required by sqlx 0.9), Tokio, Axum, tower-http (trace, timeout, request-id) | **CORS is hand-written** (`server/app/src/cors.rs`): Laravel's 204 preflights and `Vary` rules differ from `tower_http::cors`. Debian builds need rustup, because bookworm's packaged `rustc` is too old. Exact versions: MIGRATION_PLAN.md, "Crates". |
| DB | SQLx with compile-time-checked queries, **SQLite** (same engine as today) | MariaDB only if Q4 says so. Phase 2's two infrastructure queries (schema check, health ping) use the runtime API with bound parameters. `query!` macros and `cargo sqlx prepare` start in slice 3.1. |
| Templates | **N/A**. Laravel serves no UI for the SPA. The only Blade view is the scaffold `welcome` page on `/`. | |
| Password hashing | **Not needed for login.** Login is `hash_equals(EDITOR_PASSWORD, input)`. `users.password` holds a bcrypt hash of random bytes that nobody ever verifies. `bcrypt` is needed only if Rust must create the `editor@local` row with a `$2y$` hash (Q-list). | Generic brief assumed user passwords |
| Sessions / cookies / CSRF | **N/A**. Auth is bearer-token only. CORS has `supports_credentials: false`. The API group has no `statefulApi()`. | |
| API tokens | Sanctum-compatible bearer tokens (sha2, crc32fast, rand) | Main auth trap |
| Validation | Hand-written validators that emit **Laravel's exact messages** for the rules used. `garde`/`validator` only if they can produce those strings. | Message parity matters: the SPA shows `message` |
| Errors | `thiserror` inside, `anyhow` only in `main` | Same |
| Config | `dotenvy` + typed struct, fail fast | Same |
| Jobs / scheduler / mail / broadcasting | **N/A**. None exist. | |
| Logging | `tracing` + `tracing-subscriber` (JSON outside dev) | Same |
| JSON | `serde_json` with **`preserve_order`** | PHP arrays keep key order, and the export file is diffed in git |
| Multipart / uploads | Axum `multipart`. Content sniffing that matches PHP `finfo`, including `image/svg+xml`. | |
| IDs | `ulid` | |

## Hard rules

1. Work on branch `migration/rust` (created). Rust lives in `server/`. Do not
   modify or delete `backend/` until the user approves cutover.
2. No destructive schema change. Rust runs against the schema that
   `backend/database/migrations/` produces, as is. Any change ships as a
   reversible migration **and** a `.sql` file the user applies (global rule:
   no migrations against real data from this machine).
3. One commit per completed phase. The repo is **public**
   (`origin` on GitHub), so every commit is published:
   - Scan the staged diff for secrets, real emails, absolute `/Users/…` paths and `.env` values first.
   - Stage only the phase's files. Never sweep in the untracked `backend/`, `spec/` or `src/` work without approval (Q13).
4. No `unwrap()`/`expect()`/`panic!` in request paths, no `unsafe`, no SQL
   string concatenation.
5. Ambiguous or irreversible → stop and ask. That covers scope, parity for known bugs, the auth scheme, URLs, dropping routes, the DB engine and the deploy target.
6. A phase is done only when its tests pass and the command output is shown.
7. **Global rules apply:**
   - Vet every crate and tool before install (genuine, no incidents, `cargo audit`, read `build.rs`, pin with `Cargo.lock`).
   - Bump the version and name artifacts by version for anything installable.
   - Build Linux x86_64 for Debian 12 when the target is the server.
   - Every change updates its docs: `README.md`, project `CLAUDE.md`, and `spec/SPEC.md` §6 if the scope changes.
   - Advisor checkpoints before plans and before commits.
   - Self-exiting local servers: 600 s idle, bound to `127.0.0.1`.

## Parity: what "behaviour stays the same" means here

- The **oracle** is the Laravel app run from a sandbox copy (see Phase 1)
  with `APP_DEBUG=false` and a fresh SQLite seeded from `src/chapters.json`.
  The damaged `database.sqlite` is never used.
- Compare **parsed JSON**, status codes and the headers that matter. Byte
  equality is not the test, because PHP escapes `é` as `é` and `/` as `\/` in
  responses. **Exception:** the file that `POST /api/export` writes must
  match **byte for byte**: 4-space pretty print, unescaped Unicode and
  slashes, trailing `\n`.
- **Known destructive bugs are not oracle assertions.** D-03 (validation
  strips exercise fields), D-04 (`TrimStrings` eats the spaces in `before`/`after`),
  and `ConvertEmptyStringsToNull` (turns `""` answers into `null`). The suite records
  them as `KNOWN_ISSUES` cases with both expected values. **Q2 was decided on
  2026-11-19: fix both.** Rust keeps every exercise field and stops
  trimming and nulling strings anywhere in the request (plan Q2a).
- Rust writes must stay readable by Laravel (rollback):
  - timestamps as `YYYY-MM-DD HH:MM:SS` UTC text;
  - `payload` as JSON text;
  - token rows in Sanctum's format.

## Compatibility traps for this repo

Each one gets a contract test and a line in INVENTORY or KNOWN_ISSUES.

- **Sanctum tokens.**
  - Format: `<id>|<40 random chars><crc32b hex of those 40>`.
  - Storage: `sha256(<part after the pipe>)` in `personal_access_tokens.token`.
  - Lookup: with a pipe, find by id, then compare with `hash_equals`. Without a pipe, look up by hash.
  - Valid when `expires_at` is null or in the future (`sanctum.expiration` is null), and the tokenable user exists.
  - Every authenticated request writes `last_used_at`.
  - Logout deletes only the current token.
  - Login creates a new token every time and never cleans up. There are 11 local tokens today.
- **Error bodies.**
  - 422: `{"message": "<first error>[ (and N more error|errors)]", "errors": {field: [msgs]}}`.
  - 401: `{"message":"Unauthenticated."}`.
  - 404 from `findOrFail`, 404 for unknown routes, 405 for a wrong method, 413 from `ValidatePostSize`: capture the exact text in Phase 1. Don't guess it.
- **Laravel validation messages** for the rules in use: `required`,
  `string`, `max` (string and file), `regex`, `unique`, `integer`, `min`,
  `present`, `array`, `in`, `file`, `mimetypes`, `nullable`, `sometimes`.
  Field names in messages use the dotted path (`lesson.formation`).
- **Global request middleware.** `TrimStrings` (skips `password`,
  `password_confirmation` and `current_password`) and `ConvertEmptyStringsToNull`. **Rust drops them for exercise content
  (Q2 = fix).** Plan Q2a (2026-11-19) extends this **everywhere**,
  top-level fields included.
- **The `required` rule fails a whitespace-only string even without
  `TrimStrings`.** Laravel checks `trim($value) === ''`, and the Rust
  validator must do the same.
- **`validate()` returns only validated keys** (D-03). **Fixed in Rust (Q2):**
  the whole payload is kept.
- **JSON round-trip quirks of PHP.** An empty object `{}` comes back as `[]`. Key order is
  kept. `payload.number` keeps the client's raw value (a string `"3"` stays a string)
  while the `number` column is cast to int.
- **Dates.** `uploads.created_at` serialises as `2026-05-17T10:00:00.000000Z`.
  `app.timezone` is `UTC`.
- **Uploads.**
  - Size: the effective cap is `min(5120 KB rule, PHP upload_max_filesize=2M locally)`. Above 2 MB Laravel returns 422 "The file failed to upload." Capture this and decide whether to keep it.
  - Type: MIME is sniffed from content. SVG is allowed (stored-XSS candidate).
  - IDs: the DB `id` is a lowercase ULID that **differs** from the uppercase ULID in the filename, because `id` isn't fillable (to confirm in Phase 1).
  - URL: `FRONTEND_BASE_URL` + `/uploads/<file>`.
- **Export.** It writes to the repo's `src/chapters.json`. The response
  includes the **absolute** path. It returns 200 with `written:false` when the write
  fails.
- **CORS.**
  - Paths: `api/*` and `sanctum/csrf-cookie`.
  - Origins: `localhost:5173`, `127.0.0.1:5173`, `localhost:4173`, plus `FRONTEND_ORIGIN`.
  - Headers and methods: `*`. No credentials. `max_age 0`.
- **Framework routes in `route:list`.** `GET /` (Blade welcome), `GET /up`,
  `GET /sanctum/csrf-cookie`, `GET|PUT /storage/{path}`. Keep, port or drop (Q6).
- **Ordering.** Chapters sort by `position`, then `number`. Uploads sort by `created_at`
  desc, and ties are unordered.

## Phases (adapted)

- **Phase 0. Discovery (this deliverable).** INVENTORY.md and
  MIGRATION_PLAN.md. **Stop for approval.**
- **Phase 1. Characterisation suite.** First install and vet the Rust
  toolchain (plan Q14). Then write the HTTP tests as Rust integration tests
  in the `server/contract` crate (plan Q12). They take `BASE_URL`. They run against Laravel from a **sandbox copy** (temp folder
  holding `backend/`, `src/` and `public/uploads/`), so export and upload never touch the real
  repo. Use a fresh temp SQLite, `migrate --seed`, `APP_DEBUG=false`, a test
  `EDITOR_PASSWORD`, and a server bound to `127.0.0.1`. `php artisan serve`
  has no idle hook, so the runner kills it at a **600 s hard deadline**
  (global rule). Every route gets these cases:
  - guest → 401, bad token, expired or deleted token;
  - validation (each rule, with the message);
  - happy path;
  - 404 and 405;
  - CORS preflight.

  Known bugs go to `KNOWN_ISSUES.md`.
- **Phase 2. Rust foundation.** The toolchain is already installed in
  Phase 1. Deliver:
  - typed config, SQLite pool, tracing, request ids, graceful shutdown;
  - `/up`;
  - a Laravel-shaped error type;
  - CORS;
  - sqlx baseline migrations, used **only for fresh/test DBs** and proven identical with `sqlite3 .schema`.

  **Vite integration: N/A.** The SPA stays a separate Vite app. The only
  frontend contract is `VITE_API_URL` plus the JSON shapes. Serving `dist/` from
  Rust is a scope-B question.
- **Phase 3. Vertical slices, in this order:**
  1. auth (token guard, login, logout);
  2. chapters read;
  3. chapters write (the validation engine);
  4. uploads;
  5. export.

  Each slice passes its Phase 1 tests, then gets a commit.
- **Phase 4. Background work: N/A** (evidence in INVENTORY §7). Kept as a
  checkpoint that confirms nothing was missed.
- **Phase 5. Audit loop:**
  - `cargo fmt --check`, `cargo clippy -D warnings` (pedantic where sensible), `cargo test` plus the full contract suite;
  - `cargo audit`, `cargo deny check`, `cargo sqlx prepare --check`;
  - OWASP pass: every route is protected except login and `/up`, there is a test that enumerates the routes, login rate limiting (Laravel has none: Q9), SVG, path traversal on upload and delete, secrets in logs;
  - `spawn_blocking` for file I/O and hashing where heavy;
  - `npm run build`, and an admin smoke test against Rust;
  - a light `oha` benchmark. This is a single-editor tool, so the numbers are informational.
- **Phase 6. Deploy and cutover**, per Q3:
  - **Local tool:** `cargo run` / a release binary, plus a README swap.
  - **Debian:** a `x86_64-unknown-linux-gnu` build against bookworm glibc, a systemd unit, a versioned tarball `grammaire-server-<ver>.tar.gz`, and an nginx snippet. Docker only if asked (Q11).

  `CUTOVER.md` covers:
  - the env mapping;
  - the fact that there is no queue to drain;
  - token continuity;
  - rollback: start `php artisan serve` again on the same DB, which works because Rust writes Laravel-compatible rows.

## Deliverables

- `docs/migration/INVENTORY.md`, `MIGRATION_PLAN.md`, `KNOWN_ISSUES.md`,
  `AUDIT_REPORT.md`, `CUTOVER.md`, and a **parity matrix**: route → Rust handler →
  test name → status.
- `server/` passing the full contract suite, minus approved differences.
- The SPA unchanged: same `VITE_API_URL`, same shapes.
- CI: fmt, clippy, test, audit, deny, sqlx check. Today's only workflow is the
  GitHub Pages deploy.
- Updated `README.md`, project `CLAUDE.md`, and `spec/SPEC.md` §6, to match the chosen scope.

## Reporting

At the end of each phase, report:

- what was done;
- the command outputs;
- decisions taken;
- differences from Laravel;
- open questions.

Keep the report short and put the detail in `docs/migration/`.
