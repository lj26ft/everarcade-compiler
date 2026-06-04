#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/multi_lease_federation_certification_report.txt"

cd "$ROOT_DIR"
# shellcheck source=../federation/multi_lease_model.sh
source "$ROOT_DIR/federation/multi_lease_model.sh"
mkdir -p "$REPORT_DIR"

federation_write_artifacts

identity_status="FAIL"
membership_status="FAIL"
topology_status="FAIL"
checkpoint_status="FAIL"
replay_status="FAIL"
settlement_status="FAIL"
synchronization_status="FAIL"
recovery_status="FAIL"
overall_status="FAIL"

validate_federation_identity && identity_status="PASS"
validate_federation_membership && membership_status="PASS"
validate_federation_topology && topology_status="PASS"
validate_checkpoint_exchange && checkpoint_status="PASS"
validate_replay_exchange && replay_status="PASS"
validate_settlement_exchange && settlement_status="PASS"
validate_civilization_synchronization && synchronization_status="PASS"
validate_federation_recovery && recovery_status="PASS"

if [[ "$identity_status" == "PASS" \
  && "$membership_status" == "PASS" \
  && "$topology_status" == "PASS" \
  && "$checkpoint_status" == "PASS" \
  && "$replay_status" == "PASS" \
  && "$settlement_status" == "PASS" \
  && "$synchronization_status" == "PASS" \
  && "$recovery_status" == "PASS" ]]; then
  overall_status="PASS"
fi

cat > "$REPORT_PATH" <<REPORT
Multi-Lease Federation Runtime Certification Report
Version: $FEDERATION_RUNTIME_VERSION
Federation ID: $FEDERATION_ID
Membership Epoch: $MEMBERSHIP_EPOCH
Civilization Epoch: $CIVILIZATION_EPOCH

Identity: $identity_status
Membership: $membership_status
Topology: $topology_status
Checkpoint Exchange: $checkpoint_status
Replay Exchange: $replay_status
Settlement Exchange: $settlement_status
Synchronization: $synchronization_status
Recovery: $recovery_status

Federation Identity Root: $(federation_identity_root)
Membership Root: $(membership_root)
Topology Root: $(topology_root)
Checkpoint Exchange Root: $(checkpoint_exchange_root)
Replay Exchange Root: $(replay_exchange_root)
Settlement Exchange Root: $(settlement_exchange_root)
Civilization Synchronization Root: $(civilization_synchronization_root)
Recovery Root: $(recovery_root)
Recovered Replay Root: $(lease_replay_root "${FEDERATION_LEASE_IDS[1]}")
Recovered Continuity Root: $(lease_continuity_root "${FEDERATION_LEASE_IDS[1]}")

Multi-Lease Federation Runtime: $overall_status
REPORT

cat "$REPORT_PATH"
[[ "$overall_status" == "PASS" ]]
