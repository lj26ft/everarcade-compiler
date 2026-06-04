#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

VERSION="v0.1"
NAME="everarcade-lease-handoff-$VERSION"
DIST_DIR="$ROOT/dist"
PACKAGE="${1:-$DIST_DIR/$NAME.tar.gz}"
CHECKSUM="${2:-$PACKAGE.sha256}"
INSTALL_ROOT="${3:-$ROOT/handoff/lease-install}"
REPORT_DIR="$ROOT/reports"
REPORT="$REPORT_DIR/lease_handoff_install_report.txt"
VERIFY_ROOT="$ROOT/tmp/lease-handoff-install-verify"

mkdir -p "$REPORT_DIR"
for ignore_pattern in '/handoff/lease-install/' '/tmp/lease-handoff-install-verify/'; do
  grep -qxF "$ignore_pattern" "$ROOT/.git/info/exclude" || printf '%s\n' "$ignore_pattern" >> "$ROOT/.git/info/exclude"
done

verify_status="FAIL"
install_status="FAIL"
manifest_status="FAIL"
layout_status="FAIL"
overall="FAIL"
verify_output=""

rm -rf "$VERIFY_ROOT" "$INSTALL_ROOT"
mkdir -p "$VERIFY_ROOT" "$(dirname "$INSTALL_ROOT")"

if verify_output="$(bash scripts/verify_lease_handoff_package.sh "$PACKAGE" "$CHECKSUM" 2>&1)"; then
  verify_status="PASS"
fi

if [[ "$verify_status" == PASS ]]; then
  mkdir -p "$INSTALL_ROOT"
  if tar -xzf "$PACKAGE" -C "$INSTALL_ROOT"; then
    install_status="PASS"
  fi
fi

INSTALLED="$INSTALL_ROOT/$NAME"
if [[ -s "$INSTALLED/manifests/deployment-manifest.txt" ]]; then
  manifest_status="PASS"
fi
if [[ -d "$INSTALLED/package" \
  && -d "$INSTALLED/docs" \
  && -d "$INSTALLED/scripts" \
  && -d "$INSTALLED/manifests" \
  && -d "$INSTALLED/checksums" \
  && -d "$INSTALLED/reports" ]]; then
  layout_status="PASS"
fi

if [[ "$verify_status" == PASS && "$install_status" == PASS && "$manifest_status" == PASS && "$layout_status" == PASS ]]; then
  overall="PASS"
fi

cat > "$REPORT" <<REPORT_BODY
Evernode Lease Handoff Installation Report
Verify Before Install: $verify_status
Install Extract: $install_status
Manifest: $manifest_status
Layout: $layout_status
Install Target: $INSTALL_ROOT
Lease Handoff Installation: $overall

Verify Output:
$verify_output
REPORT_BODY

echo "Lease Handoff Installation: $overall"
[[ "$overall" == PASS ]]
