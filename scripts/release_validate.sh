#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DIST_DIR="${DIST_DIR:-$ROOT/dist}"
STATE_DIR="${STATE_DIR:-$(mktemp -d)}"
PKG_FILE="${PKG_FILE:-}"
VENDOR_ARCHIVE="${VENDOR_ARCHIVE:-$DIST_DIR/vendor.tar.gz}"

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

if [[ ! -f "$ROOT/.cargo/config.toml" ]]; then
  echo "missing .cargo/config.toml; run scripts/vendor_deps.sh" >&2
  exit 1
fi


if ! grep -q 'directory = "vendor"' "$ROOT/.cargo/config.toml" || grep -q '/workspace/everarcade-compiler/vendor' "$ROOT/.cargo/config.toml"; then
  echo "release_validate=failed reason=absolute_vendor_path" >&2
  exit 1
fi

if [[ ! -d "$ROOT/vendor" ]]; then
  if [[ -f "$VENDOR_ARCHIVE" ]]; then
    mkdir -p "$ROOT/vendor"
    tar -C "$ROOT" -xzf "$VENDOR_ARCHIVE"
  else
    echo "missing vendor directory; run scripts/vendor_deps.sh (optionally VENDOR_ARCHIVE=$VENDOR_ARCHIVE)" >&2
    exit 1
  fi
fi

run cargo build --release --locked --frozen --offline -p everarcade-host
run cargo test --locked --frozen --offline
run "$ROOT/scripts/release_smoke.sh"
run "$ROOT/scripts/install_smoke.sh"
run "$ROOT/scripts/validate_runtime_distribution.sh"

if [[ -z "$PKG_FILE" ]]; then
  PKG_FILE="$(ls -1t "$DIST_DIR"/everarcade-host-operator-v*.tar.gz | head -n1)"
fi

echo "release_archive=$PKG_FILE"
if [[ -f "$VENDOR_ARCHIVE" ]]; then
  echo "vendor_archive=$VENDOR_ARCHIVE"
fi
echo "release_validation=ok"
