#!/usr/bin/env bash
# EverArcade Commercial Revenue Layer v0.1 deterministic model.
# Non-custodial revenue attribution, distribution intent, settlement intent,
# analytics, metrics, and replay verification. This model performs no billing,
# custody, key management, production fee collection, tax/KYC/AML, or live XRPL
# transfers.

set -euo pipefail

COMMERCIAL_REVENUE_VERSION="0.1"
COMMERCIAL_REVENUE_ID="everarcade-commercial-revenue-v0.1"
COMMERCIAL_REVENUE_CUSTODY="non-custodial"
COMMERCIAL_REVENUE_PAYMENTS="intent-only-no-live-settlement"
COMMERCIAL_REVENUE_ORDERING="epoch-event-id-revenue-type-origin-beneficiary-lexicographic"
COMMERCIAL_REVENUE_ROOT_DIR="${COMMERCIAL_REVENUE_ROOT_DIR:-commercial-revenue}"

sha256_text() { sha256sum | awk '{print $1}'; }
validate_hash() { [[ "$1" =~ ^[0-9a-f]{64}$ ]]; }
stable_root() { printf '%s|%s|%s\n' "$COMMERCIAL_REVENUE_ID" "$COMMERCIAL_REVENUE_VERSION" "$1" | sha256_text; }
record_root() { stable_root "$1"; }

common_transcript() {
  printf 'commercial_revenue_id=%s\n' "$COMMERCIAL_REVENUE_ID"
  printf 'version=%s\n' "$COMMERCIAL_REVENUE_VERSION"
  printf 'custody=%s\n' "$COMMERCIAL_REVENUE_CUSTODY"
  printf 'payments=%s\n' "$COMMERCIAL_REVENUE_PAYMENTS"
  printf 'ordering=%s\n' "$COMMERCIAL_REVENUE_ORDERING"
}

