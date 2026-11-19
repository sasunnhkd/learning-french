# Known issues in the Laravel backoffice API

These are Laravel behaviours that look like bugs. Phase 1 captured each one
from the running app: a sandbox copy, with `APP_DEBUG=false`, seeded from
`src/chapters.json`.

Each issue has a status:

- **Approved fix**: you decided it. Rust differs on purpose.
- **Default fix**: the plan's default answer. It holds until you override it.
- **Keep**: Rust copies Laravel.

The contract suite in `server/contract/` switches its expectation on
`TARGET` only for the tests listed here, through `Ctx::expect_by_target`.

| # | Behaviour (Laravel, as captured) | Status | Rust behaviour | Contract test |
|---|---|---|---|---|
| K1 | **D-03.** `validate()` keeps only the keys that have rules. A save drops every exercise field except `type`, `rule` and `consolidation`, and every unknown key in `lesson`. It also turns `items` from an array into a JSON **object** with the keys in reverse (`{"1": …, "0": …}`). | **Approved fix** (Q2) | Stores `lesson` and `items` exactly as sent | `chapters_write::store_keeps_payload_exactly`, `round_trip_of_every_seed_chapter` |
| K2 | **D-04.** The global `TrimStrings` + `ConvertEmptyStringsToNull` trim every string (`"Il faut que tu "` → `"Il faut que tu"`) and turn `""` into `null`. That covers answers (`["sois", ""]` → `["sois", null]`), `titleEm`, `intro` and `formation[]`. An empty object `{}` also comes back as `[]` (PHP arrays). | **Approved fix** (Q2 + Q2a, everywhere) | No trimming or nulling anywhere. `{}` stays `{}`. `required` still rejects whitespace-only strings, as Laravel's rule does. | `chapters_write::store_keeps_payload_exactly`, `store_validation_whitespace_only_is_missing` (same on both) |
| K3 | The upload row `id` is a lowercase ULID that **differs** from the uppercase ULID in the file name, because `id` isn't fillable and `HasUlids` makes a new one. Example: `id 01m7tc556mfhetf4yth89pqbe8`, file `01M7TC556MFHETF4YTH89PQBE7.png`. Delete still works through `path`. | **Default fix** (Q7) | `id` = the file-name ULID, in lowercase | `uploads::upload_list_delete_png` |
| K4 | SVG is accepted as `image/svg+xml`, `<script>` included, and lands in the static site's `public/uploads/`. That is a stored-XSS vector. | **Default fix** (Q8: drop SVG) | 422 `The file field must be a file of type: image/png, image/jpeg, image/webp, image/gif.` | `uploads::svg_upload`, `upload_validation`, `types_are_sniffed_from_content` |
| K5 | Login has no rate limit. Every login makes a token that never expires and is never cleaned up. | Q9 default: rate-limit login in Rust, but configurable and **off for contract runs** (the suite logs in about 60 times). Token expiry unchanged. | Same token behaviour | `auth::every_login_makes_a_new_token` (same on both) |
| K6 | The export response returns the **absolute** path (`/…/src/chapters.json`). A failed write still answers 200, with `written: false`. | **Default fix** (Q10) for `path`. **Keep** `written:false` + 200. | `path: "src/chapters.json"` | `export::export_writes_file_and_returns_chapters`, `export_reports_unwritable_file` (same on both) |
| K7 | Uploads above PHP `upload_max_filesize` (2M locally) fail with 422 `The file failed to upload.`, although the rule says `max:5120` (KB). | **Default fix** (Q15) | Accepts up to 5120 KB. Above that: 422 `The file field must not be greater than 5120 kilobytes.` (Laravel's `max.file` string from `vendor/laravel/framework/src/Illuminate/Translation/lang/en/validation.php`; Laravel itself never reaches it under `php -S`) | `uploads::large_uploads` |
| K8 | `reorder` silently ignores unknown ids and doesn't require the full list, so gaps and duplicate positions are possible. | **Keep** | Same | `chapters_write::reorder_sets_positions_by_index` |
| K9 | `number` is not unique (D-10). The seeded placeholder `grammaire-essentiel` shares number 7 and position 0. | **Keep** | Same | `chapters_read::index_shape_order_and_values` |
| K10 | `payload` stores the client's raw `number` (the string `"3"` stays a string), while the response reads the integer column. | **Keep** (not visible in any response) | Same response; the payload copy is internal | `chapters_write::store_appends_position_and_casts_number` |
| K11 | A guest request **without** `Accept: application/json` gets **500 HTML**: Laravel tries to redirect to a `login` route that doesn't exist. The SPA always sends the header, so it never sees this. | **Default fix** (new, see the plan's open list) | Always 401 `{"message":"Unauthenticated."}` | `framework::guest_without_accept_header` |
| K12 | Scaffold routes answer even though nothing uses them: `GET /` (Blade welcome, 200 HTML) and `GET /sanctum/csrf-cookie` (204). | **Default fix** (Q6: drop) | 404 | `framework::scaffold_routes` |
| K13 | Laravel's export writes 4-space pretty JSON, but the committed `src/chapters.json` uses 2 spaces (it came from `scripts/dump-chapters.mjs`). So the first export rewrites the whole file's indentation. | **Keep** (Rust matches Laravel byte for byte) | Same bytes | `export::export_writes_file_and_returns_chapters` |

Behaviours that are environment details of `php -S`, recorded but not
asserted:

- The 413 body for an 8 MB+ request has a PHP startup warning in HTML
  before the JSON `{"message": "The POST data is too large."}`. The suite
  asserts the status only.
- Some error responses use `HTTP/1.0` on the status line.

## Proving the switch

Running the suite against Laravel with `TARGET=rust` fails exactly the 11
tests that assert differences: K1/K2 ×2, K3, K4 ×3, K6, K7, K11, K12. The
other 46 contract tests pass. That shows the shared tests make no
Laravel-only assumptions. Command:

```bash
server/contract/run-laravel.sh -- env TARGET=rust cargo test --no-fail-fast --manifest-path server/Cargo.toml -p contract -- --test-threads=1
```
