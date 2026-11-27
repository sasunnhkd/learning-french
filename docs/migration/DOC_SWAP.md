# Docs to switch at cutover

Apply these the day the Rust server goes live (`CUTOVER.md` §8). Until then
the files correctly describe the Laravel setup. They are your working files,
outside the migration commits (`README.md` has uncommitted edits;
`CLAUDE.md` and `spec/` are untracked), so the change is yours to make or to
ask for.

## README.md

**Intro, second paragraph.** Replace "(Laravel + SQLite)" and what follows
with:

> Authoring is done through a backoffice: the `#/admin` route of the same
> SPA, talking to a small Rust API (`server/`, SQLite) on the Debian 12
> server. It writes the chapters to `src/chapters.json` in its checkout,
> which is committed and pushed to publish.

**Project layout.** Replace the `backend/` line with:

```
├── server/                    Rust API used by the backoffice (Debian 12; see server/README.md)
├── backend/                   Former Laravel API, kept for rollback (docs/migration/CUTOVER.md §7)
```

**Backoffice → One-time setup and Day-to-day.** Replace both subsections
with:

````markdown
### Production

The API runs on the Debian 12 server behind nginx. Install, upgrade and
rollback: `docs/migration/CUTOVER.md`. The Pages build points the admin at
it with `VITE_API_URL`.

### Local development

```bash
# terminal 1: a fresh database from the current content, then the API
server/scripts/initial-db-sql.sh
sqlite3 /tmp/grammaire.sqlite < server/ci-artifacts/grammaire-initial-<date>.sql
cd server && DATABASE_URL=sqlite:/tmp/grammaire.sqlite EDITOR_PASSWORD=dev \
  REPO_ROOT="$PWD/.." IDLE_TIMEOUT_SECS=600 cargo run --release   # http://localhost:8000

# terminal 2
npm run dev                                # http://localhost:5173/learning-french/
```
````

**Publishing.** Replace the subsection's text (keep the git block) with:

> On the server, **exporter chapters.json** writes `src/chapters.json` in
> the server's checkout, and uploads land in its `public/uploads/`. Commit
> and push from there (`docs/migration/CUTOVER.md` §4); GH Pages redeploys.
> Locally (development), the same commands work in your own checkout.

## CLAUDE.md (project)

**Stack.** Replace the `backend/` bullet with:

> - **`server/`**: Rust (Axum, sqlx, SQLite) JSON API, a 1:1 port of the
>   former Laravel API (`docs/migration/`). Runs on Debian 12 behind nginx
>   (`server/packaging/`). Bearer tokens compatible with Sanctum.
> - **`backend/`**: the former Laravel 13 API, kept only for rollback and as
>   the contract suite's oracle (`server/contract/`). No new features there.

**Hard rules.** Add:

> - **The API lives in `server/`.** New endpoints, validation and data
>   changes go there, with a contract test in `server/contract/tests/`, run
>   against both apps while `backend/` exists.
> - **Schema changes ship as SQL files** that you apply on the server; the
>   Rust server never migrates the production database.

**Where things go.** Replace the Laravel rows with:

| Concern | Location |
|---|---|
| API routes and handlers | `server/app/src/http.rs`, `chapters.rs`, `uploads.rs`, `export.rs`, `auth.rs` |
| Validation | `server/app/src/validation.rs` (Laravel's rules), `php.rs` |
| Schema baseline (fresh and test databases only) | `server/app/migrations/` |
| Tests | `server/app/tests/` (in-process), `server/contract/tests/` (both apps) |
| Packaging | `server/packaging/` (systemd, nginx, release build) |

**Commands.** Replace the Laravel block with:

```
cd server && cargo test -p grammaire-server   # app tests
server/contract/run-rust.sh                   # contract suite against Rust
server/contract/run-laravel.sh                # ... against Laravel (oracle)
server/scripts/sqlx-prepare.sh                # after changing a query! macro
server/packaging/build-release.sh             # Debian 12 tarball (CI builds it)
```

## spec/SPEC.md §6

Add under the heading "6. Rebuild proposal: SPA + Laravel API on the Debian
server":

> **Status (2026-11-19, migration plan Q1): not taken.** The API was ported
> 1:1 to Rust instead (`docs/migration/`), and runs on Debian 12 behind nginx
> with SQLite. Drafts, versions, publishing, learner accounts and MariaDB
> below remain a possible later rebuild; the migration plan's "Under B"
> column says what each Rust slice would become there.
