#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"

REPORT_DIR="$ROOT/reports"
CERT_REPORT="$REPORT_DIR/continuity_anchor_certification_report.txt"
mkdir -p "$REPORT_DIR"

runtime_status=FAIL
migration_status=FAIL
continuity_status=FAIL
replay_status=FAIL
restore_status=FAIL
payload_status=FAIL

if node runtime/hotpocket-runtime-proof/validation/hotpocket-runtime-proof.js validate >/tmp/everarcade-continuity-runtime-proof.log 2>&1; then
  if grep -q 'EverArcade Runtime ↔ HotPocket Integration Proof v0.1: PASS' runtime/hotpocket-runtime-proof/reports/hotpocket_runtime_validation_report.txt; then
    runtime_status=PASS
  fi
fi

if node runtime/hotpocket-migration-proof/validation/hotpocket-migration-proof.js validate >/tmp/everarcade-continuity-migration-proof.log 2>&1; then
  if grep -q 'EverArcade Sovereign Runtime Migration Proof v0.1: PASS' runtime/hotpocket-migration-proof/reports/hotpocket_migration_validation_report.txt; then
    migration_status=PASS
  fi
fi

if node runtime/continuity-anchor-proof/validation/continuity-anchor-proof.js validate >/tmp/everarcade-continuity-anchor-proof.log 2>&1; then
  if grep -q 'EverArcade Continuity Anchoring Proof v0.1: PASS' runtime/continuity-anchor-proof/reports/continuity_anchor_validation_report.txt; then
    continuity_status=PASS
  fi
fi

grep -q 'Replay Equivalence Proof: PASS' runtime/continuity-anchor-proof/reports/anchor_replay_report.txt && replay_status=PASS
grep -q 'Restore Equivalence Proof: PASS' runtime/continuity-anchor-proof/reports/anchor_restore_report.txt && restore_status=PASS
if grep -q '"status": "PASS"' runtime/continuity-anchor-proof/reports/xrpl_anchor_payload_report.json \
  && grep -q '"status": "PASS"' runtime/continuity-anchor-proof/reports/xahau_anchor_payload_report.json; then
  payload_status=PASS
fi

overall=FAIL
if [[ "$runtime_status$migration_status$continuity_status$replay_status$restore_status$payload_status" == "PASSPASSPASSPASSPASSPASS" ]]; then
  overall=PASS
fi

cat > "$CERT_REPORT" <<REPORT_BODY
EverArcade Continuity Anchoring Proof v0.1 Certification
PASS runtime integration proof: $runtime_status
PASS migration proof: $migration_status
PASS continuity proof: $continuity_status
PASS replay proof: $replay_status
PASS restore proof: $restore_status
PASS payload proof: $payload_status
EverArcade Continuity Anchoring Proof v0.1: $overall
REPORT_BODY

cat "$CERT_REPORT"
if [[ "$overall" != PASS ]]; then
  echo "Runtime proof log: /tmp/everarcade-continuity-runtime-proof.log" >&2
  echo "Migration proof log: /tmp/everarcade-continuity-migration-proof.log" >&2
  echo "Continuity proof log: /tmp/everarcade-continuity-anchor-proof.log" >&2
  exit 1
fi
