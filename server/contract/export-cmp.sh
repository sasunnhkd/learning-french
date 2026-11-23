#!/usr/bin/env bash
# Plan slice 3.5 acceptance check: Laravel and Rust each export from the
# same seed (fresh sandbox per target, see run.sh), and the two
# src/chapters.json files must be byte-identical. Exits non-zero otherwise.
#   server/contract/export-cmp.sh
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUT_DIR="$(mktemp -d "${TMPDIR:-/tmp}/grammaire-export-cmp.XXXXXX")"
trap 'rm -rf "$OUT_DIR"' EXIT
STEP="$OUT_DIR/export-one.sh"
cat > "$STEP" <<'STEP_EOF'
#!/usr/bin/env bash
# Runs inside run.sh: log in, export, copy the written file to $OUT.
set -euo pipefail
token="$(curl -sf -H 'Accept: application/json' -H 'Content-Type: application/json' \
  --data-binary @- "$BASE_URL/api/auth/login" <<JSON | sed -n 's/.*"token":"\([^"]*\)".*/\1/p'
{"password":"$EDITOR_PASSWORD"}
JSON
)"
[ -n "$token" ] || { echo "login failed" >&2; exit 1; }
code="$(curl -s -o /dev/null -w '%{http_code}' -X POST -H 'Accept: application/json' \
  -H "Authorization: Bearer $token" "$BASE_URL/api/export")"
[ "$code" = 200 ] || { echo "export answered $code" >&2; exit 1; }
cp "$SANDBOX_ROOT/src/chapters.json" "$OUT"
STEP_EOF
chmod +x "$STEP"
for target in laravel rust; do
  "$SCRIPT_DIR/run.sh" "$target" -- env OUT="$OUT_DIR/$target.json" "$STEP"
done
if cmp "$OUT_DIR/laravel.json" "$OUT_DIR/rust.json"; then
  echo "export files are identical ($(wc -c < "$OUT_DIR/rust.json" | tr -d ' ') bytes)"
else
  exit 1
fi
