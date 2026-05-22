#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
MANIFEST_DIR="$ROOT/runtime/manifests"
RUNTIME_DIR="$ROOT/runtime"
DIST_DIR="$ROOT/dist"
PACKAGE_PATH="$DIST_DIR/everarcade-runtime-appliance-v0.1.0.tar.gz"

mkdir -p "$MANIFEST_DIR" "$DIST_DIR"

sha256_hex() {
  sha256sum "$1" | awk '{print $1}'
}

stable_file_list() {
  find "$1" -type f | LC_ALL=C sort
}

# Hash canonical runtime artifacts.
ARTIFACTS_SHA_FILE="$MANIFEST_DIR/runtime-artifacts.sha256"
: > "$ARTIFACTS_SHA_FILE"
while IFS= read -r f; do
  rel="${f#$ROOT/}"
  case "$rel" in
    runtime/manifests/*|runtime/replay/latest/*|runtime/logs/*) continue ;;
  esac
  printf '%s  %s\n' "$(sha256_hex "$f")" "$rel" >> "$ARTIFACTS_SHA_FILE"
done < <(stable_file_list "$RUNTIME_DIR")

# Hash replay outputs deterministically.
REPLAY_SHA_FILE="$MANIFEST_DIR/replay.sha256"
: > "$REPLAY_SHA_FILE"
if [[ -d "$RUNTIME_DIR/replay/latest" ]]; then
  while IFS= read -r f; do
    rel="${f#$ROOT/}"
    printf '%s  %s\n' "$(sha256_hex "$f")" "$rel" >> "$REPLAY_SHA_FILE"
  done < <(stable_file_list "$RUNTIME_DIR/replay/latest")
fi

if [[ ! -f "$PACKAGE_PATH" ]]; then
  bash "$ROOT/scripts/build_vm_runtime_appliance.sh"
fi

printf '%s  %s\n' "$(sha256_hex "$PACKAGE_PATH")" "${PACKAGE_PATH#$ROOT/}" > "$DIST_DIR/runtime.sha256"

# Build canonical deterministic release manifest (no generated_at field).
RUNTIME_VERSION="v0.1.0"
GIT_COMMIT="$(git -C "$ROOT" rev-parse --verify HEAD)"
BUILD_TARGET="$(rustc -vV | awk '/host:/ {print $2}')"
BUILD_PROFILE="release"
RUST_VERSION="$(rustc --version | awk '{print $2}')"
REPLAY_HASH=""
if [[ -f "$RUNTIME_DIR/replay/latest/frame-0001.json" ]]; then
  REPLAY_HASH="sha256:$(sha256_hex "$RUNTIME_DIR/replay/latest/frame-0001.json")"
fi
PACKAGE_HASH="sha256:$(sha256_hex "$PACKAGE_PATH")"

MANIFEST_PATH="$MANIFEST_DIR/runtime-release-manifest.json"
{
  echo '{'
  echo "  \"runtime_version\": \"$RUNTIME_VERSION\"," 
  echo "  \"git_commit\": \"$GIT_COMMIT\"," 
  echo "  \"build_target\": \"$BUILD_TARGET\"," 
  echo "  \"build_profile\": \"$BUILD_PROFILE\"," 
  echo "  \"rust_version\": \"$RUST_VERSION\"," 
  echo '  "artifact_hashes": {'
  awk '{printf "    \"%s\": \"sha256:%s\",\n", $2, $1}' "$ARTIFACTS_SHA_FILE" | sed '$ s/,$//'
  echo '  },'
  echo "  \"package_hash\": \"$PACKAGE_HASH\"," 
  echo "  \"replay_hash\": \"$REPLAY_HASH\""
  echo '}'
} > "$MANIFEST_PATH"

# Human-readable provenance metadata includes timestamp and non-canonical info.
PROV_PATH="$MANIFEST_DIR/build-provenance.json"
cat > "$PROV_PATH" <<JSON
{
  "git_commit": "$GIT_COMMIT",
  "rustc_version": "$(rustc --version)",
  "cargo_version": "$(cargo --version)",
  "build_target": "$BUILD_TARGET",
  "build_profile": "$BUILD_PROFILE",
  "host_platform": "$(uname -s | tr '[:upper:]' '[:lower:]')-$(uname -m)",
  "deterministic_mode": true,
  "generated_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
}
JSON

printf '%s  %s\n' "$(sha256_hex "$MANIFEST_PATH")" "${MANIFEST_PATH#$ROOT/}" > "$DIST_DIR/runtime-manifest.sha256"
printf '%s  %s\n' "$(sha256_hex "$MANIFEST_PATH")" "${MANIFEST_PATH#$ROOT/}" > "$MANIFEST_DIR/runtime-release-manifest.sha256"

echo "hash_runtime_artifacts=ok manifest=$MANIFEST_PATH"
