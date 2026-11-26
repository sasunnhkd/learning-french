#!/usr/bin/env bash
# Dependency vulnerability check without extra tools (Phase 5, AUDIT_REPORT
# §3): every crates.io package in the resolved build graph (Cargo.lock) is
# looked up in the OSV database (api.osv.dev, which includes RustSec).
# Sends only crate names and versions. Exits 1 if any has an advisory.
#   server/scripts/osv-audit.sh
set -euo pipefail
SERVER="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WORK="$(mktemp -d "${TMPDIR:-/tmp}/grammaire-osv.XXXXXX")"
trap 'rm -rf "$WORK"' EXIT
cargo metadata --manifest-path "$SERVER/Cargo.toml" --format-version 1 --locked > "$WORK/meta.json"
python3 -I - "$WORK" <<'PY'
import json, sys
work = sys.argv[1]
m = json.load(open(f"{work}/meta.json"))
ids = {n["id"] for n in m["resolve"]["nodes"]}
q = [{"package": {"name": p["name"], "ecosystem": "crates.io"}, "version": p["version"]}
     for p in m["packages"] if p["id"] in ids and p["source"]]
json.dump({"queries": q}, open(f"{work}/query.json", "w"))
PY
curl -sfS --max-time 60 -H 'Content-Type: application/json' \
  --data-binary @"$WORK/query.json" https://api.osv.dev/v1/querybatch > "$WORK/result.json"
python3 -I - "$WORK" <<'PY'
import json, sys
work = sys.argv[1]
q = json.load(open(f"{work}/query.json"))["queries"]
r = json.load(open(f"{work}/result.json"))["results"]
if len(q) != len(r):
    sys.exit(f"OSV answered {len(r)} of {len(q)} queries")
hits = [(a["package"]["name"], a["version"], [v["id"] for v in b.get("vulns", [])])
        for a, b in zip(q, r) if b.get("vulns")]
for name, version, vulns in hits:
    print(f"ADVISORY {name} {version}: {', '.join(vulns)}")
print(f"OSV: {len(q)} crates checked, {len(hits)} with advisories")
sys.exit(1 if hits else 0)
PY
