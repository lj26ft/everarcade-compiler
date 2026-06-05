#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

REPORT="reports/commercial_revenue_validation_report.txt"
MODEL="commercial-revenue/commercial_revenue_model.sh"
mkdir -p reports

# shellcheck source=/dev/null
source "$MODEL"
init_commercial_revenue

pass=true
status() {
  local name="$1" result="$2"
  printf '%s: %s\n' "$name" "$result"
  [[ "$result" == "PASS" ]] || pass=false
}

hash_file_ok() {
  local file="$1" expected="$2"
  [[ -f "$file" ]] || return 1
  local actual
  actual="$(<"$file")"
  validate_hash "$actual" && [[ "$actual" == "$expected" ]]
}

records_have() {
  local file="$1"; shift
  [[ -f "$file" ]] || return 1
  local pattern
  for pattern in "$@"; do
    grep -q "$pattern" "$file" || return 1
  done
}

{
  printf 'EverArcade Commercial Revenue Validation Report\n'
  printf 'Version: %s\n' "$COMMERCIAL_REVENUE_VERSION"
  printf 'Custody: %s\n' "$COMMERCIAL_REVENUE_CUSTODY"
  printf 'Payments: %s\n' "$COMMERCIAL_REVENUE_PAYMENTS"
  printf '\n'

  if hash_file_ok commercial-revenue/registry/REVENUE_REGISTRY_ROOT "$(registry_root)" \
    && records_have commercial-revenue/registry/revenue_registry.records 'operation=Create' 'operation=Update' 'operation=Archive' 'event_id=' 'revenue_type=' 'origin=' 'beneficiary=' 'status=' 'epoch='; then
    status 'Registry' PASS
  else status 'Registry' FAIL; fi

  if hash_file_ok commercial-revenue/creator/CREATOR_REVENUE_ROOT "$(creator_revenue_root)" \
    && records_have commercial-revenue/creator/creator_revenue.records 'Template Revenue' 'Asset Revenue' 'Marketplace Revenue' 'Subscription Revenue' 'Royalty Revenue' 'creator_attribution='; then
    status 'Creator Revenue' PASS
  else status 'Creator Revenue' FAIL; fi

  if hash_file_ok commercial-revenue/operator/OPERATOR_REVENUE_ROOT "$(operator_revenue_root)" \
    && records_have commercial-revenue/operator/operator_revenue.records 'Lease Revenue' 'Hosting Revenue' 'Federation Revenue' 'Runtime Revenue'; then
    status 'Operator Revenue' PASS
  else status 'Operator Revenue' FAIL; fi

  if hash_file_ok commercial-revenue/gpu/GPU_REVENUE_ROOT "$(gpu_revenue_root)" \
    && records_have commercial-revenue/gpu/gpu_revenue.records 'GPU Job Revenue' 'Artifact Revenue' 'Verification Revenue' 'Capacity Revenue' 'gpu/marketplace'; then
    status 'GPU Revenue' PASS
  else status 'GPU Revenue' FAIL; fi

  if hash_file_ok commercial-revenue/civilization/CIVILIZATION_REVENUE_ROOT "$(civilization_revenue_root)" \
    && records_have commercial-revenue/civilization/civilization_revenue.records 'Civilization Treasury' 'Civilization Fees' 'Civilization Events' 'Civilization Revenue'; then
    status 'Civilization Revenue' PASS
  else status 'Civilization Revenue' FAIL; fi

  if hash_file_ok commercial-revenue/marketplace/MARKETPLACE_REVENUE_ROOT "$(marketplace_revenue_root)" \
    && records_have commercial-revenue/marketplace/marketplace_revenue.records 'Sales' 'Listings' 'Royalties' 'Marketplace Fees'; then
    status 'Marketplace Revenue' PASS
  else status 'Marketplace Revenue' FAIL; fi

  if hash_file_ok commercial-revenue/protocol/PROTOCOL_REVENUE_ROOT "$(protocol_revenue_root)" \
    && records_have commercial-revenue/protocol/protocol_revenue.records 'Protocol Fee Intent' 'Protocol Sustainability Intent' 'Network Revenue' 'live_fee_collection=false'; then
    status 'Protocol Revenue' PASS
  else status 'Protocol Revenue' FAIL; fi

  if hash_file_ok commercial-revenue/distribution/DISTRIBUTION_ROOT "$(distribution_root)" \
    && records_have commercial-revenue/distribution/distribution.records 'beneficiary_type=Creator' 'beneficiary_type=Operator' 'beneficiary_type=GPU Provider' 'beneficiary_type=Civilization' 'beneficiary_type=Protocol' 'distribution_intent='; then
    status 'Distribution' PASS
  else status 'Distribution' FAIL; fi

  if hash_file_ok commercial-revenue/settlement-intents/SETTLEMENT_INTENT_ROOT "$(settlement_intent_root)" \
    && records_have commercial-revenue/settlement-intents/settlement_intents.records 'Revenue Settlement' 'Royalty Settlement' 'Distribution Settlement' 'xrpl/intent' 'live_transfers=false'; then
    status 'Settlement Intents' PASS
  else status 'Settlement Intents' FAIL; fi

  if hash_file_ok commercial-revenue/analytics/ANALYTICS_ROOT "$(analytics_root)" \
    && records_have commercial-revenue/analytics/revenue_analytics.records 'Creator Revenue' 'Operator Revenue' 'GPU Revenue' 'Civilization Revenue' 'Marketplace Revenue' 'Protocol Revenue'; then
    status 'Analytics' PASS
  else status 'Analytics' FAIL; fi

  if hash_file_ok commercial-revenue/replay/REPLAY_ROOT "$(replay_root)" \
    && [[ "$(replay_root)" == "$(revenue_root)" ]]; then
    status 'Replay' PASS
  else status 'Replay' FAIL; fi

  if hash_file_ok commercial-revenue/portal/PORTAL_REVENUE_ROOT "$(portal_revenue_root)" \
    && records_have commercial-revenue/portal/portal_revenue.records 'Creator Earnings' 'Operator Earnings' 'GPU Earnings' 'Civilization Earnings' 'Protocol Metrics' 'developer-portal/dashboard' 'creator-marketplace/portal' 'player-gateway'; then
    status 'Portal Integration' PASS
  else status 'Portal Integration' FAIL; fi

  if hash_file_ok commercial-revenue/metrics/REVENUE_METRICS_ROOT "$(metrics_root)" \
    && records_have commercial-revenue/metrics/revenue_metrics.records 'Gross Revenue' 'Creator Share' 'Operator Share' 'GPU Share' 'Civilization Share' 'Protocol Share'; then
    status 'Metrics' PASS
  else status 'Metrics' FAIL; fi

  if hash_file_ok commercial-revenue/economics/ECONOMIC_SUSTAINABILITY_ROOT "$(economic_sustainability_root)" \
    && records_have commercial-revenue/economics/economic_sustainability.records 'creator_share=policy-defined' 'operator_share=policy-defined' 'gpu_share=policy-defined' 'civilization_share=policy-defined' 'protocol_share=policy-defined' 'hardcoded_production_percentage=false'; then
    status 'Economic Sustainability' PASS
  else status 'Economic Sustainability' FAIL; fi

  printf '\nRevenue Root: %s\n' "$(revenue_root)"
  printf 'Distribution Root: %s\n' "$(distribution_root)"
  printf 'Settlement Root: %s\n' "$(settlement_intent_root)"
  printf 'Replay Root: %s\n' "$(replay_root)"

  if [[ "$pass" == true ]]; then
    printf 'Commercial Revenue Validation: PASS\n'
  else
    printf 'Commercial Revenue Validation: FAIL\n'
    exit 1
  fi
} | tee "$REPORT"
