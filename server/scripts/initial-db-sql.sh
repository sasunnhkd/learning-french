#!/usr/bin/env bash
# Make the SQL for the server's first database: Laravel's own
# `migrate --seed` in a sandbox (server/contract/run.sh), seeded from the
# current src/chapters.json, dumped with `sqlite3 .dump`. Laravel's schema
# and its `migrations` table are included, so Laravel can take the file back
# for a rollback. No tokens; one user, the seeder's `editor@local` (its
# bcrypt hash is of random bytes and is never checked; login compares
# EDITOR_PASSWORD). You apply the file on the server (docs/migration/CUTOVER.md):
#
#   server/scripts/initial-db-sql.sh            -> server/ci-artifacts/grammaire-initial-<date>.sql
#   sqlite3 /var/lib/grammaire-server/database.sqlite < grammaire-initial-<date>.sql   (on the server)
set -euo pipefail
SERVER="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="$SERVER/ci-artifacts"
mkdir -p "$OUT_DIR"
OUT="$OUT_DIR/grammaire-initial-$(date -u +%Y%m%d).sql"
"$SERVER/contract/run.sh" laravel -- sh -c 'sqlite3 "$CONTRACT_DB" .dump' > "$OUT.tmp"
# run.sh may print its own lines; keep only the dump.
sed -n '/^PRAGMA foreign_keys=OFF;$/,/^COMMIT;$/p' "$OUT.tmp" > "$OUT"
rm -f "$OUT.tmp"
grep -q '^COMMIT;$' "$OUT" || { echo "dump incomplete: $OUT" >&2; exit 1; }
n="$(grep -c '^INSERT INTO "\?personal_access_tokens"\? ' "$OUT" || true)"
[ "$n" = 0 ] || { echo "unexpected token rows in $OUT" >&2; exit 1; }
users="$(grep '^INSERT INTO "\?users"\? ' "$OUT" || true)"
[ "$(printf '%s' "$users" | grep -c .)" = 1 ] && printf '%s' "$users" | grep -q "'editor@local'" \
  || { echo "expected exactly the seeder's editor@local user in $OUT" >&2; exit 1; }
echo "wrote $OUT ($(grep -c '^INSERT INTO "\?chapters"\? ' "$OUT") chapters)"
