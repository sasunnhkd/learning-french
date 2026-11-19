#!/usr/bin/env python3
"""Check every crates.io package in a Cargo.lock against the OSV database.

Stand-in for `cargo audit` / `cargo deny` until those tools are installed and
vetted (docs/migration/MIGRATION_PLAN.md, Q14). Needs Python 3.6+ and network
access to api.osv.dev; sends only crate names and versions.

Usage: python3 -I server/scripts/osv-audit.py server/Cargo.lock
Exit 1 if any crate has an advisory or a banned crate is locked.

BANNED: crates from the August 2026 crates.io compromise (RUSTSEC-2026-0260).
Only arrayref 0.3.10, internment 0.8.7, append-only-vec 0.1.9 and the
proc-macro1 typosquat were malicious; the whole crates are banned here so a
lockfile change touching them gets a human look. To change the list, edit
BANNED.
"""
import json
import re
import sys
import urllib.request

BANNED = {"proc-macro1", "arrayref", "internment", "append-only-vec"}


def main(lock_path):
    with open(lock_path, encoding="utf-8") as f:
        text = f.read()
    pkgs = [
        {"name": name, "version": version}
        for name, version in re.findall(
            r'name = "([^"]+)"\nversion = "([^"]+)"\nsource = "registry[^"]*"', text
        )
    ]
    query = {
        "queries": [
            {"package": {"ecosystem": "crates.io", "name": p["name"]}, "version": p["version"]}
            for p in pkgs
        ]
    }
    req = urllib.request.Request(
        "https://api.osv.dev/v1/querybatch",
        data=json.dumps(query).encode(),
        headers={"Content-Type": "application/json"},
    )
    with urllib.request.urlopen(req, timeout=30) as resp:
        results = json.load(resp)["results"]

    flagged = 0
    for p, r in zip(pkgs, results):
        ids = [v["id"] for v in r.get("vulns", [])]
        flagged += bool(ids)
        print(f'{p["name"]} {p["version"]}: {", ".join(ids) or "clean"}')
    banned = sorted({p["name"] for p in pkgs} & BANNED)
    print("banned crates present:", ", ".join(banned) or "none")
    return 1 if flagged or banned else 0


if __name__ == "__main__":
    if len(sys.argv) != 2:
        sys.exit("usage: osv-audit.py path/to/Cargo.lock")
    sys.exit(main(sys.argv[1]))
