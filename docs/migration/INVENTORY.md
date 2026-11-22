# Phase 0 inventory: Laravel backend (`backend/`)

Read on 2026-11-19 from the working tree on branch `migration/rust`. That is
the uncommitted state of `main`, and `backend/` is untracked. Sources are cited
as repo paths. Exact strings were captured from the running app in Phase 1
(2026-11-20, sandbox, `APP_DEBUG=false`). The contract tests in
`server/contract/tests/` assert them, and `KNOWN_ISSUES.md` (K1–K13) lists the
behaviours that look like bugs.

## 1. Routes

Source: `php artisan route:list --json`, `backend/routes/*.php`,
`backend/bootstrap/app.php`.

### 1.1 Application routes (`routes/api.php`, prefix `/api`, group `api`)

The `api` group holds only `SubstituteBindings`. There is no throttle and no
`statefulApi()`. Protected routes add `Authenticate:sanctum`.

| # | Method | URI | Action | Auth | Validation | Success | Errors |
|---|---|---|---|---|---|---|---|
| R1 | POST | `/api/auth/login` | `AuthController@login` | none | `password: required\|string` | 200 `{"token": "<id>\|<48 chars>"}` | 422 `errors.password = ["Invalid password."]` when `EDITOR_PASSWORD` is empty or differs. Compared with `hash_equals`. |
| R2 | POST | `/api/auth/logout` | `AuthController@logout` | sanctum | none | 200 `{"ok": true}`. Deletes the current token only. | 401 |
| R3 | GET | `/api/chapters` | `ChapterController@index` | sanctum | none | 200 `[{id, number, section, title, titleEm, position}]`, ordered by `position`, then `number` | 401 |
| R4 | GET | `/api/chapters/{id}` | `ChapterController@show` | sanctum | none | 200 chapter shape (§2.1) | 401, 404 `{"message": "No query results for model [App\\Models\\Chapter] <id>"}` |
| R5 | POST | `/api/chapters` | `ChapterController@store` | sanctum | §1.3, with `id` unique | **201** chapter shape | 401, 422 |
| R6 | PUT | `/api/chapters/{id}` | `ChapterController@update` | sanctum | §1.3 + an `id` equal to the route id | 200 chapter shape (after `refresh()`) | 401, 404, 422. An `id` change gives `errors.id = ["The id of an existing chapter cannot be changed."]`. |
| R7 | DELETE | `/api/chapters/{id}` | `ChapterController@destroy` | sanctum | none | 200 `{"ok": true}`. Hard delete. | 401, 404 |
| R8 | POST | `/api/chapters/reorder` | `ChapterController@reorder` | sanctum | `order: required\|array`, `order.*: string` | 200 `{"ok": true}`. In one transaction, each `id` gets `position = index`. Unknown ids are silently ignored. | 401, 422 |
| R9 | GET | `/api/uploads` | `UploadController@index` | sanctum | none | 200 `[{id, path, url, original_name, mime, size, created_at}]`, ordered by `created_at` desc | 401 |
| R10 | POST | `/api/uploads` | `UploadController@store` | sanctum | `file: required\|file\|max:5120\|mimetypes:image/png,image/jpeg,image/webp,image/gif,image/svg+xml` (multipart) | **201** `{id, path, url, mime, size}` | 401, 422, 500 (path guard, `abort(500, …)`) |
| R11 | DELETE | `/api/uploads/{id}` | `UploadController@destroy` | sanctum | none | 200 `{"ok": true}`. Unlinks the file, then deletes the row. | 401, 404 |
| R12 | POST | `/api/export` | `ExportController` (invokable) | sanctum | none | 200 `{count, path, written, chapters[]}`. Writes the file (§6). | 401 |

Route order: `POST /api/chapters/reorder` and `/api/chapters/{id}` don't
collide, because `{id}` has no POST. A `POST /api/chapters/<x>` gets 405
with `Allow: GET, HEAD, PUT, DELETE` and `{"message": "The POST method is not supported for route api/chapters/<x>. Supported methods: GET, HEAD, PUT, DELETE."}`.

Path handling (applies to every route; captured in slice 3.2):

