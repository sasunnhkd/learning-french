# Phase 5 audit: Rust port of the backoffice API

Date: 2026-11-23. Branch `migration/rust`. Scope: `server/` (the Rust app and
its contract suite), plus a production build and an admin smoke test of the
SPA against the Rust server. The plan's Phase 5 row (MIGRATION_PLAN §2) lists
the checks; this report gives each result and every finding.

**Summary.** Every gate is green. The audit found one high-severity bug: one
unauthenticated request could crash the server. It is fixed and tested, along
with three smaller issues (F2, F5, F9). Phase 5's acceptance ("all clean, or
the findings approved") is **met as of 2026-11-24**: you approved the
defaults for the four decisions (§2–§5), and D1 is fixed:

- F3: the login rate limit behind nginx.
- D1: a `url` patch update.
- P1: SQLite WAL mode.
- A: the remaining pedantic lints.

The other findings are accepted as Laravel parity or deferred to Phase 6
(nginx config).

## 1. Gates

| Check | Result |
|---|---|
| `cargo fmt --check` | clean |
| `cargo clippy --workspace --all-targets -D warnings` | clean |
| `cargo clippy -W clippy::pedantic` | about 170 warnings, triaged in §5 |
| App tests (`cargo test -p grammaire-server`) | 71 passed |
| `scripts/sqlx-prepare.sh --check` | up to date |
| Contract suite against Laravel | 87/87 (83 HTTP tests + 4 helper tests) |
| Contract suite against Rust | 83/83 (`ported.txt` lists every test) |
| Negative run (`TARGET=rust` against Laravel) | fails exactly the 16 tests tagged in KNOWN_ISSUES, so the other 67 make no Laravel-only assumptions |
| Export `cmp` (`server/contract/export-cmp.sh`) | identical, 342,024 bytes |
| Route enumeration (`framework::every_api_route_is_guarded_like_laravel`, new) | passes on both. It reads every route from Laravel's router (12 under `/api`) and checks 401 for guests and bad tokens wherever Laravel puts `auth:sanctum`, and no 401 on login. A route added to Laravel later fails this test on Rust until it is ported. |
| SPA `npm run build` | builds: `index` 444.7 kB (127.7 kB gzip), `AdminApp` 411.8 kB (130.6 kB gzip) |
| Admin smoke test (browser pane) | passes (§6) |

Tools: `cargo-audit`, `cargo-deny` and `oha` are not installed. Each would
need your package review first, so the audit used substitutes that install
nothing:

