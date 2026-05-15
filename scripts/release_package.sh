#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$(realpath "${1:-dist}")"
PROFILE="${PROFILE:-release}"
TEMPLATE_DIR="$ROOT/deploy/templates"
SYSTEMD_DIR="$ROOT/deploy/systemd"

mkdir -p "$OUT_DIR"

if [[ "$PROFILE" == "release" ]]; then
  cargo build --release -p everarcade-host
  BIN_PATH="$ROOT/target/release/everarcade-host"
else
  cargo build -p everarcade-host
  BIN_PATH="$ROOT/target/debug/everarcade-host"
fi

VERSION="$(cargo metadata --format-version 1 --no-deps | python3 -c 'import json,sys; d=json.load(sys.stdin); print(next(p["version"] for p in d["packages"] if p["name"]=="everarcade-host"))')"
PKG_ROOT="$(mktemp -d)"
PKG_TS="$(date -u +%Y%m%dT%H%M%SZ)"
PKG_NAME="everarcade-host-operator-v${VERSION}-${PKG_TS}"
PKG_DIR="$PKG_ROOT/$PKG_NAME"
mkdir -p "$PKG_DIR/bin" "$PKG_DIR/deploy" "$PKG_DIR/meta"

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
cp "$ROOT/docs/evernode-deployment.md" "$PKG_DIR/deploy/"
cp "$ROOT/docs/linux-vm-operator-quickstart.md" "$PKG_DIR/deploy/"

cat > "$PKG_DIR/meta/version.json" <<META
{
  "name": "everarcade-host",
  "version": "$VERSION",
  "profile": "$PROFILE",
  "packaged_at_utc": "$PKG_TS",
  "core_rule": "release packaging distributes the operator; replay verification defines truth"
}
META

cp "$ROOT/scripts/install.sh" "$PKG_DIR/install.sh"
cp "$ROOT/scripts/uninstall.sh" "$PKG_DIR/uninstall.sh"
chmod +x "$PKG_DIR/install.sh" "$PKG_DIR/uninstall.sh" "$PKG_DIR/bin/everarcade-host"

( cd "$PKG_ROOT" && tar -czf "$OUT_DIR/$PKG_NAME.tar.gz" "$PKG_NAME" )
SHA_PATH="$OUT_DIR/$PKG_NAME.sha256"
( cd "$OUT_DIR" && sha256sum "$PKG_NAME.tar.gz" > "$(basename "$SHA_PATH")" )

echo "release_package=ok archive=$OUT_DIR/$PKG_NAME.tar.gz checksum=$SHA_PATH version=$VERSION"
rm -rf "$PKG_ROOT"
