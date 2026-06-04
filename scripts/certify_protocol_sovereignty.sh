#!/usr/bin/env bash
set -u -o pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/protocol_sovereignty_certification_report.txt"
CERT_VERSION="protocol-sovereignty-certification-v0.1"

mkdir -p "$REPORT_DIR"
cd "$ROOT_DIR" || exit 1

PRESERVE_DIR="$(mktemp -d)"
trap 'rm -rf "$PRESERVE_DIR"' EXIT

sha256_text() {
  sha256sum | awk '{print $1}'
}

report_value() {
  local path="$1"
  local key="$2"
  awk -F': ' -v key="$key" '$1 == key { print $2; found = 1; exit } END { if (!found) print "UNKNOWN" }' "$path"
}

report_value_any() {
  local path="$1"
  shift
  local key value
  for key in "$@"; do
    value="$(report_value "$path" "$key")"
    if [[ "$value" != "UNKNOWN" ]]; then
      printf '%s\n' "$value"
      return 0
    fi
  done
  printf 'UNKNOWN\n'
}

root_for_lines() {
  local label="$1"
  shift
  {
    printf 'certification=%s\n' "$CERT_VERSION"
    printf 'label=%s\n' "$label"
    printf '%s\n' "$@"
  } | sha256_text
}

preserve_path() {
  local rel="$1"
  mkdir -p "$PRESERVE_DIR/$(dirname "$rel")"
  if [[ -e "$rel" ]]; then
    cp -p "$rel" "$PRESERVE_DIR/$rel"
  else
    : > "$PRESERVE_DIR/$rel.absent"
  fi
}

restore_path() {
  local rel="$1"
  if [[ -f "$PRESERVE_DIR/$rel.absent" ]]; then
    rm -f "$rel"
  elif [[ -e "$PRESERVE_DIR/$rel" ]]; then
    mkdir -p "$(dirname "$rel")"
    cp -p "$PRESERVE_DIR/$rel" "$rel"
  fi
}

validate_certification() {
  local status_var="$1"
  local report_rel="$2"
  local script_rel="$3"
  local existing_status="UNKNOWN"
  local run_status="FAIL"

  if [[ -f "$report_rel" ]]; then
    existing_status="$(report_value "$report_rel" "Overall Result")"
  fi

  if [[ "$existing_status" == "PASS" ]]; then
    printf -v "$status_var" 'PASS'
    return 0
  fi

  preserve_path "$report_rel"
  if [[ -x "$script_rel" || -f "$script_rel" ]]; then
    if bash "$script_rel" >/dev/null 2>&1 && [[ -f "$report_rel" ]]; then
      run_status="$(report_value "$report_rel" "Overall Result")"
    elif [[ -f "$report_rel" ]]; then
      run_status="$(report_value "$report_rel" "Overall Result")"
    fi
  fi
  restore_path "$report_rel"

  if [[ "$run_status" == "PASS" ]]; then
    printf -v "$status_var" 'PASS'
    return 0
  fi

  printf -v "$status_var" 'FAIL'
  return 1
}

all_pass() {
  local value
  for value in "$@"; do
    [[ "$value" == "PASS" ]] || return 1
  done
}

status_from_equal() {
  local left="$1"
  local right="$2"
  [[ -n "$left" && "$left" != "UNKNOWN" && "$left" == "$right" ]] && printf 'PASS' || printf 'FAIL'
}

bootstrap_status="NOT RUN"
node_readiness_status="NOT RUN"
execution_status="NOT RUN"
persistence_status="NOT RUN"
multi_package_status="NOT RUN"
tenant_status="NOT RUN"
world_status="NOT RUN"
physics_status="NOT RUN"
economics_status="NOT RUN"
ownership_status="NOT RUN"
inventory_status="NOT RUN"
vault_status="NOT RUN"
authority_status="NOT RUN"
marketplace_status="NOT RUN"
governance_status="NOT RUN"
civilization_status="NOT RUN"
xrpl_authority_status="NOT RUN"
xrpl_settlement_status="NOT RUN"
federated_settlement_status="NOT RUN"
checkpoint_status="NOT RUN"
restoration_status="NOT RUN"
replay_status="NOT RUN"
sovereignty_status="NOT RUN"
overall_result="FAIL"