- **Advisories:** the public OSV database (`api.osv.dev`), queried with the name and version of every locked crate. RustSec advisories are in OSV.
- **Licenses, sources and duplicates:** `cargo metadata` and `cargo tree -d`.
- **Benchmark:** ApacheBench (`ab`, shipped with macOS and in Debian's `apache2-utils`).

## 2. Security findings (OWASP pass)

An independent reviewer read `server/app/src` against the OWASP Top 10 and
the API Security Top 10. Each claim below was then checked against the code
or the running apps.

| # | Severity | Finding | Status |
|---|---|---|---|
| F1 | **High** | **Unauthenticated crash.** Form and multipart field names had no nesting limit, so `POST /api/auth/login` with a form field `a[b][b]…` 20,000 levels deep (about 60 KB) overflowed the stack in the recursive `input::finish` and **aborted the whole server**. Reproduced: Rust answered nothing and `/up` went down; Laravel answered 422. | **Fixed.** `input.rs` now applies PHP's own limits, each probed on Laravel: names deeper than 64 levels are dropped (`max_input_nesting_level`); a urlencoded body keeps its first 1001 fields and a multipart body its first 1000 text fields (`max_input_vars`, off by one on urlencoded); file parts don't count; at most 20 file parts with a file name (`max_file_uploads`; empty-name parts don't count). After the fix the same request gets 422 and the server stays up. Tests: `auth::form_input_limits_like_php` and `uploads::file_part_cap_like_php` (contract, both apps), and `input::tests::php_input_limits`. |
| F2 | Medium | Appending with `a[]=` scanned every key each time, so it was quadratic: about 1.6 million appends in an 8 MB login body, run synchronously where the request timeout can't interrupt it. | **Fixed by F1's limit:** at most 1001 fields, so at most about 10^6 key checks. The next index now also follows PHP 8.3+ (after `-5` comes `-4`; `"05"` is a string key; past `PHP_INT_MAX` the value is dropped). |
| F3 | Medium | **The login rate limit becomes global behind nginx.** `ClientIp` is the TCP peer, so behind a proxy on 127.0.0.1 every client shares one bucket. Five bad logins a minute from anyone lock the editor out. | **Done in Phase 6 (2026-11-26):** the limiter keys on `X-Real-IP` when the peer is loopback (`auth::client_ip`), and `server/packaging/nginx-grammaire.conf` sets that header and adds `limit_req` on login. |
| F4 | Low | Authenticated CPU cost. The validator looks up children linearly, so a body with about 10^6 `order` or `items` entries takes seconds of CPU. | **Accept:** only the editor's token can send it, and Laravel's validator has the same shape. |
| F5 | Low | The login password compare returned early on a length mismatch, so timing revealed the length of `EDITOR_PASSWORD`. PHP's `hash_equals` leaks it the same way. | **Fixed:** the two SHA-256 digests are compared instead, so both sides are always 32 bytes. Answers are unchanged. |
| F6 | Low | No cap on token rows (each login makes one that never expires) or on total upload size. | **Accept:** Laravel parity (K5). Only reachable after login. |
| F7 | Low | Export writes in place (truncate, then write), so a crash mid-write can leave a truncated `src/chapters.json`. | **Accept:** Laravel parity, and the contract test pins it (`export_reports_unwritable_file`: a read-only file stays untouched, `written: false`). A temp file plus rename would break that test. |
| F8 | Info | Hardening that belongs in nginx: `X-Content-Type-Options: nosniff`, a strict CSP on `/uploads/`, header-read timeouts and connection limits (slowloris), whether `/up` is public, warning on `BIND_ADDR=0.0.0.0`. A client-supplied `x-request-id` is echoed into logs, which allows log forging. | **Done in Phase 6 (2026-11-26):** nginx sets `nosniff`, `X-Frame-Options`, `Referrer-Policy`, HSTS and slow-client timeouts, and keeps `/up` to the host. The app keeps a client `x-request-id` only if it matches `[A-Za-z0-9._-]{1,64}` (`http::drop_unsafe_request_id`), and warns at startup when `BIND_ADDR` isn't loopback. Uploads are served by GitHub Pages, whose headers can't be set; SVG is refused there anyway (K4). |
| F9 | Info | Two overflows: `max(position) + 1` at `i64::MAX`, and the form append index. Release builds would wrap silently. | **Fixed:** `saturating_add` and `checked_add`. |

The reviewer checked these areas and found them sound:

- **Access control:** every route except `/up` and login takes `AuthUser` first.
- **Path traversal:** upload names are generated by the server, and delete uses `basename(path)`.
- **SQL:** all queries use parameters; the one runtime query is a static string.
- **Tokens:** generated by the OS CSPRNG with 238 bits of entropy; only the SHA-256 is stored; compared in constant time.
- **Logs:** no secrets are written; `Secret` is redacted.
- **CORS:** exact origin match, no credentials.
- **Request size:** body limits hold, and JSON nesting is capped at 128.
- **Code:** no `unsafe`, and no outbound requests (so no SSRF).

## 3. Dependencies

- **Sources:** 200 packages; 198 come from crates.io, and the other 2 are this workspace.
- **Licenses:** all permissive (MIT, Apache-2.0, BSD-3-Clause, Unicode-3.0, Zlib, BSL-1.0, Unlicense). The only copyleft is r-efi's LGPL-2.1, offered as one choice in `MIT OR Apache-2.0 OR LGPL-2.1-or-later`.
- **Duplicates:** a few versions appear twice (`base64`, `hashbrown`, `syn`), pulled in by the toolchain crates. Harmless.

| # | Finding | Status |
|---|---|---|
| D1 | **`url` 2.5.8, RUSTSEC-2026-0592 (GHSA-hrj5-67jp-wqp9).** `Url::origin()` on a URL with thousands of nested `blob:` prefixes recurses until the stack overflows. `url` comes in only through sqlx, which parses `DATABASE_URL` with it. Nothing in the server or sqlx calls `Url::origin()`: the only `.origin()` call is on sqlx's own column type. So it is **not reachable**. | **Fixed 2026-11-24.** Updated to `url` 2.5.9 and, as it requires, `form_urlencoded` 1.2.3 (pin in `app/Cargo.toml`), both approved by you. Checked first: same project (servo/rust-url) and publisher; no OSV advisory; no reported compromise; fetched without building; no build script in either (`build = false`); about 100 changed lines, all URL-standard fixes (the `blob:` origin fix, a serializer panic replaced by clamping, parser and doc fixes), with no environment, file, network, process or `unsafe` code. Only those two lines moved in `Cargo.lock`; `idna` and `percent-encoding` stayed. Afterwards: OSV finds 0 advisories in all 198 crates, and every gate is green. |

## 4. Performance

`server/contract/bench.sh` runs the same requests through `ab` against
Laravel (`php -S`, single-threaded, as the sandbox starts it) and a release
build of Rust, each on a freshly seeded sandbox. 500 requests per case,
authenticated; latencies in ms.

| Case | Laravel p50 / p95 / p99 | Laravel req/s | Rust p50 / p95 / p99 | Rust req/s |
|---|---|---|---|---|
| index, 1 client | 4 / 4 / 5 | 266 | 0 / 0 / 0 | 2,815 |
| index, 10 clients | 35 / 38 / 39 | 280 | 5 / 24 / 96 | 1,173 |
| show (largest chapter), 1 client | 3 / 4 / 5 | 286 | 0 / 0 / 0 | 3,130 |
| show, 10 clients | 33 / 34 / 40 | 301 | 2 / 7 / 26 | 2,421 |
| `/up`, 10 clients | 19 / 20 / 21 | 517 | 1 / 1 / 1 | 17,281 |

`php -S` is not how Laravel would run in production (that would be php-fpm
with several workers), so read the Laravel column as the local baseline the
contract suite uses, not as production numbers.

| # | Finding | Status |
|---|---|---|
| P1 | **SQLite lock contention at 10 parallel clients:** index drops to fewer requests per second than with 1 client, and p99 reaches 96 ms. Every authenticated request writes `last_used_at` (Laravel parity), and in the default rollback-journal mode a write waits for all readers, retrying with sleeps. Confirmed on a throwaway sandbox: in WAL mode the same test gives **4,838 req/s, p50/p95/p99 2/3/4 ms** (journal mode: 1,112 req/s, 5/29/71). | **Decided 2026-11-24: keep the rollback journal.** A single editor rarely sends parallel requests. To switch later, run `sqlite3 <db> 'pragma journal_mode=wal'` once on the server (never from the app), and make backups and file permissions cover the `-wal` and `-shm` files. |

## 5. Code quality (pedantic clippy)

Of the 170 or so pedantic warnings, most are style:

- `must_use` attributes (73);
- missing `# Errors` and `# Panics` doc sections (40);
- missing backticks in docs (22);
- `single_match_else` (5);
- long test functions.

None is a bug. Two small changes were made: a doc code span that rendered
empty, and a `format!` allocation per byte in the path rewriter (`paths.rs`).

| # | Finding | Status |
|---|---|---|
| A | Turn on `clippy::pedantic` in CI, with the style lints above allowed by name? | **Decided 2026-11-24: no.** CI keeps `-D warnings` on clippy's default lints. |

## 6. Admin smoke test

The SPA ran on Vite at `127.0.0.1:5191` with `VITE_API_URL` pointing at a
sandboxed Rust server (fresh seed, throwaway password). `run.sh` takes
`CONTRACT_FRONTEND_ORIGIN` for the extra CORS origin. Both processes stopped
afterwards, and the sandbox was removed. Your own dev servers on 5173 and
5174 were left alone.

- Login: works; the chapter list loads, with all sections.
- Editor: "Le subjonctif présent" opens; the Identité and Exercices tabs show the stored content.
- Save without changes: 200 and back to the list. The stored `lesson` and `items` are **identical before and after**, key order included. On Laravel the same save strips the exercise fields (K1).
- Not clicked:
  - **Export:** the SPA turns the export into a browser download, which needs your OK. The contract test and `export-cmp.sh` cover it.
  - **Image upload:** needs a file chooser. The 13 upload contract tests cover it.

## 7. Changes made in Phase 5

- `server/app/src/input.rs`: PHP input limits, PHP 8.3+ next index, overflow-safe (F1, F2, F9); a doc code span fixed.
- `server/app/src/auth.rs`: digest compare at login (F5).
- `server/app/src/chapters.rs`: saturating `max(position) + 1` (F9).
- `server/app/src/paths.rs`: hex encoding without a `format!` per byte.
- Contract tests: `framework::every_api_route_is_guarded_like_laravel`, `auth::form_input_limits_like_php`, `uploads::file_part_cap_like_php`.
- `server/contract/run.sh`: `CONTRACT_RELEASE=1` (release build) and `CONTRACT_FRONTEND_ORIGIN`.
- `server/contract/bench.sh`: the benchmark above.
