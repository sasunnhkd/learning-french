#!/usr/bin/env bash
# Run the contract suite (or any command) against a sandboxed copy of the
# Laravel backoffice API. See docs/migration/MIGRATION_PLAN.md (Phase 1).
#
# Why a sandbox: POST /api/export writes <backend>/../src/chapters.json and
# POST /api/uploads writes <backend>/../public/uploads/. Running against the
# real checkout would overwrite committed content. The sandbox mirrors that
# layout in a temp dir:
#   $SANDBOX/backend/              copy of backend/ (vendor included, no .env, no database.sqlite)
#   $SANDBOX/src/chapters.json     copy of the authoritative content
#   $SANDBOX/public/uploads/       empty
# The damaged backend/database/database.sqlite is never read: a fresh SQLite is
# migrated and seeded from src/chapters.json.
#
# Usage:
#   server/contract/run-laravel.sh                 # cargo test -p contract (serial)
#   server/contract/run-laravel.sh -- CMD ARGS...  # run CMD with the env below instead
#
# Env passed to the command:
#   BASE_URL, EDITOR_PASSWORD, TARGET=laravel, SANDBOX_ROOT, CONTRACT_DB
#
# Server rules (global): binds 127.0.0.1, killed at a hard deadline
# (CONTRACT_DEADLINE seconds, default 600) because `php -S` has no idle hook.
# Set KEEP_SANDBOX=1 to keep the temp dir for inspection.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO="$(cd "$SCRIPT_DIR/../.." && pwd)"
PORT="${CONTRACT_PORT:-8791}"
DEADLINE="${CONTRACT_DEADLINE:-600}"
PHP="${PHP:-php}"

if [ "${1:-}" = "--" ]; then
  shift
  CMD=("$@")
else
  CMD=(cargo test --manifest-path "$REPO/server/Cargo.toml" -p contract -- --test-threads=1 "$@")
fi

[ -d "$REPO/backend/vendor" ] || { echo "backend/vendor missing: run composer install in backend/ first" >&2; exit 1; }
[ -f "$REPO/src/chapters.json" ] || { echo "src/chapters.json missing" >&2; exit 1; }
if curl -s -o /dev/null --max-time 1 "http://127.0.0.1:$PORT/"; then
  echo "port $PORT already in use; set CONTRACT_PORT" >&2
  exit 1
fi

SANDBOX="$(mktemp -d "${TMPDIR:-/tmp}/grammaire-contract.XXXXXX")"
SANDBOX="$(cd "$SANDBOX" && pwd -P)"
case "$SANDBOX/" in
  "$REPO"/*) echo "refusing: sandbox $SANDBOX is inside the repo" >&2; exit 1 ;;
esac

SERVER_PID=""
WATCHDOG_PID=""
cleanup() {
  # Kill the watchdog's `sleep` child too: it inherits stdout and would keep a
  # caller's pipe (`run-laravel.sh | grep`) open until the deadline.
  [ -n "$WATCHDOG_PID" ] && pkill -P "$WATCHDOG_PID" 2>/dev/null || true
  [ -n "$WATCHDOG_PID" ] && kill "$WATCHDOG_PID" 2>/dev/null || true
  [ -n "$SERVER_PID" ] && kill "$SERVER_PID" 2>/dev/null || true
  if [ "${KEEP_SANDBOX:-0}" = "1" ]; then
    echo "sandbox kept: $SANDBOX" >&2
  else
    rm -rf "$SANDBOX"
  fi
}
trap cleanup EXIT

mkdir -p "$SANDBOX/src" "$SANDBOX/public/uploads"
cp "$REPO/src/chapters.json" "$SANDBOX/src/chapters.json"
rsync -a \
  --exclude '/.env' \
  --exclude '/database/database.sqlite' \
  --exclude '/storage/logs/*.log' \
  --exclude '/node_modules/' \
  --exclude '/public/hot' \
  "$REPO/backend/" "$SANDBOX/backend/"
# Seed from the authoritative content, never from the damaged DB.
cp "$REPO/src/chapters.json" "$SANDBOX/backend/database/seeders/chapters.json"

EDITOR_PASSWORD="$(od -An -N16 -tx1 /dev/urandom | tr -d ' \n')"
DB="$SANDBOX/backend/database/contract.sqlite"
: > "$DB"
cat > "$SANDBOX/backend/.env" <<EOF
APP_NAME=Laravel
APP_ENV=local
APP_KEY=
APP_DEBUG=false
APP_URL=http://127.0.0.1:$PORT
EDITOR_PASSWORD=$EDITOR_PASSWORD
FRONTEND_BASE_URL=http://localhost:5173/learning-french
LOG_CHANNEL=single
LOG_LEVEL=debug
DB_CONNECTION=sqlite
DB_DATABASE=$DB
SESSION_DRIVER=array
CACHE_STORE=array
QUEUE_CONNECTION=sync
MAIL_MAILER=array
EOF

(
  cd "$SANDBOX/backend"
  "$PHP" artisan key:generate --force --no-interaction >/dev/null
  "$PHP" artisan config:clear --no-interaction >/dev/null
  "$PHP" artisan migrate --seed --force --no-interaction >/dev/null
)

# Same process `php artisan serve` starts (ServeCommand::serverCommand), run
# directly so there is a single PID to kill.
(
  cd "$SANDBOX/backend/public"
  exec "$PHP" -S "127.0.0.1:$PORT" "$SANDBOX/backend/vendor/laravel/framework/src/Illuminate/Foundation/resources/server.php"
) >"$SANDBOX/server.log" 2>&1 &
SERVER_PID=$!
( sleep "$DEADLINE"; echo "contract: hard deadline ${DEADLINE}s reached, stopping server" >&2; kill "$SERVER_PID" 2>/dev/null ) &
WATCHDOG_PID=$!

for _ in $(seq 1 50); do
  if curl -s -o /dev/null --max-time 1 "http://127.0.0.1:$PORT/up"; then break; fi
  sleep 0.2
done
curl -s -o /dev/null --max-time 2 "http://127.0.0.1:$PORT/up" || { echo "Laravel did not start; log:" >&2; cat "$SANDBOX/server.log" >&2; exit 1; }

export BASE_URL="http://127.0.0.1:$PORT"
export EDITOR_PASSWORD
export TARGET=laravel
export SANDBOX_ROOT="$SANDBOX"
export CONTRACT_DB="$DB"

status=0
"${CMD[@]}" || status=$?
exit "$status"
