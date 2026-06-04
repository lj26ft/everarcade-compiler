#!/usr/bin/env bash
# EverArcade Public Testnet v0.1 deterministic operational model.
set -euo pipefail

PUBLIC_TESTNET_VERSION="0.1"
PUBLIC_TESTNET_ID="everarcade-public-testnet-v0.1"
PUBLIC_TESTNET_NETWORK="xrpl-testnet-only"
PUBLIC_TESTNET_FUNDS="no-production-funds"
PUBLIC_TESTNET_ORDERING="participant-id-record-id-action-lexicographic"
PUBLIC_TESTNET_AUTHORITY="auditable-testnet-operations"

sha256_text() { sha256sum | awk '{print $1}'; }
validate_hash() { [[ "$1" =~ ^[0-9a-f]{64}$ ]]; }

common_transcript() {
  printf 'testnet_id=%s\n' "$PUBLIC_TESTNET_ID"
  printf 'version=%s\n' "$PUBLIC_TESTNET_VERSION"
  printf 'network=%s\n' "$PUBLIC_TESTNET_NETWORK"
  printf 'funds=%s\n' "$PUBLIC_TESTNET_FUNDS"
  printf 'ordering=%s\n' "$PUBLIC_TESTNET_ORDERING"
  printf 'authority=%s\n' "$PUBLIC_TESTNET_AUTHORITY"
}

developer_enrollment_transcript() {
  common_transcript
  cat <<'RECORDS'
developer=developer-alpha|registration=accepted|contact=forum-alpha|status=active
project=project-arena|developer=developer-alpha|registration=accepted|approval=approved|status=deployable
project_status=project-arena|build=certified-runtime|lease=assigned|player_access=open-test

developer=developer-beta|registration=accepted|contact=forum-beta|status=active
project=project-civ|developer=developer-beta|registration=accepted|approval=approved|status=deployable
project_status=project-civ|build=certified-runtime|lease=federated|player_access=limited-test

developer=developer-gamma|registration=accepted|contact=forum-gamma|status=active
project=project-render|developer=developer-gamma|registration=accepted|approval=approved|status=gpu-validation
project_status=project-render|build=certified-renderer|lease=observer|player_access=closed-test
RECORDS
}
developer_enrollment_root() { if [[ -n "${DEVELOPER_ENROLLMENT_ROOT_CACHE:-}" ]]; then printf '%s\n' "$DEVELOPER_ENROLLMENT_ROOT_CACHE"; else developer_enrollment_transcript | sha256_text; fi; }

operator_enrollment_transcript() {
  common_transcript
  cat <<'RECORDS'
operator=lease-operator-east|role=lease-operator|lease=lease-east-001|node=node-east-01|status=active
operator=node-operator-west|role=node-operator|lease=lease-west-001|node=node-west-01|status=active
operator=federation-operator-core|role=federation-operator|federation=federation-core-001|node=node-core-01|status=active
operator_record=lease-operator-east|health=pass|checkpoint=checkpoint-east-64|replay=replay-east-64
operator_record=node-operator-west|health=pass|checkpoint=checkpoint-west-71|replay=replay-west-71
operator_record=federation-operator-core|health=pass|checkpoint=checkpoint-core-99|replay=replay-core-99
RECORDS
}
operator_enrollment_root() { if [[ -n "${OPERATOR_ENROLLMENT_ROOT_CACHE:-}" ]]; then printf '%s\n' "$OPERATOR_ENROLLMENT_ROOT_CACHE"; else operator_enrollment_transcript | sha256_text; fi; }

gpu_enrollment_transcript() {
  common_transcript
  cat <<'RECORDS'
provider=gpu-provider-a|registration=accepted|capability=projection-rendering|capacity=4-jobs|status=active
provider=gpu-provider-b|registration=accepted|capability=artifact-verification|capacity=2-jobs|status=active
provider=gpu-provider-c|registration=accepted|capability=renderer-benchmark|capacity=1-job|status=standby
capability_advertisement=gpu-provider-a|device=deterministic-gpu-class-a|driver=certified-test-stack
capability_advertisement=gpu-provider-b|device=deterministic-gpu-class-b|driver=certified-test-stack
capacity_declaration=gpu-provider-a|max_parallel_jobs=4|settlement=intent-only
capacity_declaration=gpu-provider-b|max_parallel_jobs=2|settlement=intent-only
RECORDS
}
gpu_enrollment_root() { if [[ -n "${GPU_ENROLLMENT_ROOT_CACHE:-}" ]]; then printf '%s\n' "$GPU_ENROLLMENT_ROOT_CACHE"; else gpu_enrollment_transcript | sha256_text; fi; }