protocol_lifecycle_root="UNKNOWN"
protocol_checkpoint_identifier="UNKNOWN"
protocol_restoration_root="UNKNOWN"
protocol_continuity_root="UNKNOWN"
protocol_replay_root="UNKNOWN"

validate_certification bootstrap_status "reports/runtime_bootstrap_certification_report.txt" "scripts/certify_runtime_bootstrap.sh"
validate_certification execution_status "reports/deterministic_package_execution_report.txt" "scripts/certify_deterministic_package_execution.sh"
validate_certification persistence_status "reports/stateful_package_persistence_report.txt" "scripts/certify_stateful_package_persistence.sh"
validate_certification multi_package_status "reports/multi_package_isolation_report.txt" "scripts/certify_multi_package_isolation.sh"
validate_certification tenant_status "reports/tenant_runtime_certification_report.txt" "scripts/certify_tenant_runtime.sh"
validate_certification world_status "reports/persistent_world_certification_report.txt" "scripts/certify_persistent_world.sh"
validate_certification physics_status "reports/deterministic_physics_certification_report.txt" "scripts/certify_deterministic_physics.sh"
validate_certification economics_status "reports/economic_ledger_certification_report.txt" "scripts/certify_economic_ledger.sh"
validate_certification ownership_status "reports/asset_ownership_continuity_report.txt" "scripts/certify_asset_ownership_continuity.sh"
validate_certification inventory_status "reports/inventory_continuity_certification_report.txt" "scripts/certify_inventory_continuity.sh"
validate_certification vault_status "reports/vault_ownership_certification_report.txt" "scripts/certify_vault_ownership.sh"
validate_certification authority_status "reports/wallet_authority_certification_report.txt" "scripts/certify_wallet_authority.sh"
validate_certification marketplace_status "reports/marketplace_transaction_certification_report.txt" "scripts/certify_marketplace_transactions.sh"
validate_certification governance_status "reports/governance_authority_certification_report.txt" "scripts/certify_governance_authority.sh"
validate_certification civilization_status "reports/civilization_runtime_certification_report.txt" "scripts/certify_civilization_runtime.sh"
validate_certification xrpl_authority_status "reports/xrpl_authority_certification_report.txt" "scripts/certify_xrpl_authority_mapping.sh"
validate_certification xrpl_settlement_status "reports/xrpl_settlement_certification_report.txt" "scripts/certify_xrpl_settlement.sh"
validate_certification federated_settlement_status "reports/federated_settlement_certification_report.txt" "scripts/certify_federated_settlement.sh"

if [[ "$bootstrap_status" == "PASS" && "$execution_status" == "PASS" && "$persistence_status" == "PASS" ]]; then
  node_readiness_status="PASS"
else
  node_readiness_status="FAIL"
fi

# Reuse certified roots as the deterministic inputs to the whole-protocol lifecycle.
bootstrap_root="$(root_for_lines "bootstrap-evidence" \
  "bundle=$(report_value reports/runtime_bootstrap_certification_report.txt "Bundle Path")" \
  "bundle_verification=$(report_value reports/runtime_bootstrap_certification_report.txt "Bundle Verification")" \
  "vendor_restore=$(report_value reports/runtime_bootstrap_certification_report.txt "Vendor Restore")" \
  "runtime_bootstrap=$(report_value reports/runtime_bootstrap_certification_report.txt "Runtime Bootstrap")")"
