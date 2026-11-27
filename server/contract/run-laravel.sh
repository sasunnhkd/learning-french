#!/usr/bin/env bash
# Contract suite against the sandboxed Laravel app. See run.sh.
exec "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/run.sh" laravel "$@"