deployment_registry_transcript() {
  common_transcript
  printf 'developer_enrollment_root=%s\n' "$(developer_enrollment_root)"
  printf 'operator_enrollment_root=%s\n' "$(operator_enrollment_root)"
  cat <<'RECORDS'
project=project-arena|deployment=deploy-arena-001|lease=lease-east-001|federation=federation-core-001|status=running
project=project-civ|deployment=deploy-civ-001|lease=lease-west-001|federation=federation-core-001|status=running
project=project-render|deployment=deploy-render-001|lease=lease-observer-001|federation=federation-observer-001|status=validating
membership=federation-core-001|lease=lease-east-001|operator=lease-operator-east|status=active
membership=federation-core-001|lease=lease-west-001|operator=node-operator-west|status=active
RECORDS
}
deployment_registry_root() { if [[ -n "${DEPLOYMENT_REGISTRY_ROOT_CACHE:-}" ]]; then printf '%s\n' "$DEPLOYMENT_REGISTRY_ROOT_CACHE"; else deployment_registry_transcript | sha256_text; fi; }

civilization_registry_transcript() {
  common_transcript
  printf 'deployment_registry_root=%s\n' "$(deployment_registry_root)"
  cat <<'RECORDS'
civilization=civ-genesis|world=world-genesis|region=region-north|governance_state=open-council|economy_state=test-credit
civilization=civ-arcade|world=world-arena|region=region-south|governance_state=operator-council|economy_state=test-ticket
world=world-genesis|status=active|replay=replay-world-genesis-12
region=region-north|status=active|shard=shard-north-01
RECORDS
}
civilization_registry_root() { if [[ -n "${CIVILIZATION_REGISTRY_ROOT_CACHE:-}" ]]; then printf '%s\n' "$CIVILIZATION_REGISTRY_ROOT_CACHE"; else civilization_registry_transcript | sha256_text; fi; }

settlement_test_transcript() {
  common_transcript
  printf 'deployment_registry_root=%s\n' "$(deployment_registry_root)"
  cat <<'RECORDS'
settlement_intent=intent-0001|project=project-arena|amount=100-test-drops|production_funds=false|status=authorized-test
xrpl_test_settlement=xrpl-test-0001|intent=intent-0001|network=testnet|ledger=observed-test-ledger|status=observed
xaman_test_authorization=xaman-test-0001|intent=intent-0001|wallet=xaman-test-alpha|status=signed-test
receipt_import=receipt-0001|settlement=xrpl-test-0001|imported=true|status=verified
settlement_intent=intent-0002|project=project-civ|amount=250-test-drops|production_funds=false|status=authorized-test
receipt_import=receipt-0002|settlement=xrpl-test-0002|imported=true|status=verified
RECORDS
}
settlement_test_root() { if [[ -n "${SETTLEMENT_TEST_ROOT_CACHE:-}" ]]; then printf '%s\n' "$SETTLEMENT_TEST_ROOT_CACHE"; else settlement_test_transcript | sha256_text; fi; }

gpu_test_transcript() {
  common_transcript
  printf 'gpu_enrollment_root=%s\n' "$(gpu_enrollment_root)"
  printf 'settlement_test_root=%s\n' "$(settlement_test_root)"
  cat <<'RECORDS'
gpu_job=job-0001|provider=gpu-provider-a|artifact=artifact-projection-0001|verification=verification-0001|settlement_intent=intent-gpu-0001|status=verified
gpu_job=job-0002|provider=gpu-provider-b|artifact=artifact-render-0002|verification=verification-0002|settlement_intent=intent-gpu-0002|status=verified
artifact=artifact-projection-0001|hash=deterministic-artifact-a|replay=replay-gpu-0001
artifact=artifact-render-0002|hash=deterministic-artifact-b|replay=replay-gpu-0002
verification=verification-0001|result=pass|observer=operator-core
verification=verification-0002|result=pass|observer=operator-core
RECORDS
}
gpu_test_root() { if [[ -n "${GPU_TEST_ROOT_CACHE:-}" ]]; then printf '%s\n' "$GPU_TEST_ROOT_CACHE"; else gpu_test_transcript | sha256_text; fi; }