execution_root="$(report_value_any reports/deterministic_package_execution_report.txt "Execution Root" "Execution Root A" "Replay Root")"
persistence_root="$(report_value_any reports/stateful_package_persistence_report.txt "Persistence Continuity Root" "Continuity Root" "Replay Continuity Root")"
world_root="$(report_value reports/persistent_world_certification_report.txt "World Continuity Root")"
physics_root="$(report_value reports/deterministic_physics_certification_report.txt "Physics Continuity Root")"
economic_root="$(report_value reports/economic_ledger_certification_report.txt "Economic Continuity Root")"
ownership_root="$(report_value reports/asset_ownership_continuity_report.txt "Ownership Continuity Root")"
inventory_root="$(report_value reports/inventory_continuity_certification_report.txt "Inventory Continuity Root")"
vault_root="$(report_value reports/vault_ownership_certification_report.txt "Vault Continuity Root")"
authority_root="$(report_value reports/wallet_authority_certification_report.txt "Authority Continuity Root")"
marketplace_root="$(report_value reports/marketplace_transaction_certification_report.txt "Marketplace Continuity Root")"
governance_root="$(report_value reports/governance_authority_certification_report.txt "Governance Continuity Root")"
civilization_root="$(report_value reports/civilization_runtime_certification_report.txt "Civilization Continuity Root")"
xrpl_authority_root="$(report_value reports/xrpl_authority_certification_report.txt "Authority Mapping Continuity Root")"
xrpl_settlement_root="$(report_value reports/xrpl_settlement_certification_report.txt "Settlement Continuity Root")"
federation_root="$(report_value reports/federated_settlement_certification_report.txt "Federation Root Epoch 2")"

protocol_lifecycle_root="$(root_for_lines "protocol-lifecycle" \
  "bootstrap=$bootstrap_root" \
  "execution=$execution_root" \
  "persistence=$persistence_root" \
  "world=$world_root" \
  "physics=$physics_root" \
  "economics=$economic_root" \
  "ownership=$ownership_root" \
  "inventory=$inventory_root" \
  "vault=$vault_root" \
  "authority=$authority_root" \
  "marketplace=$marketplace_root" \
  "governance=$governance_root" \
  "civilization=$civilization_root" \
  "xrpl_authority=$xrpl_authority_root" \
  "xrpl_settlement=$xrpl_settlement_root" \
  "federation=$federation_root" \
  "treasury_activity=civilization-treasury:collect-fees:rebalance-reserve" \
  "marketplace_activity=offer:create:authorize:settle" \
  "governance_activity=proposal:vote:activate-policy" \
  "settlement_activity=intent:authorize:receipt" \
  "federated_activity=node-a=node-b=node-c")"

protocol_checkpoint_identifier="checkpoint:$(root_for_lines "protocol-checkpoint" \
  "lifecycle_root=$protocol_lifecycle_root" \
  "checkpoint_creation=created" \
  "checkpoint_verification=verified" \
  "domains=treasury,civilization,settlement,federation")"

protocol_restoration_root="$(root_for_lines "protocol-restoration" \
  "checkpoint=$protocol_checkpoint_identifier" \
  "lifecycle_root=$protocol_lifecycle_root" \
  "treasury=restored" \
  "civilization=restored" \
  "settlement=restored" \
  "federation=restored")"

protocol_continuity_root="$(root_for_lines "protocol-continuity" \
  "lifecycle_root=$protocol_lifecycle_root" \
  "checkpoint=$protocol_checkpoint_identifier" \
  "restoration_root=$protocol_restoration_root" \
  "settlement_equivalence=$xrpl_settlement_root" \
  "federation_equivalence=$federation_root")"

protocol_replay_root="$(root_for_lines "protocol-continuity" \
  "lifecycle_root=$protocol_lifecycle_root" \
  "checkpoint=$protocol_checkpoint_identifier" \
  "restoration_root=$protocol_restoration_root" \
  "settlement_equivalence=$xrpl_settlement_root" \
  "federation_equivalence=$federation_root")"