- The first global middleware, `ValidatePathEncoding`, answers 400
  `{"message": "Malformed URL."}` when `rawurldecode($request->path())` isn't
  valid UTF-8 (`/api/chapters/%FF`). It runs before CORS, the post-size
  check, routing and auth, so the response has `Cache-Control` but no CORS
  headers.
- `UriValidator` matches routes against `rawurldecode(rtrim(pathInfo, '/'))`.
  `/api/chapters/` is the index, `x/` is `x`, `/api/%63hapters` is the
  index, and `/api/chapters%2Freorder` is the reorder route. But `x%2F`
  (trimmed raw, then decoded to `x/`) and `a%2Fb` match no route.
- Route parameters are decoded once: `a%20b` → `a b`, `a%2541` → `a%41`,
  `%` → `%`, and `+` stays `+`.
- Route 404 and 405 messages quote the raw path (`$request->path()`, trimmed
  of slashes): `The route api/chapters/a%2Fb could not be found.`

Rust: `server/app/src/paths.rs`; contract test
`framework::paths_are_decoded_like_laravel`.

### 1.2 Framework and web routes

| # | Method | URI | Source | Used by the SPA? |
|---|---|---|---|---|
| F1 | GET/HEAD | `/` | `routes/web.php`, Blade `welcome` (scaffold page, uses `@vite` only if `public/build/manifest.json` or `public/hot` exists) | No |
| F2 | GET/HEAD | `/up` | `bootstrap/app.php` `health: '/up'` | No (health check) |
| F3 | GET/HEAD | `/sanctum/csrf-cookie` | Sanctum, `web` group | No (the SPA uses bearer tokens) |
| F4 | GET/HEAD | `/storage/{path}` | `filesystems.disks.local.serve = true`, signed-URL serving of `storage/app/private` | No |
| F5 | PUT | `/storage/{path}` | same, signed upload | No |

### 1.3 Chapter payload validation (`ChapterController::validatePayload`)

```
id                               required|string|max:120|regex:/^[a-z0-9-]+$/ (+ unique:chapters,id on create)
number                           required|integer|min:1
section                          required|string|max:120
title                            required|string|max:200
titleEm                          nullable|string|max:200
position                         sometimes|nullable|integer
lesson                           present|array
lesson.intro                     nullable|string
lesson.formation                 present|array          lesson.formation.*  nullable|string
lesson.irregulars                present|array          .*  array ; .*.v, .*.forms  nullable|string
lesson.triggers                  present|array          .*  array ; .*.title/.body/.note  nullable|string
lesson.triggers.*.subcases       nullable|array         .*  array ; .*.label/.example  nullable|string
items                            present|array          items.*  array
items.*.type                     nullable|in:fill,qcm,transform
items.*.rule                     nullable|string
items.*.consolidation            nullable|array
```

**How the rules behave (captured in Phase 1, refined in slice 3.3).** The
header of `server/contract/tests/validation.rs` gives the full list:

- Errors come in rule declaration order, except that every rule without `*`
  comes first (`items` before `lesson.formation.0`). Each wildcard rule is
  then expanded over the data in turn. For a rule ending in `*`, scalar,
  `null` and empty children come before non-empty arrays and objects
  (`formation: [["a"], 5]` reports `.1` before `.0`). This is
  `ValidationRuleParser::explodeRules` plus
  `ValidationData::initializeAndGatherData`.
- `integer` follows PHP `filter_var(FILTER_VALIDATE_INT)`: `true`, `1.0` and `" 4 "` pass; `"03"`, `"3.0"`, `"1e3"` and `3.5` fail. Floats go through PHP's 14-digit string form, so `0.999999999999999` passes (as `"1"`) and is stored as `(int)`, which is 0.
- `max` counts characters for strings and items for arrays, whatever the value's type; the message still says "characters". `min` on `number` compares the exact decimal (brick/math), so `false`, `0.5` and `1e-7` fail both `integer` and `min`.
- `regex` sees PHP's string form of a number (`-5` and `1e13` match, `1e14` is `"1.0E+14"` and doesn't). `in` compares `(string)` values strictly.
- `required` treats `""`, whitespace-only strings, `null` and `[]` as missing.
- The middleware has already turned every blank string into `null`, and "blank" includes no-break spaces, soft hyphens and the other invisible characters `Str::trim` removes. So a blank `position` means no position, and a blank `order` entry fails `string`.
- Wildcard attributes keep their key in messages (`lesson.formation.a.b`, `items..rule`); others are `Str::snake`d (`title em`).