governance_transcript() {
  common_transcript
  cat <<'RECORDS'
proposal=proposal-0001|title=expand-gpu-provider-window|status=passed|policy=policy-gpu-window-v1
vote=vote-0001|proposal=proposal-0001|voter=developer-alpha|choice=yes|weight=testnet-one
vote=vote-0002|proposal=proposal-0001|voter=lease-operator-east|choice=yes|weight=testnet-one
policy=policy-gpu-window-v1|rule_change=allow-standby-provider-burst|status=active-testnet
rule_change=rule-0001|proposal=proposal-0001|auditable=true|replayable=true
RECORDS
}
governance_root() { if [[ -n "${GOVERNANCE_ROOT_CACHE:-}" ]]; then printf '%s\n' "$GOVERNANCE_ROOT_CACHE"; else governance_transcript | sha256_text; fi; }

analytics_transcript() {
  common_transcript
  printf 'developer_enrollment_root=%s\n' "$(developer_enrollment_root)"
  printf 'deployment_registry_root=%s\n' "$(deployment_registry_root)"
  printf 'civilization_registry_root=%s\n' "$(civilization_registry_root)"
  printf 'gpu_test_root=%s\n' "$(gpu_test_root)"
  printf 'settlement_test_root=%s\n' "$(settlement_test_root)"
  printf 'operator_enrollment_root=%s\n' "$(operator_enrollment_root)"
  cat <<'RECORDS'
metric=developers|count=3|source=developer-enrollment
metric=deployments|count=3|source=deployment-registry
metric=civilizations|count=2|source=civilization-registry
metric=gpu_jobs|count=2|source=gpu-marketplace-test
metric=settlement_events|count=2|source=settlement-test
metric=operator_activity|count=3|source=operator-enrollment
RECORDS
}
analytics_root() { if [[ -n "${ANALYTICS_ROOT_CACHE:-}" ]]; then printf '%s\n' "$ANALYTICS_ROOT_CACHE"; else analytics_transcript | sha256_text; fi; }

testnet_root() {
  {
    common_transcript
    printf 'developer_enrollment_root=%s\n' "$(developer_enrollment_root)"
    printf 'operator_enrollment_root=%s\n' "$(operator_enrollment_root)"
    printf 'gpu_enrollment_root=%s\n' "$(gpu_enrollment_root)"
    printf 'deployment_registry_root=%s\n' "$(deployment_registry_root)"
    printf 'civilization_registry_root=%s\n' "$(civilization_registry_root)"
    printf 'settlement_test_root=%s\n' "$(settlement_test_root)"
    printf 'gpu_test_root=%s\n' "$(gpu_test_root)"
    printf 'governance_root=%s\n' "$(governance_root)"
    printf 'analytics_root=%s\n' "$(analytics_root)"
  } | sha256_text
}

replay_transcript() {
  common_transcript
  printf 'replay=enrollment|root=%s\n' "$( { developer_enrollment_root; operator_enrollment_root; gpu_enrollment_root; } | sha256_text )"
  printf 'replay=deployments|root=%s\n' "$(deployment_registry_root)"
  printf 'replay=civilizations|root=%s\n' "$(civilization_registry_root)"
  printf 'replay=gpu_activity|root=%s\n' "$(gpu_test_root)"
  printf 'replay=settlement_activity|root=%s\n' "$(settlement_test_root)"
  printf 'replay=governance_activity|root=%s\n' "$(governance_root)"
  printf 'testnet_root=%s\n' "$(testnet_root)"
}
testnet_replay_root() { if [[ -n "${PUBLIC_TESTNET_ROOT_CACHE:-}" ]]; then printf '%s\n' "$PUBLIC_TESTNET_ROOT_CACHE"; else testnet_root; fi; }

public_testnet_compute_roots() {
  DEVELOPER_ENROLLMENT_ROOT_CACHE="$(developer_enrollment_transcript | sha256_text)"
  OPERATOR_ENROLLMENT_ROOT_CACHE="$(operator_enrollment_transcript | sha256_text)"
  GPU_ENROLLMENT_ROOT_CACHE="$(gpu_enrollment_transcript | sha256_text)"
  DEPLOYMENT_REGISTRY_ROOT_CACHE="$(deployment_registry_transcript | sha256_text)"
  CIVILIZATION_REGISTRY_ROOT_CACHE="$(civilization_registry_transcript | sha256_text)"
  SETTLEMENT_TEST_ROOT_CACHE="$(settlement_test_transcript | sha256_text)"
  GPU_TEST_ROOT_CACHE="$(gpu_test_transcript | sha256_text)"
  GOVERNANCE_ROOT_CACHE="$(governance_transcript | sha256_text)"
  ANALYTICS_ROOT_CACHE="$(analytics_transcript | sha256_text)"
  PUBLIC_TESTNET_ROOT_CACHE="$(testnet_root)"
}

