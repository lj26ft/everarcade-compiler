#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
mkdir -p dist release
bash scripts/verify_vendor_integrity.sh
GIT_COMMIT=$(git rev-parse --short=12 HEAD)
CARGO_LOCK_HASH=$(sha256sum Cargo.lock | awk '{print $1}')
RUNTIME_ROOT=$(sha256sum runtime/config/runtime.toml | awk '{print $1}')
VALIDATION_ROOT=$(sha256sum execution-core/Cargo.toml | awk '{print $1}')
printf '%s
' "$RUNTIME_ROOT" > release/RUNTIME_ROOT
printf '%s
' "$VALIDATION_ROOT" > release/VALIDATION_ROOT
cat > release/BUILD_INFO <<EOT
git_commit=$GIT_COMMIT
build_target=x86_64-unknown-linux-gnu
cargo_lock_hash=$CARGO_LOCK_HASH
release_timestamp_policy=SOURCE_DATE_EPOCH_or_UTC0
vendor_hash=$(cat release/VENDOR_HASH)
EOT
cat > release/manifest.json <<EOT
{"git_commit":"$GIT_COMMIT","build_target":"x86_64-unknown-linux-gnu","cargo_lock_hash":"$CARGO_LOCK_HASH","vendor_hash":"$(cat release/VENDOR_HASH)","runtime_validation_root":"$VALIDATION_ROOT","runtime_root":"$RUNTIME_ROOT"}
EOT
(
  sha256sum release/manifest.json release/BUILD_INFO release/RUNTIME_ROOT release/VALIDATION_ROOT
) > release/SHA256SUMS
STAGE="dist/everarcade-v0.1.0-linux-amd64"
rm -rf "$STAGE"
mkdir -p "$STAGE"/{bin,runtime,docs,reports,vendor,scripts,manifest}
cp -a runtime/config "$STAGE/runtime/"
cp -a docs "$STAGE/"
cp -a scripts "$STAGE/"
cp -a release/* "$STAGE/manifest/"
[ -d world/reports ] && cp -a world/reports "$STAGE/reports/world" || true
[ -d wasm/reports ] && cp -a wasm/reports "$STAGE/reports/wasm" || true
[ -d deployment/reports ] && cp -a deployment/reports "$STAGE/reports/deployment" || true
cp -a vendor "$STAGE/"
mkdir -p "$STAGE/.cargo" && cp -a .cargo/config.toml "$STAGE/.cargo/" 2>/dev/null || true
cp -a Cargo.lock "$STAGE/"
TAR="dist/everarcade-v0.1.0-linux-amd64.tar.gz"
SOURCE_DATE_EPOCH=${SOURCE_DATE_EPOCH:-0} tar --sort=name --mtime='UTC 1970-01-01' --owner=0 --group=0 --numeric-owner -czf "$TAR" -C dist everarcade-v0.1.0-linux-amd64
sha256sum "$TAR" > dist/everarcade-v0.1.0-linux-amd64.tar.gz.sha256