Rust: `server/app/src/validation.rs` (`validate`, `validation_view`),
`server/app/src/php.rs` (PHP number and string rules),
`server/app/src/chapters.rs` (`chapter_rules`, `to_row`). Differences: K1,
K2, K18.

**D-03 (spec/SPEC.md §4).** `$request->validate()` returns only the keys
that have rules. `items.*.before/hint/after/answers/prompt/context/options/
correctIndex/instruction/source` and everything inside `consolidation` have no
rule, so a save drops them. This is confirmed by the damaged
`subjonctif-present` row.

`toRow()` decides `position` like this:
- if the request gives a non-null `position`, use it;
- otherwise, if the chapter exists, keep its position;
- otherwise use `max(position) + 1`, or `0` when the table is empty.

It writes `payload = {id, number, section, title, titleEm, lesson, items}`,
taken from the validated data.

### 1.4 Error format (Laravel defaults, `withExceptions` is empty)

- 422 `ValidationException`:
  - shape: `{"message": "...", "errors": {"<dotted.field>": ["..."]}}`;
  - `message` is the first error, plus ` (and N more error)` or ` (and N more errors)` when there are more.
- 401: `{"message": "Unauthenticated."}`. The SPA clears its token on any 401
  outside `/api/auth/*`.
- Captured with `APP_DEBUG=false`. These bodies are pretty-printed by Laravel,
  so compare them as parsed JSON:

  | Status | When | Body |
  |---|---|---|
  | 404 | `findOrFail` on a chapter | `{"message": "No query results for model [App\\Models\\Chapter] <id>"}` |
  | 404 | `findOrFail` on an upload | `{"message": "No query results for model [App\\Models\\Upload] <id>"}` |
  | 404 | Unknown route | `{"message": "The route api/nope could not be found."}` |
  | 405 | Wrong method | `{"message": "The POST method is not supported for route api/chapters/x. Supported methods: GET, HEAD, PUT, DELETE."}`, plus an `Allow` header |
  | 413 | Body above `post_max_size` | `{"message": "The POST data is too large."}`. Under `php -S` an HTML PHP warning comes before it. |
  | 500 | Debug off | `{"message": "Server Error"}` |
- **Validation message details:**
  - camelCase attribute names are split into words: `titleEm` → "The title em field …".
  - Dotted paths stay as they are: "The lesson.formation field must be present."
  - Malformed JSON is treated as an empty body.
  - Form-encoded bodies are accepted as well as JSON.
- **Request input (captured in slice 3.1).** `$request->all()`:
  - **merges the query string** (K14);
  - reads JSON when Content-Type contains `/json` or `+json`, charset parameters allowed. JSON that isn't an object is cast to a PHP array (`["x"]` → `{0: "x"}`; a scalar `"x"` → `{0: "x"}`);
  - reads `application/x-www-form-urlencoded` and multipart text fields with PHP `parse_str` rules: `password[]=x` is an array, and the last duplicate key wins;
  - ignores any other Content-Type (`text/plain`, none).

  Rust copies all of this except the query string: `server/app/src/input.rs`.
- **Bearer parsing:** `Bearer  <t>` (two spaces) is 401. With `Bearer <t> `
  (trailing space) the result depends on the HTTP server (K15).
- **Token use:** each authenticated request sets `last_used_at` **and**
  `updated_at` to now (Eloquent `save()`).
- The SPA always sends `Accept: application/json`, so Laravel renders JSON.
  **Without it**, a guest gets a 500 HTML page: the auth redirect targets a
  `login` route that doesn't exist (K11).
