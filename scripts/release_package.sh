#!/usr/bin/env bash
set -euo pipefail

OUT_DIR="$(realpath "${1:-dist}")"
PROFILE="${PROFILE:-release}"
TEMPLATE_DIR="deploy/templates"
SYSTEMD_DIR="deploy/systemd"

mkdir -p "$OUT_DIR"

if [[ "$PROFILE" == "release" ]]; then
  cargo build --release -p everarcade-host
  BIN_PATH="target/release/everarcade-host"
else
  cargo build -p everarcade-host
  BIN_PATH="target/debug/everarcade-host"
fi

PKG_ROOT="$(mktemp -d)"
PKG_NAME="everarcade-host-operator-$(date -u +%Y%m%dT%H%M%SZ)"
PKG_DIR="$PKG_ROOT/$PKG_NAME"
mkdir -p "$PKG_DIR/bin" "$PKG_DIR/deploy"

cp "$BIN_PATH" "$PKG_DIR/bin/everarcade-host"
cp -R "$TEMPLATE_DIR" "$PKG_DIR/deploy/templates"
cp -R "$SYSTEMD_DIR" "$PKG_DIR/deploy/systemd"
mkdir -p "$PKG_DIR/deploy/manifests"
cat > "$PKG_DIR/deploy/manifests/evernode-deployment.manifest.json" <<'MANIFEST'
{
  "manifest_version": 1,
  "runtime": "everarcade-host",
  "output": {
    "receipt": "<set-by-deploy-proof>",
    "checkpoint": "<set-by-deploy-proof>",
    "anchor": "<set-by-deploy-proof>",
    "ipfs_manifest": "<set-by-deploy-proof>"
  }
}
MANIFEST
cp docs/evernode-deployment.md "$PKG_DIR/deploy/"
cp docs/linux-vm-operator-quickstart.md "$PKG_DIR/deploy/"

( cd "$PKG_ROOT" && tar -czf "$OUT_DIR/$PKG_NAME.tar.gz" "$PKG_NAME" )
SHA_PATH="$OUT_DIR/$PKG_NAME.sha256"
( cd "$OUT_DIR" && sha256sum "$PKG_NAME.tar.gz" > "$(basename "$SHA_PATH")" )

echo "release_package=ok archive=$OUT_DIR/$PKG_NAME.tar.gz checksum=$SHA_PATH"
rm -rf "$PKG_ROOT"
