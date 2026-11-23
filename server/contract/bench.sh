#!/usr/bin/env bash
# Phase 5 latency comparison: the same requests against the sandboxed
# Laravel app (`php -S`, single-threaded, as run.sh starts it) and a release
# build of the Rust app, each on a freshly seeded sandbox. Uses ApacheBench
# (`ab`, shipped with macOS and in Debian's apache2-utils). Prints p50, p95,
# p99 and requests per second per case.
#   server/contract/bench.sh            (N=500 requests per case)
#   N=2000 server/contract/bench.sh
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
command -v ab >/dev/null || { echo "ab (ApacheBench) is required" >&2; exit 1; }
STEP="$(mktemp "${TMPDIR:-/tmp}/grammaire-bench.XXXXXX")"
trap 'rm -f "$STEP"' EXIT
cat > "$STEP" <<'STEP_EOF'
#!/usr/bin/env bash
# Runs inside run.sh with BASE_URL, EDITOR_PASSWORD, TARGET set.
set -euo pipefail
N="${N:-500}"
token="$(curl -sf -H 'Accept: application/json' -H 'Content-Type: application/json' \
  --data-binary @- "$BASE_URL/api/auth/login" <<JSON | sed -n 's/.*"token":"\([^"]*\)".*/\1/p'
{"password":"$EDITOR_PASSWORD"}
JSON
)"
[ -n "$token" ] || { echo "login failed" >&2; exit 1; }
run() { # name concurrency path
  local out
  out="$(ab -q -n "$N" -c "$2" -H 'Accept: application/json' -H "Authorization: Bearer $token" "$BASE_URL$3" 2>&1)"
  local fail; fail="$(sed -n 's/^Failed requests: *//p' <<<"$out")"
  local non2xx; non2xx="$(sed -n 's/^Non-2xx responses: *//p' <<<"$out")"
  local rps; rps="$(sed -n 's/^Requests per second: *\([0-9.]*\).*/\1/p' <<<"$out")"
  local p50 p95 p99
  p50="$(sed -n 's/^ *50% *//p' <<<"$out")"; p95="$(sed -n 's/^ *95% *//p' <<<"$out")"; p99="$(sed -n 's/^ *99% *//p' <<<"$out")"
  printf '%-8s %-28s c=%-3s p50=%5sms p95=%5sms p99=%5sms rps=%8s failed=%s non2xx=%s\n' \
    "$TARGET" "$1" "$2" "$p50" "$p95" "$p99" "$rps" "$fail" "${non2xx:-0}"
}
run "index"          1  /api/chapters
run "index"          10 /api/chapters
run "show (largest)" 1  /api/chapters/subjonctif-present
run "show (largest)" 10 /api/chapters/subjonctif-present
run "up"             10 /up
STEP_EOF
chmod +x "$STEP"
"$SCRIPT_DIR/run.sh" laravel -- env N="${N:-500}" "$STEP"
CONTRACT_RELEASE=1 "$SCRIPT_DIR/run.sh" rust -- env N="${N:-500}" "$STEP"