- **CORS, as captured:**
  - A preflight from an allowed origin answers 204. It echoes the origin, the requested method in `Access-Control-Allow-Methods`, and the requested headers in `Access-Control-Allow-Headers`, with `Access-Control-Max-Age: 0`.
  - A preflight from an unknown origin answers 204 with no `Access-Control-Allow-Origin`.
  - Every response carries `Vary: Origin` and `Cache-Control: no-cache, private`.

## 2. Models

### 2.1 `App\Models\Chapter` → table `chapters`

| Aspect | Value |
|---|---|
| Key | `id` string, not incrementing (the slug) |
| Fillable | `id, number, section, title, title_em, payload, position` |
| Casts | `payload: array` (JSON), `number: integer`, `position: integer` |
| Timestamps | yes (`created_at`, `updated_at`, text `Y-m-d H:i:s`, UTC). **Never serialised** in responses. |
| Hidden, appends, scopes, soft deletes, observers, relations | none |
| Response shape (`shape()`, also used by export) | `{id, number, section, title, titleEm (from title_em), position, lesson: payload.lesson ?? null, items: payload.items ?? []}` |
| Payload read (captured in slice 3.2) | `json_decode($payload, true)`. Text that doesn't decode, or isn't an object, counts as empty. `??` falls back only on a missing key or `null`, so `items: false` comes back as `false`. PHP's array model shows in the response: `{}` → `[]`, objects keyed `"0".."n-1"` → lists, whole floats lose `.0`, integers above `PHP_INT_MAX` become floats, and nesting may go to 512 levels. Rust returns the stored JSON as it is instead (KNOWN_ISSUES K2, K16, K17). |
| Index order (captured in slice 3.2) | Eloquent's SQL is `select "id", "number", "section", "title", "title_em", "position" from "chapters" order by "position" asc, "number" asc`. SQLite scans `chapters_position_index` and sorts `number` in a temp B-tree. The seed has no full (position, number) ties. |

### 2.2 `App\Models\Upload` → table `uploads`

| Aspect | Value |
|---|---|
| Key | ULID via `HasUlids`: lowercase, generated on `creating` |
| Fillable | `path, original_name, mime, size`. **`id` is not fillable.** |
| Casts | none. `created_at` serialises through Carbon as `YYYY-MM-DDTHH:MM:SS.000000Z`. |
| Quirk | The controller builds the filename from `(string) Str::ulid()` (uppercase) and passes `id`, but `create()` drops it. So the row `id` is a **different** lowercase ULID than the one in `path`. Delete still works because it uses `basename(path)`. **Confirmed in Phase 1** (K3). |

### 2.3 `App\Models\User` → table `users`

- Fillable: `name, email, password`. Hidden: `password, remember_token`.
- Casts: `email_verified_at: datetime`, `password: hashed`.
- Traits: `HasApiTokens, HasFactory, Notifiable`. Notifiable is never used.
- Only one row is ever made: `editor@local`, via `firstOrCreate` in login and in
  `DatabaseSeeder`, with a bcrypt hash of random bytes or of a placeholder
  string. **No code ever verifies `users.password`.**

### 2.4 Sanctum `PersonalAccessToken` → table `personal_access_tokens`

Source: `vendor/laravel/sanctum/src/{HasApiTokens,PersonalAccessToken,Guard}.php`,
v4.3.2.

- **Create:**
  - plaintext = `token_prefix ('') + Str::random(40) + crc32b(those 40)`;
  - the row stores `sha256(plaintext)` and `abilities = ["*"]` (JSON);
  - `name = "admin-session"`, `tokenable_type = "App\Models\User"`, `expires_at = null`;
  - the client receives `"<row id>|<plaintext>"`.
- **Find:**
  - without `|`: look up `where token = sha256(input)`;
  - with `|`: split at the first `|`, `find(id)`, then `hash_equals(row.token, sha256(rest))`.
- **Valid:** `sanctum.expiration` (null) is not exceeded, and `expires_at` is
  null or in the future, and the tokenable exists. Then `last_used_at = now()`
  is saved on every authenticated request.
- Local state: 11 rows. They are dev tokens, but the brief says to keep existing tokens valid.

### 2.5 Schema (SQLite DDL, `sqlite3 .schema`)

