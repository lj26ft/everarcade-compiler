#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"; cd "$ROOT"
PACKAGE="${1:-$ROOT/dist/everarcade-evernode-v0.1.tar.gz}"; CHECKSUM="${2:-$PACKAGE.sha256}"
REPORT_DIR="$ROOT/reports"; REPORT="$REPORT_DIR/evernode_upgrade_report.txt"; BACKUP_DIR="$ROOT/evernode/backups/upgrade-$(date -u +"%Y%m%dT%H%M%SZ")"; mkdir -p "$REPORT_DIR" evernode/reports evernode/backups
backup_status="FAIL"; package_status="FAIL"; layout_status="FAIL"; checkpoint_status="FAIL"; overall="FAIL"
checkpoint_before="$(cat node/checkpoints/latest_checkpoint 2>/dev/null || printf NONE)"
checkpoint_root_before=""; [[ "$checkpoint_before" != NONE && -f "$checkpoint_before" ]] && checkpoint_root_before="$(sha256sum "$checkpoint_before" | awk '{print $1}')"
mkdir -p "$BACKUP_DIR"
cp -R evernode/lease evernode/runtime evernode/node evernode/hotpocket evernode/checkpoints evernode/journals "$BACKUP_DIR/" 2>/dev/null || true
[[ -d "$BACKUP_DIR/lease" && -d "$BACKUP_DIR/runtime" ]] && backup_status="PASS"
if [[ -s "$PACKAGE" && -s "$CHECKSUM" ]] && (cd "$(dirname "$PACKAGE")" && sha256sum -c "$(basename "$CHECKSUM")" >/dev/null); then package_status="PASS"; fi
[[ -d evernode/lease && -d evernode/runtime && -d evernode/node && -d evernode/hotpocket ]] && layout_status="PASS"
checkpoint_after="$(cat node/checkpoints/latest_checkpoint 2>/dev/null || printf NONE)"
checkpoint_root_after=""; [[ "$checkpoint_after" != NONE && -f "$checkpoint_after" ]] && checkpoint_root_after="$(sha256sum "$checkpoint_after" | awk '{print $1}')"
if [[ -n "$checkpoint_root_before" && "$checkpoint_root_before" == "$checkpoint_root_after" ]]; then checkpoint_status="PASS"; fi
if [[ "$backup_status" == PASS && "$package_status" == PASS && "$layout_status" == PASS && "$checkpoint_status" == PASS ]]; then overall="PASS"; fi
cat > "$REPORT" <<REPORT_BODY
Evernode Upgrade Report
Backup Created: $backup_status
Package Verified: $package_status
Layout Preserved: $layout_status
Checkpoint Preserved: $checkpoint_status
Upgrade: $overall
REPORT_BODY
cp "$REPORT" evernode/reports/
echo "Upgrade: $overall"
[[ "$overall" == PASS ]]
