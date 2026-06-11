#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"

runtime_status=FAIL
export_status=FAIL
restore_status=FAIL
equivalence_status=FAIL
continuation_status=FAIL
replay_status=FAIL
validator_status=FAIL

if node runtime/hotpocket-runtime-proof/validation/hotpocket-runtime-proof.js validate >/tmp/everarcade-hotpocket-runtime-proof.log 2>&1; then
  if grep -q 'PASS' /tmp/everarcade-hotpocket-runtime-proof.log; then runtime_status=PASS; fi
fi

if node runtime/hotpocket-migration-proof/validation/hotpocket-migration-proof.js validate >/tmp/everarcade-hotpocket-migration-proof.log 2>&1; then
  if grep -q 'PASS' /tmp/everarcade-hotpocket-migration-proof.log; then
    grep -q 'Export proof: PASS' runtime/hotpocket-migration-proof/reports/hotpocket_migration_validation_report.txt && export_status=PASS
    grep -q 'Restore proof: PASS' runtime/hotpocket-migration-proof/reports/hotpocket_migration_validation_report.txt && restore_status=PASS
    grep -q 'Equivalence proof: PASS' runtime/hotpocket-migration-proof/reports/hotpocket_migration_validation_report.txt && equivalence_status=PASS
    grep -q 'Continuation proof: PASS' runtime/hotpocket-migration-proof/reports/hotpocket_migration_validation_report.txt && continuation_status=PASS
    grep -q 'Replay proof: PASS' runtime/hotpocket-migration-proof/reports/hotpocket_migration_validation_report.txt && replay_status=PASS
    grep -q 'Validator agreement proof: PASS' runtime/hotpocket-migration-proof/reports/hotpocket_migration_validation_report.txt && validator_status=PASS
  fi
fi

echo "PASS runtime integration proof: $runtime_status"
echo "PASS export proof: $export_status"
echo "PASS restore proof: $restore_status"
echo "PASS equivalence proof: $equivalence_status"
echo "PASS continuation proof: $continuation_status"
echo "PASS replay proof: $replay_status"
echo "PASS validator agreement proof: $validator_status"

if [[ "$runtime_status$export_status$restore_status$equivalence_status$continuation_status$replay_status$validator_status" == "PASSPASSPASSPASSPASSPASSPASS" ]]; then
  echo "EverArcade Sovereign Runtime Migration Proof v0.1: PASS"
else
  echo "EverArcade Sovereign Runtime Migration Proof v0.1: FAIL"
  echo "Runtime proof log: /tmp/everarcade-hotpocket-runtime-proof.log" >&2
  echo "Migration proof log: /tmp/everarcade-hotpocket-migration-proof.log" >&2
  exit 1
fi
