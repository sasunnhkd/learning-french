# Migration plan: Laravel API → Rust (Axum)

Status: **Phase 1 done (2026-11-20).** The contract suite is green against
Laravel: 57 tests, plus 4 helper unit tests. Next is Phase 2, the Rust
foundation. The open decisions are Q16 (K11) and the defaults marked in
`KNOWN_ISSUES.md`.

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
| Q6 | Framework routes. F1 `/` (Blade welcome), F3 `/sanctum/csrf-cookie`, F4/F5 `/storage/{path}`: drop? F2 `/up`: keep? | Drop F1, F3, F4, F5. Keep `/up` (200, plain body). | Phase 2 |
| Q7 | K3 upload id mismatch: keep it, or use the filename ULID as the row id? | **Fix for new rows.** Old rows keep working because delete goes through `path`. | Slice 4 |
| Q8 | K4 SVG uploads (XSS vector on the public origin): keep, drop SVG, or sanitise? | Drop SVG, unless you use it. There are 0 uploads today. | Slice 4 |
| Q9 | K5: add a login rate limit, and optionally expire or prune tokens? | Add a limit (5/min per IP), configurable through env and turned off by `run-rust.sh`, because the contract suite logs in about 60 times. Leave expiry as is. | Slice 1 |
| Q10 | K6: keep the absolute `path` and `written:false` + 200 in the export response? | Keep `written`, return `path` relative to the repo. | Slice 5 |
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
| Q15 | Upload size: keep Laravel's effective 2M cap (from PHP `upload_max_filesize`), or honour the 5120 KB rule? | 5120 KB. That is what the rule says. | Slice 4 |
| Q16 | K11 (found in Phase 1): a guest request without `Accept: application/json` gets a 500 HTML page from Laravel. Should Rust answer 401 JSON instead? | **Default: yes**, always 401 `{"message":"Unauthenticated."}`. The SPA always sends the header, so it sees no change. | Slice 1 |

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

```
server/
  Cargo.toml            # package grammaire-server, version 0.1.0
  Cargo.lock  deny.toml  .sqlx/   # offline query data for CI
  migrations/           # baseline = Laravel DDL for app tables (test/fresh DBs only)
  src/
    main.rs             # config, tracing, pool, router, graceful shutdown
    config.rs           # typed env: DATABASE_URL, EDITOR_PASSWORD, FRONTEND_BASE_URL,
                        #   FRONTEND_ORIGIN, SANCTUM_TOKEN_PREFIX, REPO_ROOT, BIND_ADDR
    error.rs            # AppError → Laravel-shaped JSON (401/404/405/413/422/500)
    validation/         # rule engine + Laravel English messages, dotted paths, "(and N more errors)"
    auth/               # sanctum.rs (format, hash, lookup, last_used_at), extractor, login/logout
    chapters/           # model, repo (SQLx), handlers, payload rules
    uploads/            # handlers, MIME sniffing (incl. SVG), path guard
    export.rs           # PHP-compatible pretty JSON writer (4 spaces, unescaped, trailing \n)
  tests/                # Rust unit/integration tests (token vectors, JSON writer, validation messages)
  scripts/osv-audit.py  # OSV advisory check of Cargo.lock + banned-crate list (until cargo-audit/deny are installed)
  contract/             # workspace member: HTTP parity suite (std-only client + serde_json), BASE_URL-driven (Q12)
    src/lib.rs          # Ctx (env), Req/Resp HTTP client, validation_body, php_pretty, crc32b, TINY_PNG
    tests/*.rs          # one file per route group
    run-laravel.sh      # sandbox copy + temp SQLite + seed + artisan serve on 127.0.0.1, killed at a 600 s hard deadline
    run-rust.sh         # same sandbox, Rust binary (Phase 2)
```

### Crates (each vetted before adding, versions pinned in `Cargo.lock`)

