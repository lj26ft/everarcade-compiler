#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
PACKAGE="${1:-$ROOT/dist/everarcade-evernode-v0.1.tar.gz}"
CHECKSUM="${2:-$PACKAGE.sha256}"
REPORT_DIR="$ROOT/reports"
REPORT="$REPORT_DIR/evernode_install_report.txt"
INSTALL_ROOT="$ROOT/evernode/install"
TMP_DIR="$ROOT/tmp/evernode-package-install"
mkdir -p "$REPORT_DIR" "$INSTALL_ROOT"
rm -rf "$TMP_DIR"
mkdir -p "$TMP_DIR"

checksum_status="FAIL"
manifest_status="FAIL"
layout_status="FAIL"
install_status="FAIL"
overall="FAIL"

if [[ -s "$PACKAGE" && -s "$CHECKSUM" ]] && (cd "$(dirname "$PACKAGE")" && sha256sum -c "$(basename "$CHECKSUM")" >/dev/null); then
  checksum_status="PASS"
  tar -xzf "$PACKAGE" -C "$TMP_DIR"
  package_root="$(find "$TMP_DIR" -mindepth 1 -maxdepth 1 -type d | head -n1)"
  if [[ -n "${package_root:-}" && -s "$package_root/PACKAGE_MANIFEST.txt" ]] && grep -q '^EverArcade Evernode Deployment Package v0.1$' "$package_root/PACKAGE_MANIFEST.txt"; then
    manifest_status="PASS"
  fi
  required=(
    evernode/lease/deployment-manifest.txt
    evernode/runtime
    evernode/node
    evernode/hotpocket
    runtime/config
    runtime/deployment
    node
    hotpocket/adapter/hotpocket_adapter.sh
    docs/runtime-platform/evernode-deployment-v0.1.md
  )
  if [[ "$manifest_status" == PASS ]]; then
    layout_status="PASS"
    for path in "${required[@]}"; do
      if [[ ! -e "$package_root/$path" ]]; then
        layout_status="FAIL"
        break
      fi
    done
  fi
  if [[ "$layout_status" == PASS ]]; then
    rm -rf "$INSTALL_ROOT/current"
    mkdir -p "$INSTALL_ROOT"
    cp -R "$package_root" "$INSTALL_ROOT/current"
    install_status="PASS"
  fi
fi

if [[ "$checksum_status" == PASS && "$manifest_status" == PASS && "$layout_status" == PASS && "$install_status" == PASS ]]; then
  overall="PASS"
fi

cat > "$REPORT" <<REPORT_BODY
Evernode Install Report
Package: $PACKAGE
Checksum: $checksum_status
Manifest: $manifest_status
Layout: $layout_status
Installed: $install_status
Installation: $overall
REPORT_BODY

echo "Installation: $overall"
[[ "$overall" == PASS ]]
