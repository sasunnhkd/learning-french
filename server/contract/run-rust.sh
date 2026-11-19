#!/usr/bin/env bash
# Contract suite against the sandboxed Rust app (ported tests by default). See run.sh.
exec "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/run.sh" rust "$@"
