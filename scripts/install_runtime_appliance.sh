#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if [[ -f "$SCRIPT_DIR/../MANIFEST.sha256" && -d "$SCRIPT_DIR/../runtime" ]]; then
  MODE="in-place"
  APPLIANCE_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
  VERSION="$(basename "$APPLIANCE_DIR")"
  PREFIX="$APPLIANCE_DIR"
else
  MODE="bundle"
  ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
  VERSION="everarcade-runtime-v0.1"
  DIST_DIR="${DIST_DIR:-$ROOT_DIR/dist}"
  BUNDLE="${1:-$DIST_DIR/$VERSION.tar.gz}"
  PREFIX="${2:-${EVERARCADE_APPLIANCE_PREFIX:-$ROOT_DIR/appliance/installed}}"
  CHECKSUM_FILE="$BUNDLE.sha256"
fi

fail() { echo "Install: FAIL - $*" >&2; exit 1; }
verify_manifest() {
  local dir="$1"
  [[ -f "$dir/MANIFEST.sha256" ]] || fail "missing manifest: $dir/MANIFEST.sha256"
  (cd "$dir" && sha256sum -c MANIFEST.sha256 >/dev/null) || fail "manifest verification failed"
}
initialize_layout() {
  local dir="$1"
  local runtime_root="$dir/runtime"
  local world_id="${EVERARCADE_WORLD_ID:-civilization-alpha}"
  mkdir -p \
    "$runtime_root/config" \
    "$runtime_root/worlds/$world_id/journals" \
    "$runtime_root/worlds/$world_id/checkpoints" \
    "$runtime_root/worlds/$world_id/backups" \
    "$runtime_root/worlds/$world_id/receipts" \
    "$runtime_root/worlds/$world_id/state" \
    "$runtime_root/journals" \
    "$runtime_root/checkpoints" \
    "$runtime_root/backups" \
    "$runtime_root/reports"
  [[ -x "$dir/bin/everarcade-runtime" ]] || fail "runtime binary is missing or not executable"
  [[ -f "$runtime_root/config/runtime.env" ]] || fail "runtime config missing"
  [[ -f "$runtime_root/worlds/$world_id/package/manifest.json" ]] || fail "world package manifest missing"
}

if [[ "${MODE}" == "bundle" ]]; then
  [[ -s "$BUNDLE" ]] || fail "bundle missing: $BUNDLE"
  [[ -s "$CHECKSUM_FILE" ]] || fail "checksum missing: $CHECKSUM_FILE"
  (cd "$(dirname "$BUNDLE")" && sha256sum -c "$(basename "$CHECKSUM_FILE")" >/dev/null) || fail "bundle checksum verification failed"
  TMP="$(mktemp -d)"
  trap 'rm -rf "$TMP"' EXIT
  tar -xzf "$BUNDLE" -C "$TMP"
  [[ -d "$TMP/$VERSION" ]] || fail "bundle did not contain $VERSION"
  verify_manifest "$TMP/$VERSION"
  rm -rf "$PREFIX"
  mkdir -p "$(dirname "$PREFIX")"
  mv "$TMP/$VERSION" "$PREFIX"
  initialize_layout "$PREFIX"
else
  verify_manifest "$APPLIANCE_DIR"
  initialize_layout "$APPLIANCE_DIR"
fi

echo "Install: PASS"
echo "prefix=$PREFIX"
