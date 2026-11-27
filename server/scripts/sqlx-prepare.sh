#!/usr/bin/env bash
# Regenerate (or --check) the sqlx offline query data in server/app/.sqlx
# without sqlx-cli: the query macros save their metadata when compiled with
# a live DATABASE_URL and SQLX_OFFLINE_DIR set. The database is a throwaway
# file built from app/migrations/ (Laravel's exact schema), never real data.
#
#   server/scripts/sqlx-prepare.sh          write server/app/.sqlx
#   server/scripts/sqlx-prepare.sh --check  fail if server/app/.sqlx is stale
#
# Run it after adding or changing any sqlx::query!/query_scalar! call.
set -euo pipefail
SERVER="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WORK="$(mktemp -d "${TMPDIR:-/tmp}/grammaire-sqlx.XXXXXX")"
trap 'rm -rf "$WORK"' EXIT

DB="$WORK/schema.sqlite"
for f in "$SERVER"/app/migrations/*.sql; do sqlite3 "$DB" < "$f"; done
mkdir -p "$WORK/sqlx"

cd "$SERVER"
# Force the macros to re-expand so every query writes its data file.
cargo clean --quiet -p grammaire-server
SQLX_OFFLINE=false DATABASE_URL="sqlite:$DB" SQLX_OFFLINE_DIR="$WORK/sqlx" \
  cargo check --quiet -p grammaire-server --all-targets

if [ "${1:-}" = "--check" ]; then
  if diff -r "$WORK/sqlx" "$SERVER/app/.sqlx" >/dev/null 2>&1; then
    echo "app/.sqlx is up to date"
  else
    echo "app/.sqlx is stale: run server/scripts/sqlx-prepare.sh" >&2
    diff -r "$WORK/sqlx" "$SERVER/app/.sqlx" >&2 || true
    exit 1
  fi
else
  rm -rf "$SERVER/app/.sqlx"
  cp -R "$WORK/sqlx" "$SERVER/app/.sqlx"
  echo "wrote $(ls "$SERVER/app/.sqlx" | wc -l | tr -d ' ') query files to app/.sqlx"
fi