Tables come from `backend/database/migrations/`:

- `users`, `password_reset_tokens`, `sessions`;
- `cache`, `cache_locks`;
- `jobs`, `job_batches`, `failed_jobs`;
- `personal_access_tokens`, `chapters`, `uploads`;
- `migrations` (Laravel's own).

App-relevant DDL:

```sql
CREATE TABLE "chapters" ("id" varchar not null, "number" integer not null, "section" varchar not null,
  "title" varchar not null, "title_em" varchar, "payload" text not null, "position" integer not null default '0',
  "created_at" datetime, "updated_at" datetime, primary key ("id"));
CREATE INDEX "chapters_position_index" on "chapters" ("position");
CREATE TABLE "uploads" ("id" varchar not null, "path" varchar not null, "original_name" varchar not null,
  "mime" varchar not null, "size" integer not null, "created_at" datetime, "updated_at" datetime, primary key ("id"));
CREATE UNIQUE INDEX "uploads_path_unique" on "uploads" ("path");
CREATE TABLE "personal_access_tokens" ("id" integer primary key autoincrement not null, "tokenable_type" varchar not null,
  "tokenable_id" integer not null, "name" text not null, "token" varchar not null, "abilities" text,
  "last_used_at" datetime, "expires_at" datetime, "created_at" datetime, "updated_at" datetime);
-- + unique(token), index(tokenable_type, tokenable_id), index(expires_at)
CREATE TABLE "users" ("id" integer primary key autoincrement not null, "name" varchar not null, "email" varchar not null,
  "email_verified_at" datetime, "password" varchar not null, "remember_token" varchar, "created_at" datetime, "updated_at" datetime);
-- + unique(email)
```

Local row counts (read-only):

| Table | Rows |
|---|---|
| chapters | 27 (damaged, see spec/SPEC.md §1) |
| uploads | 0 |
| personal_access_tokens | 11 |
| users | 1 |
| jobs, failed_jobs, sessions, cache | 0 |

## 3. Middleware, auth, policies

- **Global (Laravel 13 defaults):**
  - `TrustProxies` (none configured), `HandleCors`, `PreventRequestsDuringMaintenance`;
  - `ValidatePostSize` (413 above PHP `post_max_size`, 8M locally);
  - `TrimStrings` (skips `current_password`, `password`, `password_confirmation`);
  - `ConvertEmptyStringsToNull`.
- **`TrimStrings` + `ConvertEmptyStringsToNull` = D-04.**
  - Spaces in `before`/`after` are lost.
  - An empty answer `""` becomes `null`, as in D-08's "no article" item.
- **Guards:**
  - `auth:sanctum` tries the `web` guard first. There is no session in the `api` group, so that always misses.
  - Then it tries the bearer token.
- **The docs disagree with the code.** The project `CLAUDE.md` says the SPA
  uses a "cookie-based session against `backend/`". In fact
  `src/admin/api.js` sends `Authorization: Bearer` with the token kept in
  `localStorage`, and `backend/bootstrap/app.php` never enables
  `statefulApi()`. The `CLAUDE.md` fix waits on plan Q1.
- Policies, gates, Form Requests: **none**. Validation is inline in the controllers.
- Rate limiters: **none**. Login has no throttle.
- **CORS** (`config/cors.php`):
  - paths `api/*` and `sanctum/csrf-cookie`;
  - origins `http://localhost:5173`, `http://127.0.0.1:5173`, `http://localhost:4173`, plus the optional `FRONTEND_ORIGIN`;
  - methods and headers `*`, no exposed headers, `max_age 0`, `supports_credentials false`.

## 4. Sessions, cookies, CSRF, password flows

`SESSION_DRIVER=database` is configured, but no API route starts a session.

None of these exist:
- remember-me;
- password reset routes, although the `password_reset_tokens` table exists;
- email verification.

**Nothing to carry over except Sanctum tokens.**

## 5. Queues, scheduler, events, mail, broadcasting

| Concern | State |
|---|---|
| Queue | `QUEUE_CONNECTION=database`, 0 jobs, no `Job` classes |
| Scheduler | none. `routes/console.php` has only the scaffold `inspire` command. |
| Events / listeners / notifications | none |
| Mail | `MAIL_MAILER=log`, no Mailables |
| Broadcasting | `log`, unused |

## 6. Files, cache, localisation, config

- **Uploads:**
  - Location: `realpath(backend/..)/public/uploads/<ULID>.<ext>`. That is the SPA's `public/` folder, which is committed and shipped with the static site.
  - The extension comes from the sniffed MIME (`png`, `jpg`, `webp`, `gif`, `svg`), falling back to the client extension.
  - A path guard checks `realpath` containment.
  - Public URL: `rtrim(FRONTEND_BASE_URL, '/') . '/' . path`.
  - **Captured in slice 3.4** (contract tests `uploads::which_part_is_the_file`, `sniffing_edges_and_php_names`, `original_names`, `delete_when_file_is_gone`):
    - The validator sees `array_replace_recursive($input, $files, $input)`, so a text field named `file` beats a file part of that name. Of several `file` parts the last wins. A part with `filename=""` is no file (required fails). `file[]`/`file[x]` parts make an array: "must be a file" plus the type message. A part with only `filename*=` is a text field.
    - A non-file value fails `file` and `mimetypes`, and `max:5120` counts its characters ("… 5120 characters.").
    - `finfo` (libmagic 5.46) types: PNG needs the signature plus the IHDR chunk header (16 bytes); JPEG needs `FF D8 FF` plus one byte; GIF needs `GIF8`; WebP needs `RIFF????WEBP`. An empty file is `application/x-empty` and fails the type rule.
    - `shouldBlockPhpUpload`: a client extension of `php`, `php3`–`php8`, `phtml` or `phar` (trimmed, any case) fails the type rule whatever the bytes.
    - `original_name` is the client name after the last `/` or `\`, with `\"` unescaped.
    - PHP's multipart parser (`rfc1867.c`) matches the `name` and `filename` parameters in any case (`Name=`, `FILENAME=`), accepts `'…'` quotes, ignores `filename*=`, and keeps file-name bytes as sent. A non-UTF-8 name then breaks the list with a 500 (K19). Rust: `input::php_disposition`.
    - Delete answers 200 when the file is already gone.
    - Rust: `server/app/src/uploads.rs` (`upload_value`, `validate_file`, `sniff`).
  - Chapter HTML refers to `uploads/<file>`, and `src/RichText.jsx` prefixes `BASE_URL`.
- **Size limit:**
  - The rule says 5120 KB.
  - The effective limit is PHP `upload_max_filesize`: **2M** on the local CLI. Above that, the request fails with 422 `The file failed to upload.` (K7).
- **Export:**
  - Writes `realpath(backend/..)/src/chapters.json`.
  - Encoding: `json_encode(PRETTY_PRINT | UNESCAPED_UNICODE | UNESCAPED_SLASHES) . "\n"`, which gives a 4-space indent.
  - Write errors are suppressed (`@`) and reported only as `written: false`. The status stays 200.
  - The response also returns the absolute `path`.
- **Disks:** `local` (`storage/app/private`, served at F4/F5), `public`.
  The app uses neither: uploads bypass the Storage facade.
- **Cache:** configured as `database`, never used by the app.
- **Localisation:** `APP_LOCALE=en`. Validation messages come from Laravel's
  bundled English. There is no `lang/` folder.
- **Timezone:** `UTC` (hard-coded in `config/app.php`).

### `env()` keys the app depends on

| Key | Used for | Rust equivalent |
|---|---|---|
| `EDITOR_PASSWORD` | Login comparison. Empty means every login fails. | same name |
| `FRONTEND_BASE_URL` | Upload URLs (default `http://localhost:5173/learning-french`) | same name |
| `FRONTEND_ORIGIN` | Extra CORS origin (optional) | same name |
| `APP_KEY` | Not used by any API path: no encryption, cookies or sessions in play | not needed |
| `APP_DEBUG` | Error body detail | replaced by `RUST_LOG` + a fixed prod-style body |
| `DB_CONNECTION` / `DB_DATABASE` | SQLite file (default `database/database.sqlite`) | `DATABASE_URL` |
| `SANCTUM_TOKEN_PREFIX` | Token prefix (default empty) | same name, default empty |
| `APP_URL` | Sanctum stateful domains only, which are unused | not needed |

The rest of `.env.example` (mail, Redis, AWS, logs) is scaffold and unused.

## 7. Packages (`backend/composer.json`)

| Package | Purpose here | Rust equivalent |
|---|---|---|
| `laravel/framework` 13.35.0 | Everything | axum, tower-http, sqlx, serde_json |
| `laravel/sanctum` 4.3.2 | Bearer tokens | own module: `sha2`, `crc32fast`, `rand`, `subtle`/constant-time eq |
| `laravel/tinker` | REPL (dev) | none needed |
| dev: `pail`, `pao`, `pint`, `collision`, `mockery`, `faker`, `phpunit` | Dev tooling | `cargo fmt`, `clippy`, `cargo test` |

There are no third-party integrations: no HTTP clients, payments, storage or
mail providers.

## 8. Vite setup

- **SPA (root):**
  - `vite.config.js`: `@vitejs/plugin-react`, `base: "/learning-french/"`.
  - Single entry `index.html` → `src/main.jsx`. The admin is lazy-loaded when the hash starts with `#/admin`.
  - Env vars:
    - `VITE_API_URL`, admin only, default `http://localhost:8000`;
    - `BASE_URL`, built in, used by `RichText.jsx`.
  - Built by `npm run build` → `dist/`, deployed by `.github/workflows/` to GitHub Pages.
  - Dev: `npm run dev` on `:5173`. Prod: static files, no server. **Laravel never serves the SPA.**
- **Backend's own Vite:**
  - `backend/vite.config.js` with `laravel-vite-plugin`, Tailwind 4 and Vite ^8.
  - Inputs `resources/css/app.css` and `resources/js/app.js`, used only by `welcome.blade.php` (F1).
  - No manifest is built. This has no SPA role.

## 9. Tests

- `backend/tests/Feature/ExampleTest.php` (`GET /` → 200) and
  `tests/Unit/ExampleTest.php` (`assertTrue(true)`).
- `phpunit.xml`: SQLite `:memory:`, array cache and session, sync queue.
- Result on 2026-11-19 (`composer test`):
  `{"tool":"phpunit","result":"passed","tests":2,"passed":2,"assertions":2,"duration_ms":89}`.
- **No API route has a PHPUnit test.** Since Phase 1, the oracle is the
  black-box contract suite `server/contract/`: 79 HTTP tests (as of slice 3.4), all green
  against Laravel. See MIGRATION_PLAN.md §4.
- SPA: no test runner. `spec/tools/check.mjs` checks content (Node, no deps).

## 10. Behaviours that look like bugs (candidates for `KNOWN_ISSUES.md`)

| # | Behaviour | Source |
|---|---|---|
| K1 | Validation drops every exercise field except `type`, `rule` and `consolidation` | D-03, §1.3 |
| K2 | `TrimStrings` and `ConvertEmptyStringsToNull` change content | D-04, §3 |
| K3 | The upload row `id` differs from the ULID in the filename | §2.2 |
| K4 | SVG uploads are accepted and served from the public site origin (stored-XSS vector) | §1.1 R10 |
| K5 | Login has no rate limit, and every login makes a never-expiring token that is never cleaned up | §2.4, §3 |
| K6 | The export response leaks the absolute filesystem path. A failed write still returns 200. | §6 |
| K7 | The effective upload cap (2M) is below the validation rule (5120 KB) | §6 |
| K8 | `reorder` silently ignores unknown ids, and doesn't require the full set (gaps or duplicate positions are possible) | §1.1 R8 |
| K9 | `number` is not unique (D-10). The placeholder chapter collides with number 7. | spec/SPEC.md §4 |
| K10 | `payload` stores the client's raw `number` (a string `"3"` stays a string) while the column is cast to int | §1.3 |

Phase 1 confirmed K1–K10 and added K11–K13. Status, decisions and test names
are in [KNOWN_ISSUES.md](KNOWN_ISSUES.md).
