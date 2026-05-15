#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DIST_DIR="${DIST_DIR:-$ROOT/dist}"
STATE_DIR="${STATE_DIR:-$(mktemp -d)}"
PKG_FILE="${PKG_FILE:-}"

cleanup() {
  rm -rf "$STATE_DIR"
}
trap cleanup EXIT

run() {
  echo "+ $*"
  "$@"
}

cd "$ROOT"

if [[ ! -f "$ROOT/Cargo.lock" ]]; then
  echo "missing Cargo.lock" >&2
  exit 1
fi

if [[ ! -d "$ROOT/vendor" ]]; then
  echo "missing vendor directory; run scripts/vendor_deps.sh" >&2
  exit 1
fi

run cargo build --release --locked --frozen --offline -p everarcade-host
run cargo test --locked --frozen --offline
run "$ROOT/scripts/release_smoke.sh"
run "$ROOT/scripts/install_smoke.sh"

if [[ -z "$PKG_FILE" ]]; then
  PKG_FILE="$(ls -1t "$DIST_DIR"/everarcade-host-operator-v*.tar.gz | head -n1)"
fi

echo "release_archive=$PKG_FILE"
echo "release_validation=ok"
