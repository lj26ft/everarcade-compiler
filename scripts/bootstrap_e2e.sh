#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
STATE_DIR="${STATE_DIR:-$(mktemp -d /tmp/everarcade-e2e-state.XXXXXX)}"
PKG_FILE="${PKG_FILE:-$(mktemp /tmp/everarcade-e2e-package.XXXXXX.bin)}"
SKIP_VENDOR="${SKIP_VENDOR:-0}"

cleanup() {
  if [[ "${KEEP_ARTIFACTS:-0}" != "1" ]]; then
    rm -rf "$STATE_DIR"
    rm -f "$PKG_FILE"
  fi
}
trap cleanup EXIT

run() {
  echo "+ $*"
  "$@"
}

cd "$ROOT"

if [[ "$SKIP_VENDOR" != "1" ]]; then
  run "$ROOT/scripts/vendor_deps.sh"
else
  echo "skip_vendor=1 (using existing vendor/ + .cargo/config.toml)"
fi

run cargo run -p everarcade-cli -- init-game my-first-world
run cargo run -p everarcade-cli -- build-game
run cargo run -p everarcade-cli -- package-game
run cargo run -p everarcade-cli -- run-local-federation
run cargo run -p everarcade-cli -- replay-world
run cargo run -p everarcade-cli -- inspect-simulation

run cargo run -p everarcade-host -- init --state "$STATE_DIR"
run cargo run -p everarcade-host -- generate-fixture --output "$PKG_FILE"
run cargo run -p everarcade-host -- run --package "$PKG_FILE" --state "$STATE_DIR"
run cargo run -p everarcade-host -- verify --state "$STATE_DIR"
run cargo run -p everarcade-host -- status --state "$STATE_DIR"

echo "bootstrap_e2e=ok state=$STATE_DIR package=$PKG_FILE"
if [[ "${KEEP_ARTIFACTS:-0}" != "1" ]]; then
  echo "artifacts_cleanup=enabled (set KEEP_ARTIFACTS=1 to retain state/package)"
fi