public_testnet_write_artifacts() {
  public_testnet_compute_roots
  local root="${PUBLIC_TESTNET_ROOT:-public-testnet}"
  mkdir -p "$root"/{developers,operators,gpu-providers,players,deployments,governance,reports,civilizations,settlements,gpu-marketplace,analytics,replay,records}
  developer_enrollment_transcript > "$root/developers/developer_enrollment.records"
  operator_enrollment_transcript > "$root/operators/operator_enrollment.records"
  gpu_enrollment_transcript > "$root/gpu-providers/gpu_provider_enrollment.records"
  deployment_registry_transcript > "$root/deployments/deployment_registry.records"
  civilization_registry_transcript > "$root/civilizations/civilization_registry.records"
  settlement_test_transcript > "$root/settlements/settlement_test.records"
  gpu_test_transcript > "$root/gpu-marketplace/gpu_marketplace_test.records"
  governance_transcript > "$root/governance/governance.records"
  analytics_transcript > "$root/analytics/analytics.records"
  replay_transcript > "$root/replay/testnet_replay.records"
  cat > "$root/players/player_access.records" <<PLAYERS
$(common_transcript)
player=player-alpha|project=project-arena|access=open-test|status=active
player=player-beta|project=project-civ|access=limited-test|status=active
player=player-gamma|project=project-render|access=closed-test-observer|status=active
PLAYERS
  cat > "$root/records/roots.env" <<ROOTS
DEVELOPER_ENROLLMENT_ROOT=$(developer_enrollment_root)
OPERATOR_ENROLLMENT_ROOT=$(operator_enrollment_root)
GPU_ENROLLMENT_ROOT=$(gpu_enrollment_root)
DEPLOYMENT_REGISTRY_ROOT=$(deployment_registry_root)
CIVILIZATION_REGISTRY_ROOT=$(civilization_registry_root)
SETTLEMENT_TEST_ROOT=$(settlement_test_root)
GPU_TEST_ROOT=$(gpu_test_root)
GOVERNANCE_ROOT=$(governance_root)
ANALYTICS_ROOT=$(analytics_root)
TESTNET_REPLAY_ROOT=$(testnet_replay_root)
PUBLIC_TESTNET_ROOT_HASH=$(testnet_root)
ROOTS
}

validate_developers() { developer_enrollment_transcript | grep -q 'developer=developer-alpha' && developer_enrollment_transcript | grep -q 'approval=approved' && validate_hash "$(developer_enrollment_root)"; }
validate_operators() { for role in lease-operator node-operator federation-operator; do operator_enrollment_transcript | grep -q "role=$role"; done && validate_hash "$(operator_enrollment_root)"; }
validate_gpu_providers() { gpu_enrollment_transcript | grep -q 'capability_advertisement=' && gpu_enrollment_transcript | grep -q 'capacity_declaration=' && validate_hash "$(gpu_enrollment_root)"; }
validate_deployments() { deployment_registry_transcript | grep -q 'deployment=deploy-arena-001' && deployment_registry_transcript | grep -q 'federation=' && validate_hash "$(deployment_registry_root)"; }
validate_civilizations() { civilization_registry_transcript | grep -q 'governance_state=' && civilization_registry_transcript | grep -q 'economy_state=' && validate_hash "$(civilization_registry_root)"; }
validate_settlements() { settlement_test_transcript | grep -q 'production_funds=false' && settlement_test_transcript | grep -q 'xaman_test_authorization=' && validate_hash "$(settlement_test_root)"; }
validate_gpu_marketplace() { gpu_test_transcript | grep -q 'gpu_job=' && gpu_test_transcript | grep -q 'verification=' && validate_hash "$(gpu_test_root)"; }
validate_governance() { governance_transcript | grep -q 'proposal=' && governance_transcript | grep -q 'rule_change=' && validate_hash "$(governance_root)"; }
validate_analytics_public_testnet() { for metric in developers deployments civilizations gpu_jobs settlement_events operator_activity; do analytics_transcript | grep -q "metric=$metric"; done && validate_hash "$(analytics_root)"; }
validate_replay() { [[ "$(testnet_replay_root)" == "$(testnet_root)" ]] && grep -q 'replay=governance_activity' <<< "$(replay_transcript)" && validate_hash "$(testnet_replay_root)"; }
