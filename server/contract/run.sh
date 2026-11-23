#!/usr/bin/env bash
# Run the contract suite (or any command) against a sandboxed server:
#   run.sh laravel [ARGS...]   the Laravel app in backend/ (php -S)
#   run.sh rust    [ARGS...]   the Rust app in server/app (built first)
# run-laravel.sh and run-rust.sh are thin wrappers. See server/README.md and
# docs/migration/MIGRATION_PLAN.md (Phase 1, Phase 2).
#
# Why a sandbox: POST /api/export writes <root>/src/chapters.json and
# POST /api/uploads writes <root>/public/uploads/. Running against the real
# checkout would overwrite committed content. The sandbox mirrors that
# layout in a temp dir:
#   $SANDBOX/backend/              copy of backend/ (vendor included, no .env, no database.sqlite)
#   $SANDBOX/src/chapters.json     copy of the authoritative content
#   $SANDBOX/public/uploads/       empty
# Both targets use a database made by Laravel's own `migrate --seed` (fresh
# SQLite seeded from src/chapters.json), so the Rust app is tested against
# exactly the schema and rows Laravel produces. The damaged
# backend/database/database.sqlite is never read.
#
# ARGS:
#   (none)        laravel: the whole suite. rust: only the tests listed in
#                 server/contract/ported.txt (slices ported so far).
#   --all         the whole suite (useful for rust to see what is left)
#   -- CMD...     run CMD with the env below instead of cargo test
#   other         passed to the test harness (e.g. a name filter)
#
# Env passed to the command:
#   BASE_URL, EDITOR_PASSWORD, TARGET, SANDBOX_ROOT, CONTRACT_DB
#
# Server rules (global): binds 127.0.0.1 and is killed at a hard deadline
# (CONTRACT_DEADLINE seconds, default 600). CONTRACT_RELEASE=1 runs a release
# build of the Rust server (bench.sh). CONTRACT_FRONTEND_ORIGIN adds a CORS
# origin (a dev SPA on another port, for a smoke test). The Rust server also gets
# IDLE_TIMEOUT_SECS=600 and LOGIN_RATE_LIMIT=0 (the suite logs in ~70
# times). Set KEEP_SANDBOX=1 to keep the temp dir.
set -euo pipefail

TARGET="${1:-}"
case "$TARGET" in
  laravel|rust) shift ;;
  *) echo "usage: run.sh laravel|rust [--all | -- CMD... | test filters...]" >&2; exit 2 ;;
esac

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO="$(cd "$SCRIPT_DIR/../.." && pwd)"
PORT="${CONTRACT_PORT:-8791}"
DEADLINE="${CONTRACT_DEADLINE:-600}"
PHP="${PHP:-php}"
# Cargo reads server/.cargo/config.toml only when run from inside server/,
# and this script runs from anywhere: keep sqlx macros offline explicitly.
export SQLX_OFFLINE=true
CARGO_TEST=(cargo test --manifest-path "$REPO/server/Cargo.toml" -p contract --)

if [ "${1:-}" = "--" ]; then
  shift
  CMD=("$@")
elif [ "${1:-}" = "--all" ]; then
  shift
  CMD=("${CARGO_TEST[@]}" --test-threads=1 "$@")
elif [ "$TARGET" = "rust" ] && [ "$#" -eq 0 ]; then
  PORTED=()
  while IFS= read -r line; do
    line="${line%%#*}"; line="$(echo "$line" | tr -d '[:space:]')"
    [ -n "$line" ] && PORTED+=("$line")
  done < "$SCRIPT_DIR/ported.txt"
  [ "${#PORTED[@]}" -gt 0 ] || { echo "ported.txt lists no tests" >&2; exit 1; }
  CMD=("${CARGO_TEST[@]}" --test-threads=1 --exact "${PORTED[@]}")
else
  CMD=("${CARGO_TEST[@]}" --test-threads=1 "$@")
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

if [ "$TARGET" = "laravel" ]; then
  # Same process `php artisan serve` starts (ServeCommand::serverCommand),
  # run directly so there is a single PID to kill.
  (
    cd "$SANDBOX/backend/public"
    exec "$PHP" -S "127.0.0.1:$PORT" "$SANDBOX/backend/vendor/laravel/framework/src/Illuminate/Foundation/resources/server.php"
  ) >"$SANDBOX/server.log" 2>&1 &
else
  if [ "${CONTRACT_RELEASE:-0}" = "1" ]; then
    cargo build --quiet --release --manifest-path "$REPO/server/Cargo.toml" -p grammaire-server
    BIN="$REPO/server/target/release/grammaire-server"
  else
    cargo build --quiet --manifest-path "$REPO/server/Cargo.toml" -p grammaire-server
    BIN="$REPO/server/target/debug/grammaire-server"
  fi
  # cwd = sandbox root: no .env there, and REPO_ROOT is explicit anyway.
  (
    cd "$SANDBOX"
    exec env -i PATH="$PATH" HOME="$HOME" \
      DATABASE_URL="sqlite:$DB" \
      BIND_ADDR="127.0.0.1:$PORT" \
      EDITOR_PASSWORD="$EDITOR_PASSWORD" \
      FRONTEND_BASE_URL="http://localhost:5173/learning-french" \
      FRONTEND_ORIGIN="${CONTRACT_FRONTEND_ORIGIN:-}" \
      REPO_ROOT="$SANDBOX" \
      IDLE_TIMEOUT_SECS=600 \
      LOGIN_RATE_LIMIT=0 \
      RUST_LOG="${RUST_LOG:-info}" \
      "$BIN"
  ) >"$SANDBOX/server.log" 2>&1 &
fi
SERVER_PID=$!
( sleep "$DEADLINE"; echo "contract: hard deadline ${DEADLINE}s reached, stopping server" >&2; kill "$SERVER_PID" 2>/dev/null ) &
WATCHDOG_PID=$!

for _ in $(seq 1 50); do
  if curl -s -o /dev/null --max-time 1 "http://127.0.0.1:$PORT/up"; then break; fi
  sleep 0.2
done
curl -s -o /dev/null --max-time 2 "http://127.0.0.1:$PORT/up" || { echo "$TARGET server did not start; log:" >&2; cat "$SANDBOX/server.log" >&2; exit 1; }

export BASE_URL="http://127.0.0.1:$PORT"
export EDITOR_PASSWORD
export TARGET
export SANDBOX_ROOT="$SANDBOX"
export CONTRACT_DB="$DB"

status=0
"${CMD[@]}" || status=$?
exit "$status"
