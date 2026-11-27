#!/usr/bin/env bash
# Build the installable release for Debian 12 (bookworm). Run it on Debian
# 12 or in a `rust:*-bookworm` container (CI does: .github/workflows/server.yml),
# so the binary needs no newer glibc than bookworm's 2.36.
#
#   server/packaging/build-release.sh
#
# Output in server/ci-artifacts/ (git-ignored):
#   grammaire-server-<version>-linux-<arch>.tar.gz
#   grammaire-server-<version>-linux-<arch>.tar.gz.sha256
# The tarball holds bin/grammaire-server, share/ (systemd unit, nginx site,
# env example, CUTOVER.md) and VERSION. The first database is not in it:
# server/scripts/initial-db-sql.sh makes that from the current content.
#
# Versioning (global rule): bump `version` in server/app/Cargo.toml before
# every installable build. The script fails if the built binary doesn't
# report that version, or if it needs a glibc newer than bookworm's.
set -euo pipefail
SERVER="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPO="$(cd "$SERVER/.." && pwd)"
cd "$SERVER"
[ "$(uname -s)" = Linux ] || { echo "build on Debian 12 / a bookworm container (CI does this)" >&2; exit 1; }
VERSION="$(sed -n 's/^version = "\(.*\)"$/\1/p' app/Cargo.toml | head -1)"
[ -n "$VERSION" ] || { echo "no version in app/Cargo.toml" >&2; exit 1; }
case "$(uname -m)" in
  x86_64) ARCH=amd64 ;;
  aarch64) ARCH=arm64 ;;
  *) ARCH="$(uname -m)" ;;
esac
# BUILD_ID (CI: the commit) marks snapshot builds: <version>+<id>.
NAME="grammaire-server-$VERSION${BUILD_ID:++$BUILD_ID}-linux-$ARCH"

SQLX_OFFLINE=true cargo build --release --locked -p grammaire-server
BIN="$SERVER/target/release/grammaire-server"
REPORTED="$("$BIN" --version)"
[ "$REPORTED" = "grammaire-server $VERSION" ] || { echo "binary reports '$REPORTED', expected $VERSION" >&2; exit 1; }
if command -v objdump >/dev/null; then
  NEED="$(objdump -T "$BIN" | grep -o 'GLIBC_[0-9.]*' | sed 's/GLIBC_//' | sort -V | tail -1)"
  if [ "$(printf '%s\n2.36\n' "$NEED" | sort -V | tail -1)" != "2.36" ]; then
    echo "binary needs glibc $NEED; Debian 12 has 2.36" >&2; exit 1
  fi
  echo "glibc needed: $NEED (bookworm: 2.36)"
fi

OUT="$SERVER/ci-artifacts"
STAGE="$OUT/$NAME"
rm -rf "$STAGE" && mkdir -p "$STAGE/bin" "$STAGE/share"
install -m 0755 "$BIN" "$STAGE/bin/grammaire-server"
install -m 0644 packaging/grammaire-server.service packaging/nginx-grammaire.conf \
  packaging/env.example "$STAGE/share/"
install -m 0644 "$REPO/docs/migration/CUTOVER.md" "$STAGE/share/"
echo "$VERSION" > "$STAGE/VERSION"
tar -C "$OUT" -czf "$OUT/$NAME.tar.gz" "$NAME"
(cd "$OUT" && sha256sum "$NAME.tar.gz" > "$NAME.tar.gz.sha256")
rm -rf "$STAGE"
echo "built $OUT/$NAME.tar.gz ($REPORTED)"
