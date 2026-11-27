#!/usr/bin/env bash
# Rollback check (plan Phase 6: "Laravel reads the Rust-written DB"). The
# Rust app writes a database (a login token, a new chapter with every kind
# of content, an update, a reorder, an upload); then the Laravel app is
# started on a copy of that same file and must read it all back, accept the
# Rust-minted token (Q5), and write to it. Prints "rollback check passed".
#   server/contract/rollback-check.sh
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK="$(mktemp -d "${TMPDIR:-/tmp}/grammaire-rollback.XXXXXX")"
trap 'rm -rf "$WORK"' EXIT

cat > "$WORK/rust-writes.py" <<'PY'
import json, os, subprocess, sys, urllib.request, urllib.error
work = sys.argv[1]; base = os.environ["BASE_URL"]
def call(method, path, body=None, tok=None, ctype="application/json"):
    data = body if isinstance(body, (bytes, type(None))) else json.dumps(body).encode()
    req = urllib.request.Request(base + path, data=data, method=method)
    req.add_header("Accept", "application/json")
    if data is not None: req.add_header("Content-Type", ctype)
    if tok: req.add_header("Authorization", "Bearer " + tok)
    try:
        with urllib.request.urlopen(req) as r: return r.status, json.loads(r.read() or b"null")
    except urllib.error.HTTPError as e: return e.code, json.loads(e.read() or b"null")
st, b = call("POST", "/api/auth/login", {"password": os.environ["EDITOR_PASSWORD"]}); assert st == 200, b
tok = b["token"]
chapter = {
    "id": "rollback-check", "number": 99, "section": "  Padded  ", "title": "Rollback", "titleEm": "",
    "lesson": {"intro": "<p>é /   \"q\"</p>", "formation": ["a", ""], "irregulars": [{"v": "être", "forms": "x", "extra": 1}],
               "triggers": [{"title": "t", "subcases": [{"label": "l", "example": "e"}]}], "custom": {}},
    "items": [{"type": "fill", "before": "Il faut que tu ", "answers": ["sois", ""], "consolidation": {"type": "qcm", "options": ["a", "b"]}},
              {"type": "qcm", "prompt": "p", "options": ["x", "y"], "correctIndex": 1}],
}
st, b = call("POST", "/api/chapters", chapter, tok); assert st == 201, b
chapter["title"] = "Rollback updated"
st, b = call("PUT", "/api/chapters/rollback-check", chapter, tok); assert st == 200, b
st, b = call("POST", "/api/chapters/reorder", {"order": ["rollback-check"]}, tok); assert st == 200, b
png = b"\x89PNG\r\n\x1a\n\x00\x00\x00\x0dIHDR" + b"\x00" * 20
mp = (b'--XYZ\r\nContent-Disposition: form-data; name="file"; filename="r.png"\r\nContent-Type: image/png\r\n\r\n'
      + png + b"\r\n--XYZ--\r\n")
st, up = call("POST", "/api/uploads", mp, tok, "multipart/form-data; boundary=XYZ"); assert st == 201, up
st, shown = call("GET", "/api/chapters/rollback-check", tok=tok); assert st == 200, shown
st, uploads = call("GET", "/api/uploads", tok=tok); assert st == 200, uploads
json.dump({"token": tok, "chapter": shown, "upload": up, "uploads": uploads}, open(f"{work}/expect.json", "w"))
# A consistent copy of the live file.
subprocess.run(["sqlite3", os.environ["CONTRACT_DB"], f".backup {work}/rust.sqlite"], check=True)
print("rust wrote: token, chapter, update, reorder, upload")
PY

cat > "$WORK/laravel-reads.py" <<'PY'
import json, os, sys, urllib.request, urllib.error
work = sys.argv[1]; base = os.environ["BASE_URL"]
exp = json.load(open(f"{work}/expect.json"))
def call(method, path, body=None, tok=None):
    data = None if body is None else json.dumps(body).encode()
    req = urllib.request.Request(base + path, data=data, method=method)
    req.add_header("Accept", "application/json")
    if data is not None: req.add_header("Content-Type", "application/json")
    if tok: req.add_header("Authorization", "Bearer " + tok)
    try:
        with urllib.request.urlopen(req) as r: return r.status, json.loads(r.read() or b"null")
    except urllib.error.HTTPError as e: return e.code, json.loads(e.read() or b"null")
tok = exp["token"]
st, ch = call("GET", "/api/chapters/rollback-check", tok=tok)
assert st == 200, f"Laravel rejected the Rust token or row: {st} {ch}"
want = exp["chapter"]
for k in ["id", "number", "section", "title", "titleEm", "position"]:
    assert ch[k] == want[k], (k, ch[k], want[k])
# Laravel reads payload through PHP arrays: {} comes back as [] (K2), so
# compare after mapping empty objects to empty lists.
def php(v):
    if isinstance(v, dict): return [] if not v else {k: php(x) for k, x in v.items()}
    if isinstance(v, list): return [php(x) for x in v]
    return v
assert ch["lesson"] == php(want["lesson"]), "lesson differs"
assert ch["items"] == php(want["items"]), "items differ"
st, idx = call("GET", "/api/chapters", tok=tok); assert st == 200
row = next(r for r in idx if r["id"] == "rollback-check")
assert row["position"] == 0 and row["title"] == "Rollback updated", row
# ORDER BY position, number: number 99 sorts after the position-0 seed rows.
assert [r["id"] for r in idx if r["position"] == 0][-1] == "rollback-check"
st, ups = call("GET", "/api/uploads", tok=tok); assert st == 200
assert any(u["id"] == exp["upload"]["id"] and u["path"] == exp["upload"]["path"] for u in ups), ups
# Laravel can write to the file too.
st, b = call("DELETE", "/api/uploads/" + exp["upload"]["id"], tok=tok); assert st == 200, b
st, b = call("DELETE", "/api/chapters/rollback-check", tok=tok); assert st == 200, b
st, b = call("POST", "/api/auth/logout", tok=tok); assert st == 200, b
st, b = call("GET", "/api/chapters", tok=tok); assert st == 401, b
print("laravel read and wrote the Rust database; the Rust token worked and was revoked")
PY

"$SCRIPT_DIR/run.sh" rust -- python3 -I "$WORK/rust-writes.py" "$WORK"
# Same password in both sandboxes is not needed: the token carries the login.
CONTRACT_DB_FROM="$WORK/rust.sqlite" "$SCRIPT_DIR/run.sh" laravel -- python3 -I "$WORK/laravel-reads.py" "$WORK"
echo "rollback check passed"