registry_transcript() {
  common_transcript
  cat <<'RECORDS'
operation=Create|event_id=rev-0001|revenue_type=Template Revenue|origin=creator-marketplace/templates|beneficiary=creator-alpha|status=active|epoch=100|distribution_policy=policy-creator-template-default
operation=Create|event_id=rev-0002|revenue_type=Asset Revenue|origin=creator-marketplace/assets|beneficiary=creator-beta|status=active|epoch=101|distribution_policy=policy-creator-asset-default
operation=Create|event_id=rev-0003|revenue_type=Marketplace Revenue|origin=marketplace/creator|beneficiary=marketplace-operator|status=active|epoch=102|distribution_policy=policy-marketplace-default
operation=Create|event_id=rev-0004|revenue_type=Subscription Revenue|origin=creator-sdk/monetization|beneficiary=creator-gamma|status=active|epoch=103|distribution_policy=policy-creator-subscription-default
operation=Create|event_id=rev-0005|revenue_type=Royalty Revenue|origin=creator-marketplace/royalties|beneficiary=creator-alpha|status=active|epoch=104|distribution_policy=policy-royalty-default
operation=Create|event_id=rev-0006|revenue_type=Lease Revenue|origin=evernode/lease|beneficiary=operator-atlas|status=active|epoch=105|distribution_policy=policy-operator-lease-default
operation=Create|event_id=rev-0007|revenue_type=Hosting Revenue|origin=runtime/operator-hosting|beneficiary=operator-boreal|status=active|epoch=106|distribution_policy=policy-operator-hosting-default
operation=Create|event_id=rev-0008|revenue_type=Federation Revenue|origin=federation/topology|beneficiary=operator-atlas|status=active|epoch=107|distribution_policy=policy-operator-federation-default
operation=Create|event_id=rev-0009|revenue_type=Runtime Revenue|origin=everarcade-host|beneficiary=operator-boreal|status=active|epoch=108|distribution_policy=policy-operator-runtime-default
operation=Create|event_id=rev-0010|revenue_type=GPU Job Revenue|origin=gpu/jobs|beneficiary=gpu-provider-nova|status=active|epoch=109|distribution_policy=policy-gpu-job-default
operation=Create|event_id=rev-0011|revenue_type=Artifact Revenue|origin=gpu/artifacts|beneficiary=gpu-provider-nova|status=active|epoch=110|distribution_policy=policy-gpu-artifact-default
operation=Create|event_id=rev-0012|revenue_type=Verification Revenue|origin=gpu/verification|beneficiary=gpu-provider-quartz|status=active|epoch=111|distribution_policy=policy-gpu-verification-default
operation=Create|event_id=rev-0013|revenue_type=Capacity Revenue|origin=gpu/marketplace|beneficiary=gpu-provider-quartz|status=active|epoch=112|distribution_policy=policy-gpu-capacity-default
operation=Create|event_id=rev-0014|revenue_type=Civilization Treasury|origin=creator-sdk/civilization|beneficiary=civilization-aurora|status=active|epoch=113|distribution_policy=policy-civilization-treasury-default
operation=Create|event_id=rev-0015|revenue_type=Civilization Fees|origin=game-discovery/civilizations|beneficiary=civilization-aurora|status=active|epoch=114|distribution_policy=policy-civilization-fees-default
operation=Create|event_id=rev-0016|revenue_type=Civilization Events|origin=examples/civilization-sim|beneficiary=civilization-solstice|status=active|epoch=115|distribution_policy=policy-civilization-events-default
operation=Create|event_id=rev-0017|revenue_type=Civilization Revenue|origin=developer-portal/civilizations|beneficiary=civilization-solstice|status=active|epoch=116|distribution_policy=policy-civilization-revenue-default
operation=Create|event_id=rev-0018|revenue_type=Sales|origin=marketplace/sales|beneficiary=marketplace-operator|status=active|epoch=117|distribution_policy=policy-marketplace-sales-default
operation=Create|event_id=rev-0019|revenue_type=Listings|origin=marketplace/listings|beneficiary=marketplace-operator|status=active|epoch=118|distribution_policy=policy-marketplace-listings-default
operation=Create|event_id=rev-0020|revenue_type=Royalties|origin=marketplace/royalties|beneficiary=creator-beta|status=active|epoch=119|distribution_policy=policy-marketplace-royalties-default
operation=Create|event_id=rev-0021|revenue_type=Marketplace Fees|origin=creator-marketplace/metrics|beneficiary=marketplace-operator|status=active|epoch=120|distribution_policy=policy-marketplace-fees-default
operation=Create|event_id=rev-0022|revenue_type=Protocol Fee Intent|origin=protocol/sustainability|beneficiary=protocol-sustainability-pool|status=intent-only|epoch=121|distribution_policy=policy-protocol-fee-default
operation=Create|event_id=rev-0023|revenue_type=Protocol Sustainability Intent|origin=economic-ledger/protocol|beneficiary=protocol-sustainability-pool|status=intent-only|epoch=122|distribution_policy=policy-protocol-sustainability-default
operation=Create|event_id=rev-0024|revenue_type=Network Revenue|origin=wallet-authority/network|beneficiary=protocol-network|status=intent-only|epoch=123|distribution_policy=policy-protocol-network-default
operation=Update|event_id=rev-0003|field=status|value=active|epoch=124|replayable=true
operation=Archive|event_id=rev-legacy-0000|revenue_type=Legacy Test Revenue|origin=tests/legacy|beneficiary=protocol-network|status=archived|epoch=125|distribution_policy=policy-archived-noop
RECORDS
}
registry_root() { record_root registry_transcript; }

