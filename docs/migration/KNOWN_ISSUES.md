# Known issues in the Laravel backoffice API

These are Laravel behaviours that look like bugs. Phase 1 captured each one
from the running app: a sandbox copy, with `APP_DEBUG=false`, seeded from
`src/chapters.json`.

Each issue has a status:

- **Approved fix**: you decided it. Rust differs on purpose.
- **Default fix** / **Default: accept**: the plan's default answer, waiting for you to confirm. Seven were approved on 2026-11-20, and K14 and K15 later the same day. K16 and K17 were approved on 2026-11-22, and K18 and K19 the same day. None is pending.
- **Keep**: Rust copies Laravel.

The contract suite in `server/contract/` switches its expectation on
`TARGET` only for the tests listed here, through `Ctx::expect_by_target`.

| # | Behaviour (Laravel, as captured) | Status | Rust behaviour | Contract test |
|---|---|---|---|---|
| K1 | **D-03.** `validate()` keeps only the keys that have rules. A save drops every exercise field except `type`, `rule` and `consolidation`, and every unknown key in `lesson`. It also turns `items` from an array into a JSON **object** with the keys in reverse (`{"1": …, "0": …}`). | **Approved fix** (Q2) | Stores `lesson` and `items` exactly as sent | `chapters_write::store_keeps_payload_exactly`, `round_trip_of_every_seed_chapter` |
| K2 | **D-04.** The global `TrimStrings` + `ConvertEmptyStringsToNull` trim every string (`"Il faut que tu "` → `"Il faut que tu"`) and turn `""` into `null`. That covers answers (`["sois", ""]` → `["sois", null]`), `titleEm`, `intro` and `formation[]`. An empty object `{}` also comes back as `[]` (PHP arrays). | **Approved fix** (Q2 + Q2a, everywhere) | No trimming or nulling anywhere in what is stored. `{}` stays `{}`. Validation still sees a blank string (spaces, and also no-break spaces, soft hyphens and the other characters `Str::trim` removes) as `null`, as Laravel's does: `required` rejects it, `nullable` accepts it, and a blank `position` means no position (`server/app/src/validation.rs` `validation_view`). Non-blank strings are validated untrimmed (K18). Reads follow from this (slice 3.2): `show` returns the stored `lesson`/`items` JSON as it is, so `{}` and objects keyed `"0".."n-1"` come back as objects, where Laravel's `array` cast turns them into `[]` and lists. Text Laravel stored is already in PHP's shape, so seeded chapters read the same on both. | `chapters_write::store_keeps_payload_exactly`, `store_validation_whitespace_only_is_missing` and `validation::blank_strings_count_as_null` (same on both), `chapters_read::show_returns_stored_payload` |
| K3 | The upload row `id` is a lowercase ULID that **differs** from the uppercase ULID in the file name, because `id` isn't fillable and `HasUlids` makes a new one. Example: `id 01m7tc556mfhetf4yth89pqbe8`, file `01M7TC556MFHETF4YTH89PQBE7.png`. Delete still works through `path`. | **Approved fix** (Q7, 2026-11-20) | `id` = the file-name ULID, in lowercase | `uploads::upload_list_delete_png` |
| K4 | SVG is accepted as `image/svg+xml`, `<script>` included, and lands in the static site's `public/uploads/`. That is a stored-XSS vector. | **Approved fix** (Q8, 2026-11-20: drop SVG) | 422 `The file field must be a file of type: image/png, image/jpeg, image/webp, image/gif.` | `uploads::svg_upload`, `upload_validation`, `types_are_sniffed_from_content` |
| K5 | Login has no rate limit. Every login makes a token that never expires and is never cleaned up. | **Approved** (Q9, 2026-11-20): rate-limit login in Rust, but configurable and **off for contract runs** (the suite logs in about 70 times). Token expiry unchanged. | Same token behaviour | `auth::every_login_makes_a_new_token` (same on both) |
| K6 | The export response returns the **absolute** path (`/…/src/chapters.json`). A failed write still answers 200, with `written: false`. | **Approved fix** (Q10, 2026-11-20) for `path`. **Keep** `written:false` + 200. | `path: "src/chapters.json"` | `export::export_writes_file_and_returns_chapters`, `export_reports_unwritable_file` (same on both) |
| K7 | Uploads above PHP `upload_max_filesize` (2M locally) fail with 422 `The file failed to upload.`, although the rule says `max:5120` (KB). | **Approved fix** (Q15, 2026-11-20) | Accepts up to 5120 KB. Above that: 422 `The file field must not be greater than 5120 kilobytes.` (Laravel's `max.file` string from `vendor/laravel/framework/src/Illuminate/Translation/lang/en/validation.php`; Laravel itself never reaches it under `php -S`) | `uploads::large_uploads` |
| K8 | `reorder` silently ignores unknown ids and doesn't require the full list, so gaps and duplicate positions are possible. An **object** `order` uses its keys as positions, stored with SQLite's integer affinity: `"7"` → 7, `"05"` → 5, `"1.5"` → 1.5 (read as 1), and `"x"` stays text (read as 0, sorted after every number; the next new chapter then gets position 1). Found in slice 3.3. | **Keep** | Same: positions are bound as text, so SQLite stores what PDO stores, and reads use PHP's `(int)` | `chapters_write::reorder_sets_positions_by_index`, `reorder_with_object_keys` |
| K9 | `number` is not unique (D-10). The seeded placeholder `grammaire-essentiel` shares number 7 and position 0. | **Keep** | Same | `chapters_read::index_shape_order_and_values` |
| K10 | `payload` stores the client's raw `number` (the string `"3"` stays a string), while the response reads the integer column. | **Keep** (not visible in any response) | Same response; the payload copy is internal | `chapters_write::store_appends_position_and_casts_number` |
| K11 | A guest request **without** `Accept: application/json` gets **500 HTML**: Laravel tries to redirect to a `login` route that doesn't exist. The SPA always sends the header, so it never sees this. | **Approved fix** (Q16, 2026-11-20) | Always 401 `{"message":"Unauthenticated."}` | `framework::guest_without_accept_header` |
| K12 | Scaffold routes answer even though nothing uses them: `GET /` (Blade welcome, 200 HTML) and `GET /sanctum/csrf-cookie` (204). | **Approved fix** (Q6, 2026-11-20: drop) | 404 | `framework::scaffold_routes` |
| K14 | Laravel's `$request->all()` merges the **query string** into the input. `POST /api/auth/login?password=…` with an empty body logs in, so the password lands in access and proxy logs. Found in slice 3.1. | **Approved fix** (2026-11-20) | Every route's input comes from the body only: JSON, form or multipart (`server/app/src/input.rs`). A query-string password gets 422 `The password field is required.` | `auth::login_password_in_query_string` |
| K15 | `Authorization: Bearer <token> ` with a **trailing** space: PHP's built-in server keeps the space, so the token doesn't match (401). This is the HTTP stack, not Laravel code. | **Approved: accept** (2026-11-20) | hyper strips the whitespace around header values (RFC 9110 §5.5), so the token is valid (200). A leading extra space (`Bearer  x`) is still 401 on both. | `auth::bearer_with_extra_spaces` |
| K16 | Numbers inside `payload` go through PHP floats and ints: a whole float loses its `.0` (`1.0` → `1`, `1e2` → `100`), and an integer above `PHP_INT_MAX` (`12345678901234567890`) becomes a float (`1.2345678901234567e+19`). Found in slice 3.2. | **Approved: accept** (2026-11-22) | Numbers come back as stored: `1.0`, `100.0`, `12345678901234567890` exactly. A JavaScript client parses the same values either way (`1.0 === 1`), except integers above 2^53, which JS rounds in both cases. Export writes the same values as stored, so its file differs from Laravel's for three kinds of number: whole floats (`1.0`, where PHP writes `1`), exponent forms (`1e-7` and `1e+25`, where PHP writes `1.0e-7` and `1.0e+25`), and integers above `PHP_INT_MAX` (PHP writes `1.2345678901234567e+19`). The SPA's `JSON.stringify` never sends a whole float or an exponent form, the seed has none of the three, and both exported files are byte-identical (slice 3.5, `server/contract/export-cmp.sh`). | `chapters_read::show_returns_stored_payload` |
| K17 | `json_decode` accepts 512 levels of nesting. Found in slice 3.2. | **Approved: accept** (2026-11-22) | serde_json stops at 128 levels, so a payload nested deeper reads as empty (`lesson: null`, `items: []`) and a request body nested deeper reads as empty input. The seed nests at most 6 levels. The SPA's editor (`src/admin/ChapterEditor.jsx`) builds fixed shapes a few levels deep and stores rich text as HTML strings, so it can't get near 128. Raising the limit needs serde_json's `unbounded_depth` feature, and then a deep enough body could overflow a worker stack and abort the server. | `chapters_read::show_deeply_nested_payload` |
| K18 | `TrimStrings` trims every string before validation, so padded values pass and are saved trimmed: id `" abc "` or `"abc\n"` saves as `abc`, an item `type` of `" fill "` saves as `fill`, `number: "\f4"` saves 4, a title of 200 characters plus a trailing space fits `max:200`, an update of `abc` whose body says `" abc "` succeeds, and reorder with `" abc "` moves `abc`. Found in slice 3.3. | **Approved: accept** (2026-11-22; follows from Q2a: Rust never trims) | Values are validated as sent: those saves get 422 (`The id field format is invalid.`, `The selected items.0.type is invalid.`, `"\f4"` gets both `The number field must be an integer.` and `… must be at least 1.`, the max-length message), and reorder ignores `" abc "` (no such id). PCRE's `$` would also accept `"abc\n"`; Rust's id check doesn't copy that. The SPA sends ids from its own list and types from a select, so it never pads them. | `chapters_write::untrimmed_values_are_judged_as_sent` |
| K19 | A multipart file name that isn't UTF-8 (`filename="caf\xe9.png"`, Latin-1) is stored as raw bytes in `original_name`. From then on `GET /api/uploads` answers 500 `Server Error` (`json_encode` fails on the name) until that upload is deleted. Found in slice 3.4. | **Approved fix** (2026-11-22) | The name is stored with U+FFFD in place of the bad bytes (`caf�.png`), and the list keeps working. Browsers send UTF-8 file names, so the SPA never hits this. | `uploads::non_utf8_file_name` |
| K13 | Laravel's export writes 4-space pretty JSON, but the committed `src/chapters.json` uses 2 spaces (it came from `scripts/dump-chapters.mjs`). So the first export rewrites the whole file's indentation. | **Keep** (Rust matches Laravel byte for byte) | Same bytes | `export::export_writes_file_and_returns_chapters` |

Behaviours that are environment details of `php -S`, recorded but not
asserted:

- The 413 body for an 8 MB+ request has a PHP startup warning in HTML
  before the JSON `{"message": "The POST data is too large."}`. The suite
  asserts the status only.
- Some error responses use `HTTP/1.0` on the status line.

## Proving the switch

Running the suite against Laravel with `TARGET=rust` fails exactly the
**16** tests that assert differences:

- K1/K2: 2 tests
- K2 + K16 (reads): 1 (`show_returns_stored_payload`)
- K3: 1
- K4: 3
- K6, K7, K11, K12, K14, K15, K17, K18, K19: 1 each

The other 64 of the 80 contract tests pass, which shows the shared tests make
no Laravel-only assumptions. Checked 2026-11-23, slice 3.5.

Correction: earlier docs and commit messages overcounted this list by one.
Phase 1 had 10 tagged tests, not 11; its first run had 9, not 10. The
per-issue lists were right; only the totals were wrong.

Command:

```bash
server/contract/run-laravel.sh -- env TARGET=rust cargo test --no-fail-fast --manifest-path server/Cargo.toml -p contract -- --test-threads=1
```

Export check (plan slice 3.5): both apps export from the same seed, and the
files must be byte-identical. Prints `export files are identical` or fails:

```bash
server/contract/export-cmp.sh
```