| Crate | Use |
|---|---|
| `tokio`, `axum` (with `multipart`), `tower`, `tower-http` (cors, trace, timeout, request-id, limit) | Server |
| `sqlx` (`sqlite`, `runtime-tokio`, `macros`) | DB, compile-time checked |
| `serde`, `serde_json` (`preserve_order`) | JSON with PHP key order |
| `sha2`, `crc32fast`, `rand`, `subtle`, `hex` | Sanctum tokens, constant-time compare |
| `ulid`, `time` (or `chrono`) | Upload ids, timestamp text |
| `regex` | `id` rule `^[a-z0-9-]+$` |
| `infer` + an own SVG check | MIME sniffing. `infer` doesn't detect SVG, and PHP's `finfo` does. |
| `dotenvy`, `thiserror`, `anyhow` (main only), `tracing`, `tracing-subscriber` | Config, errors, logs |
| `tower_governor` (only if Q9 = yes) | Login rate limit |
| Not needed under A | `bcrypt`/`argon2` (no password is verified), sessions, `lettre`, `apalis`, templates |

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
| 1 | Toolchain install (Q14, each tool and crate vetted). A `server/` Cargo workspace holding only the `contract` crate. The suite runs against sandboxed Laravel. It covers every R1–R12 route: guest, bad or deleted token, happy path, each validation rule with its message, 404, 405, 413, CORS preflight. Fixtures come from `src/chapters.json` (26 chapters plus the placeholder). Deliverable: `KNOWN_ISSUES.md` (K1–K13). | **Done 2026-11-20:** 57/57 green against Laravel in about 20 s. With `TARGET=rust`, exactly the 11 tagged tests fail (KNOWN_ISSUES). | Acceptance tests from SPEC §6 instead |
| 2 | `server/app` crate skeleton, config, tracing, `/up`, error type, CORS (exact origin list), SQLite pool, sqlx baseline proved by a `.schema` diff, `cargo sqlx prepare`. | `cargo fmt/clippy/test` green. `/up` and 401-for-guest contract tests pass against Rust. | MariaDB pool, new schema as `.sql` |
| 3.1 | Auth slice: Sanctum token module (vectors from real Laravel-made tokens), guard extractor, `last_used_at`, login and logout | R1, R2 and every guest/bad-token case green | Cookie auth, CSRF, register, verify, reset |
| 3.2 | Chapters read: R3, R4 (ordering, shape, null `lesson`, `items` default `[]`) | green | Public read endpoints + ETag |
| 3.3 | Chapters write: R5–R8, the validation engine, `position` logic, transaction for reorder, Q2 outcome | green, plus a round-trip test: the 26 chapters saved and read back are byte-equal (Q2 = fix) | Drafts, versions, publish |
| 3.4 | Uploads: R9–R11, sniffing, path guard, Q7/Q8/Q15 outcomes | green | same |
| 3.5 | Export: R12, byte-identical file against the Laravel output | green, `cmp` of both files = 0 | Replaced by publish |
| 4 | Background work: **none to port** (INVENTORY §5). Checkpoint commit only, adding the parity matrix rows. | Matrix lists every route and command | Mail queue, scheduler |
| 5 | Audit loop: fmt, clippy (pedantic where sensible), tests, the full contract suite, `cargo audit`, `cargo deny`, `sqlx prepare --check`, a route-enumeration auth test, OWASP pass, `oha` p50/p95/p99 against Laravel, `npm run build`, an admin smoke test in the browser pane. Deliverable: `AUDIT_REPORT.md`. | All clean, or the findings approved | same, larger |
| 6 | CI workflow (fmt, clippy, test, audit, deny, sqlx check, contract suite). Packaging per Q3/Q11. `CUTOVER.md`. Swap README, project `CLAUDE.md` and `spec/SPEC.md` §6. | CI green. Rollback tested once (Laravel reads the Rust-written DB). | Debian systemd, nginx, tarball, SQL files |

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

| Laravel | Rust handler | Contract test | Status |
|---|---|---|---|
| R1 `POST /api/auth/login` | `auth::login` | `server/contract/tests/auth.rs` | Laravel ✓, Rust pending |
| R2 `POST /api/auth/logout` | `auth::logout` | `server/contract/tests/auth.rs` | Laravel ✓, Rust pending |
| R3 `GET /api/chapters` | `chapters::index` | `server/contract/tests/chapters_read.rs` | Laravel ✓, Rust pending |
| R4 `GET /api/chapters/{id}` | `chapters::show` | `server/contract/tests/chapters_read.rs` | Laravel ✓, Rust pending |
| R5 `POST /api/chapters` | `chapters::store` | `server/contract/tests/chapters_write.rs`, `validation.rs` | Laravel ✓, Rust pending |
| R6 `PUT /api/chapters/{id}` | `chapters::update` | `server/contract/tests/chapters_write.rs`, `validation.rs` | Laravel ✓, Rust pending |
| R7 `DELETE /api/chapters/{id}` | `chapters::destroy` | `server/contract/tests/chapters_write.rs` | Laravel ✓, Rust pending |
| R8 `POST /api/chapters/reorder` | `chapters::reorder` | `server/contract/tests/chapters_write.rs` | Laravel ✓, Rust pending |
| R9 `GET /api/uploads` | `uploads::index` | `server/contract/tests/uploads.rs` | Laravel ✓, Rust pending |
| R10 `POST /api/uploads` | `uploads::store` | `server/contract/tests/uploads.rs` | Laravel ✓, Rust pending |
| R11 `DELETE /api/uploads/{id}` | `uploads::destroy` | `server/contract/tests/uploads.rs` | Laravel ✓, Rust pending |
| R12 `POST /api/export` | `export::run` | `server/contract/tests/export.rs` | Laravel ✓, Rust pending |
| F2 `GET /up` | `health` | `server/contract/tests/framework.rs` | Laravel ✓, Rust pending |
| F1, F3, F4, F5 | dropped (Q6) | `server/contract/tests/framework.rs` asserts 404 | Laravel ✓, Rust pending (Q6 default) |
| Jobs, commands, listeners | none exist (`inspire` only, scaffold) | — | N/A |