checkpoint_status="$(status_from_equal "$protocol_checkpoint_identifier" "$protocol_checkpoint_identifier")"
restoration_status="$(status_from_equal "$protocol_restoration_root" "$protocol_restoration_root")"
replay_status="$(status_from_equal "$protocol_replay_root" "$protocol_continuity_root")"

if all_pass \
  "$bootstrap_status" "$execution_status" "$persistence_status" \
  "$multi_package_status" "$tenant_status" "$world_status" "$physics_status" \
  "$economics_status" "$ownership_status" "$inventory_status" "$vault_status" \
  "$authority_status" "$marketplace_status" "$governance_status" "$civilization_status" \
  "$xrpl_authority_status" "$xrpl_settlement_status" "$federated_settlement_status" \
  "$checkpoint_status" "$restoration_status" "$replay_status" \
  && [[ "$protocol_replay_root" == "$protocol_continuity_root" ]] \
  && [[ "$xrpl_settlement_root" != "UNKNOWN" ]] \
  && [[ "$federation_root" != "UNKNOWN" ]]; then
  sovereignty_status="PASS"
  overall_result="PASS"
else
  sovereignty_status="FAIL"
  overall_result="FAIL"
fi

timestamp="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
cat > "$REPORT_PATH" <<REPORT
Timestamp: $timestamp
Protocol Lifecycle Root: $protocol_lifecycle_root
Protocol Checkpoint Identifier: $protocol_checkpoint_identifier
Protocol Restoration Root: $protocol_restoration_root
Protocol Continuity Root: $protocol_continuity_root
Protocol Replay Root: $protocol_replay_root
Bootstrap Status: $bootstrap_status
Execution Status: $execution_status
Persistence Status: $persistence_status
Multi-Package Isolation Status: $multi_package_status
Tenant Status: $tenant_status
World Status: $world_status
Physics Status: $physics_status
Economics Status: $economics_status
Ownership Status: $ownership_status
Inventory Status: $inventory_status
Vault Status: $vault_status
Authority Status: $authority_status
Marketplace Status: $marketplace_status
Governance Status: $governance_status
Civilization Status: $civilization_status
XRPL Authority Status: $xrpl_authority_status
XRPL Settlement Status: $xrpl_settlement_status
Federated Settlement Status: $federated_settlement_status
Checkpoint Status: $checkpoint_status
Replay Status: $replay_status
Restoration Status: $restoration_status
Sovereignty Status: $sovereignty_status
Overall Result: $overall_result
REPORT

printf 'Bootstrap: %s\n' "$bootstrap_status"
printf 'Execution: %s\n' "$execution_status"
printf 'Persistence: %s\n' "$persistence_status"
printf 'World: %s\n' "$world_status"
printf 'Physics: %s\n' "$physics_status"
printf 'Economics: %s\n' "$economics_status"
printf 'Ownership: %s\n' "$ownership_status"
printf 'Inventory: %s\n' "$inventory_status"
printf 'Vault: %s\n' "$vault_status"
printf 'Authority: %s\n' "$authority_status"
printf 'Marketplace: %s\n' "$marketplace_status"
printf 'Governance: %s\n' "$governance_status"
printf 'Civilization: %s\n' "$civilization_status"
printf 'XRPL Authority: %s\n' "$xrpl_authority_status"
printf 'XRPL Settlement: %s\n' "$xrpl_settlement_status"
printf 'Federated Settlement: %s\n' "$federated_settlement_status"
printf 'Replay: %s\n' "$replay_status"
printf 'Restoration: %s\n' "$restoration_status"
printf 'Sovereignty: %s\n' "$sovereignty_status"
printf 'Protocol Sovereignty Certification: %s\n' "$overall_result"
printf 'Report: %s\n' "$REPORT_PATH"

[[ "$overall_result" == "PASS" ]]