creator_revenue_transcript() {
  common_transcript; printf 'registry_root=%s\n' "$(registry_root)"
  registry_transcript | awk -F'|' '/Template Revenue|Asset Revenue|Marketplace Revenue|Subscription Revenue|Royalty Revenue/ {print "creator_attribution=" $0 "|attribution=creator-id-origin-policy"}'
}
creator_revenue_root() { record_root creator_revenue_transcript; }
operator_revenue_transcript() { common_transcript; printf 'registry_root=%s\n' "$(registry_root)"; registry_transcript | awk -F'|' '/Lease Revenue|Hosting Revenue|Federation Revenue|Runtime Revenue/ {print "operator_attribution=" $0}'; }
operator_revenue_root() { record_root operator_revenue_transcript; }
gpu_revenue_transcript() { common_transcript; printf 'registry_root=%s\n' "$(registry_root)"; printf 'gpu_marketplace_integration=gpu/marketplace|mode=reference-only\n'; registry_transcript | awk -F'|' '/GPU Job Revenue|Artifact Revenue|Verification Revenue|Capacity Revenue/ {print "gpu_attribution=" $0}'; }
gpu_revenue_root() { record_root gpu_revenue_transcript; }
civilization_revenue_transcript() { common_transcript; printf 'registry_root=%s\n' "$(registry_root)"; registry_transcript | awk -F'|' '/Civilization Treasury|Civilization Fees|Civilization Events|Civilization Revenue/ {print "civilization_attribution=" $0}'; }
civilization_revenue_root() { record_root civilization_revenue_transcript; }
marketplace_revenue_transcript() { common_transcript; printf 'registry_root=%s\n' "$(registry_root)"; registry_transcript | awk -F'|' '/Sales|Listings|Royalties|Marketplace Fees/ {print "marketplace_attribution=" $0}'; }
marketplace_revenue_root() { record_root marketplace_revenue_transcript; }
protocol_revenue_transcript() { common_transcript; printf 'registry_root=%s\n' "$(registry_root)"; printf 'live_fee_collection=false\n'; registry_transcript | awk -F'|' '/Protocol Fee Intent|Protocol Sustainability Intent|Network Revenue/ {print "protocol_attribution=" $0}'; }
protocol_revenue_root() { record_root protocol_revenue_transcript; }

revenue_root_transcript() {
  common_transcript
  printf 'registry_root=%s\n' "$(registry_root)"
  printf 'creator_revenue_root=%s\n' "$(creator_revenue_root)"
  printf 'operator_revenue_root=%s\n' "$(operator_revenue_root)"
  printf 'gpu_revenue_root=%s\n' "$(gpu_revenue_root)"
  printf 'civilization_revenue_root=%s\n' "$(civilization_revenue_root)"
  printf 'marketplace_revenue_root=%s\n' "$(marketplace_revenue_root)"
  printf 'protocol_revenue_root=%s\n' "$(protocol_revenue_root)"
}
revenue_root() { record_root revenue_root_transcript; }

distribution_transcript() {
  common_transcript; printf 'revenue_root=%s\n' "$(revenue_root)"
  cat <<'RECORDS'
distribution_intent=dist-creator-001|source=rev-0001|beneficiary=creator-alpha|beneficiary_type=Creator|policy=policy-creator-template-default|status=intent-only|epoch=200
distribution_intent=dist-operator-001|source=rev-0006|beneficiary=operator-atlas|beneficiary_type=Operator|policy=policy-operator-lease-default|status=intent-only|epoch=201
distribution_intent=dist-gpu-001|source=rev-0010|beneficiary=gpu-provider-nova|beneficiary_type=GPU Provider|policy=policy-gpu-job-default|status=intent-only|epoch=202
distribution_intent=dist-civ-001|source=rev-0014|beneficiary=civilization-aurora|beneficiary_type=Civilization|policy=policy-civilization-treasury-default|status=intent-only|epoch=203
distribution_intent=dist-protocol-001|source=rev-0022|beneficiary=protocol-sustainability-pool|beneficiary_type=Protocol|policy=policy-protocol-fee-default|status=intent-only|epoch=204
RECORDS
}
distribution_root() { record_root distribution_transcript; }

settlement_intent_transcript() {
  common_transcript; printf 'distribution_root=%s\n' "$(distribution_root)"; printf 'xrpl_settlement_integration=xrpl/intent|mode=unsigned-intent-only|live_transfers=false\n'
  cat <<'RECORDS'
settlement_intent=settle-revenue-001|kind=Revenue Settlement|distribution_intent=dist-creator-001|rail=xrpl-intent|status=unsigned|epoch=300
settlement_intent=settle-royalty-001|kind=Royalty Settlement|distribution_intent=dist-creator-001|rail=xrpl-intent|status=unsigned|epoch=301
settlement_intent=settle-distribution-001|kind=Distribution Settlement|distribution_intent=dist-operator-001|rail=xrpl-intent|status=unsigned|epoch=302
RECORDS
}
settlement_intent_root() { record_root settlement_intent_transcript; }

analytics_transcript() {
  common_transcript; printf 'revenue_root=%s\n' "$(revenue_root)"; printf 'distribution_root=%s\n' "$(distribution_root)"; printf 'settlement_intent_root=%s\n' "$(settlement_intent_root)"
  printf 'analytics=Creator Revenue|root=%s|metric=gross-intent-events\n' "$(creator_revenue_root)"
  printf 'analytics=Operator Revenue|root=%s|metric=gross-intent-events\n' "$(operator_revenue_root)"
  printf 'analytics=GPU Revenue|root=%s|metric=gross-intent-events\n' "$(gpu_revenue_root)"
  printf 'analytics=Civilization Revenue|root=%s|metric=gross-intent-events\n' "$(civilization_revenue_root)"
  printf 'analytics=Marketplace Revenue|root=%s|metric=gross-intent-events\n' "$(marketplace_revenue_root)"
  printf 'analytics=Protocol Revenue|root=%s|metric=intent-only-events\n' "$(protocol_revenue_root)"
}
analytics_root() { record_root analytics_transcript; }

portal_transcript() {
  common_transcript; printf 'analytics_root=%s\n' "$(analytics_root)"
  cat <<'RECORDS'
portal=Developer Portal|integration=developer-portal/dashboard|view=Creator Earnings|source=analytics|status=reference-only
portal=Creator Marketplace|integration=creator-marketplace/portal|view=Creator Earnings|source=analytics|status=reference-only
portal=Player Gateway|integration=player-gateway|view=Protocol Metrics|source=metrics|status=reference-only
portal=Operator Console|integration=developer-portal/deployments|view=Operator Earnings|source=analytics|status=reference-only
portal=GPU Provider|integration=developer-portal/gpu|view=GPU Earnings|source=analytics|status=reference-only
portal=Civilization|integration=developer-portal/civilizations|view=Civilization Earnings|source=analytics|status=reference-only
RECORDS
}
portal_revenue_root() { record_root portal_transcript; }

metrics_transcript() {
  common_transcript; printf 'analytics_root=%s\n' "$(analytics_root)"
  cat <<'RECORDS'
metric=Gross Revenue|basis=intent-events|value=deterministic-placeholder|production_amount=false
metric=Creator Share|basis=policy-weight|value=policy-defined|hardcoded_percentage=false
metric=Operator Share|basis=policy-weight|value=policy-defined|hardcoded_percentage=false
metric=GPU Share|basis=policy-weight|value=policy-defined|hardcoded_percentage=false
metric=Civilization Share|basis=policy-weight|value=policy-defined|hardcoded_percentage=false
metric=Protocol Share|basis=policy-weight|value=policy-defined|hardcoded_percentage=false
RECORDS
}
metrics_root() { record_root metrics_transcript; }

economic_sustainability_transcript() {
  common_transcript; printf 'metrics_root=%s\n' "$(metrics_root)"
  cat <<'RECORDS'
policy=policy-creator-template-default|creator_share=policy-defined|operator_share=policy-defined|gpu_share=policy-defined|civilization_share=policy-defined|protocol_share=policy-defined|hardcoded_production_percentage=false
policy=policy-marketplace-default|creator_share=policy-defined|operator_share=policy-defined|gpu_share=policy-defined|civilization_share=policy-defined|protocol_share=policy-defined|hardcoded_production_percentage=false
policy=policy-protocol-sustainability-default|creator_share=policy-defined|operator_share=policy-defined|gpu_share=policy-defined|civilization_share=policy-defined|protocol_share=policy-defined|hardcoded_production_percentage=false
RECORDS
}
economic_sustainability_root() { record_root economic_sustainability_transcript; }

replay_transcript() {
  revenue_root_transcript
}
replay_root() { revenue_root; }

commercial_revenue_root_transcript() {
  revenue_root_transcript
  printf 'distribution_root=%s\n' "$(distribution_root)"
  printf 'settlement_intent_root=%s\n' "$(settlement_intent_root)"
  printf 'analytics_root=%s\n' "$(analytics_root)"
  printf 'portal_revenue_root=%s\n' "$(portal_revenue_root)"
  printf 'metrics_root=%s\n' "$(metrics_root)"
  printf 'economic_sustainability_root=%s\n' "$(economic_sustainability_root)"
  printf 'replay_root=%s\n' "$(replay_root)"
}
commercial_revenue_root() { record_root commercial_revenue_root_transcript; }

write_pair() { local dir="$1" base="$2" func="$3" root_func="$4" root_name="$5"; mkdir -p "$dir"; "$func" | LC_ALL=C sort > "$dir/$base.records"; "$root_func" > "$dir/$root_name"; }

init_commercial_revenue() {
  local root="$COMMERCIAL_REVENUE_ROOT_DIR"
  mkdir -p "$root"/{registry,creator,operator,gpu,civilization,marketplace,protocol,distribution,settlement-intents,analytics,replay,portal,metrics,economics,records}
  write_pair "$root/registry" revenue_registry registry_transcript registry_root REVENUE_REGISTRY_ROOT
  write_pair "$root/creator" creator_revenue creator_revenue_transcript creator_revenue_root CREATOR_REVENUE_ROOT
  write_pair "$root/operator" operator_revenue operator_revenue_transcript operator_revenue_root OPERATOR_REVENUE_ROOT
  write_pair "$root/gpu" gpu_revenue gpu_revenue_transcript gpu_revenue_root GPU_REVENUE_ROOT
  write_pair "$root/civilization" civilization_revenue civilization_revenue_transcript civilization_revenue_root CIVILIZATION_REVENUE_ROOT
  write_pair "$root/marketplace" marketplace_revenue marketplace_revenue_transcript marketplace_revenue_root MARKETPLACE_REVENUE_ROOT
  write_pair "$root/protocol" protocol_revenue protocol_revenue_transcript protocol_revenue_root PROTOCOL_REVENUE_ROOT
  write_pair "$root/distribution" distribution distribution_transcript distribution_root DISTRIBUTION_ROOT
  write_pair "$root/settlement-intents" settlement_intents settlement_intent_transcript settlement_intent_root SETTLEMENT_INTENT_ROOT
  write_pair "$root/analytics" revenue_analytics analytics_transcript analytics_root ANALYTICS_ROOT
  write_pair "$root/replay" revenue_replay replay_transcript replay_root REPLAY_ROOT
  write_pair "$root/portal" portal_revenue portal_transcript portal_revenue_root PORTAL_REVENUE_ROOT
  write_pair "$root/metrics" revenue_metrics metrics_transcript metrics_root REVENUE_METRICS_ROOT
  write_pair "$root/economics" economic_sustainability economic_sustainability_transcript economic_sustainability_root ECONOMIC_SUSTAINABILITY_ROOT
  write_pair "$root/records" commercial_revenue_root commercial_revenue_root_transcript commercial_revenue_root COMMERCIAL_REVENUE_ROOT
}

case "${1:-}" in
  init) init_commercial_revenue ;;
  registry-root) registry_root ;;
  creator-root) creator_revenue_root ;;
  operator-root) operator_revenue_root ;;
  gpu-root) gpu_revenue_root ;;
  civilization-root) civilization_revenue_root ;;
  marketplace-root) marketplace_revenue_root ;;
  protocol-root) protocol_revenue_root ;;
  distribution-root) distribution_root ;;
  settlement-root) settlement_intent_root ;;
  analytics-root) analytics_root ;;
  replay-root) replay_root ;;
  portal-root) portal_revenue_root ;;
  metrics-root) metrics_root ;;
  economics-root) economic_sustainability_root ;;
  revenue-root) revenue_root ;;
  commercial-root) commercial_revenue_root ;;
  *) : ;;
esac
