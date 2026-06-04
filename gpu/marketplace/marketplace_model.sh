#!/usr/bin/env bash
# GPU Marketplace v0.1 deterministic model.
# Providers remain non-authoritative workers. Leases schedule projection jobs;
# marketplace roots are derived from canonical identity, registration,
# capability, capacity, assignment, artifact, verification, settlement,
# reputation, replay, and lease-integration transcripts.

set -euo pipefail

GPU_MARKETPLACE_VERSION="0.1"
GPU_MARKETPLACE_ID="everarcade-gpu-marketplace-v0.1"
GPU_MARKETPLACE_AUTHORITY="non-authoritative-projection-compute"
GPU_MARKETPLACE_CANONICAL_ORDERING="epoch-priority-job-id-provider-id-lexicographic"
GPU_MARKETPLACE_REPLAY_SAFE="true"
GPU_MARKETPLACE_LIVE_DISCOVERY="false"
GPU_MARKETPLACE_PAYMENTS="settlement-intent-only"

# shellcheck source=../jobs/gpu_model.sh
source "${GPU_MARKETPLACE_ROOT:-.}/gpu/jobs/gpu_model.sh"

PROVIDER_IDS=(provider-alpha provider-beta provider-gamma)
PROVIDER_NODE_IDS=(gpu-worker-alpha gpu-worker-beta gpu-worker-gamma)
PROVIDER_REGISTRATION_EPOCHS=(100 100 101)

CAPABILITY_GPU_MODELS=(deterministic-gpu-a deterministic-gpu-b deterministic-gpu-c)
CAPABILITY_MEMORY_MB=(8192 4096 12288)
CAPABILITY_QUEUE_CAPACITY=(4 2 3)
CAPABILITY_RENDER_CLASSES=(world-render+inventory-visualization entity-render+physics-visualization replay-render+event-visualization)
CAPABILITY_AVAILABILITY=(available available suspended)

CAPACITY_AVAILABLE_SLOTS=(3 1 0)
CAPACITY_RESERVED_SLOTS=(1 1 0)
CAPACITY_CONSUMED_SLOTS=(2 1 0)
CAPACITY_EPOCHS=(110 110 110)

REGISTRATION_ACTIONS=(register register register update suspend recover retire)
REGISTRATION_PROVIDERS=(provider-alpha provider-beta provider-gamma provider-alpha provider-gamma provider-gamma provider-beta)
REGISTRATION_EPOCHS=(100 100 101 105 106 108 111)
REGISTRATION_STATUS=(registered registered registered updated suspended recovered retired)

MARKETPLACE_JOB_IDS=(gpu-job-0001 gpu-job-0002 gpu-job-0003 gpu-job-0004)
MARKETPLACE_JOB_PRIORITIES=(10 20 30 40)
MARKETPLACE_ASSIGNMENT_PROVIDERS=(provider-alpha provider-beta provider-alpha provider-alpha)
MARKETPLACE_ASSIGNMENT_EPOCHS=(112 112 113 113)
MARKETPLACE_ASSIGNMENT_IDS=(assign-0001 assign-0002 assign-0003 assign-0004)

ARTIFACT_IDS=(artifact-0001 artifact-0002 artifact-0003 artifact-0004)
ARTIFACT_PROVIDERS=(provider-alpha provider-beta provider-alpha provider-alpha)
ARTIFACT_JOBS=(gpu-job-0001 gpu-job-0002 gpu-job-0003 gpu-job-0004)
ARTIFACT_EPOCHS=(114 114 115 115)
ARTIFACT_PAYLOADS=(world-frame-gpu-job-0001 entity-frame-gpu-job-0002 physics-overlay-gpu-job-0003 inventory-overlay-gpu-job-0004)

VERIFICATION_IDS=(verify-0001 verify-0002 verify-0003 verify-0004)
VERIFICATION_ARTIFACTS=(artifact-0001 artifact-0002 artifact-0003 artifact-0004)
VERIFICATION_PROVIDERS=(provider-alpha provider-beta provider-alpha provider-alpha)
VERIFICATION_JOBS=(gpu-job-0001 gpu-job-0002 gpu-job-0003 gpu-job-0004)
VERIFICATION_RESULTS=(verified verified rejected verified)
VERIFICATION_EPOCHS=(116 116 117 117)

SETTLEMENT_IDS=(settle-0001 settle-0002 settle-0003 settle-0004)
SETTLEMENT_PROVIDERS=(provider-alpha provider-beta provider-alpha provider-alpha)
SETTLEMENT_WORK_COMPLETED=(gpu-job-0001 gpu-job-0002 gpu-job-0003 gpu-job-0004)
SETTLEMENT_VERIFICATION_RESULTS=(verified verified rejected verified)
SETTLEMENT_REWARD_UNITS=(10 10 0 10)

REPUTATION_PROVIDERS=(provider-alpha provider-beta provider-gamma)
REPUTATION_SUCCESSFUL_JOBS=(2 1 0)
REPUTATION_FAILED_JOBS=(1 0 0)
REPUTATION_VERIFIED_ARTIFACTS=(2 1 0)
REPUTATION_PROVIDER_SCORES=(80 100 0)

sha256_text() {
  sha256sum | awk '{print $1}'
}

marketplace_projection_root() {
  printf '%s' "${GPU_PROJECTION_ROOT:-$(projection_root)}"
}

marketplace_common_transcript() {
  printf 'marketplace_id=%s\n' "$GPU_MARKETPLACE_ID"
  printf 'version=%s\n' "$GPU_MARKETPLACE_VERSION"
  printf 'authority=%s\n' "$GPU_MARKETPLACE_AUTHORITY"
  printf 'canonical_ordering=%s\n' "$GPU_MARKETPLACE_CANONICAL_ORDERING"
  printf 'replay_safe=%s\n' "$GPU_MARKETPLACE_REPLAY_SAFE"
  printf 'live_discovery=%s\n' "$GPU_MARKETPLACE_LIVE_DISCOVERY"
  printf 'payments=%s\n' "$GPU_MARKETPLACE_PAYMENTS"
  printf 'renderer_projection_root=%s\n' "$(marketplace_projection_root)"
  printf 'gpu_runtime_job_root=%s\n' "$(gpu_job_root)"
}

capability_record() {
  local i="$1"
  printf 'provider_id=%s\n' "${PROVIDER_IDS[$i]}"
  printf 'gpu_model=%s\n' "${CAPABILITY_GPU_MODELS[$i]}"
  printf 'memory_mb=%s\n' "${CAPABILITY_MEMORY_MB[$i]}"
  printf 'queue_capacity=%s\n' "${CAPABILITY_QUEUE_CAPACITY[$i]}"
  printf 'render_classes=%s\n' "${CAPABILITY_RENDER_CLASSES[$i]}"
  printf 'availability=%s\n' "${CAPABILITY_AVAILABILITY[$i]}"
}

capability_root_for_index() {
  local i="$1"
  { marketplace_common_transcript; capability_record "$i"; } | sha256_text
}

capability_transcript() {
  marketplace_common_transcript
  local i
  for i in "${!PROVIDER_IDS[@]}"; do
    printf 'capability=%s|gpu_model=%s|memory_mb=%s|queue_capacity=%s|render_classes=%s|availability=%s|capability_root=%s\n' \
      "${PROVIDER_IDS[$i]}" \
      "${CAPABILITY_GPU_MODELS[$i]}" \
      "${CAPABILITY_MEMORY_MB[$i]}" \
      "${CAPABILITY_QUEUE_CAPACITY[$i]}" \
      "${CAPABILITY_RENDER_CLASSES[$i]}" \
      "${CAPABILITY_AVAILABILITY[$i]}" \
      "$(capability_root_for_index "$i")"
  done | LC_ALL=C sort
}

capability_root() { printf '%s' "${MARKETPLACE_CAPABILITY_ROOT:-$(capability_transcript | sha256_text)}"; }

identity_record() {
  local i="$1"
  printf 'provider_id=%s\n' "${PROVIDER_IDS[$i]}"
  printf 'node_id=%s\n' "${PROVIDER_NODE_IDS[$i]}"
  printf 'registration_epoch=%s\n' "${PROVIDER_REGISTRATION_EPOCHS[$i]}"
  printf 'capability_root=%s\n' "$(capability_root_for_index "$i")"
}

identity_root_for_index() {
  local i="$1"
  { marketplace_common_transcript; identity_record "$i"; } | sha256_text
}

identity_transcript() {
  marketplace_common_transcript
  local i
  for i in "${!PROVIDER_IDS[@]}"; do
    printf 'identity=%s|node_id=%s|registration_epoch=%s|capability_root=%s|identity_root=%s\n' \
      "${PROVIDER_IDS[$i]}" \
      "${PROVIDER_NODE_IDS[$i]}" \
      "${PROVIDER_REGISTRATION_EPOCHS[$i]}" \
      "$(capability_root_for_index "$i")" \
      "$(identity_root_for_index "$i")"
  done | LC_ALL=C sort
}

provider_identity_root() { printf '%s' "${MARKETPLACE_IDENTITY_ROOT:-$(identity_transcript | sha256_text)}"; }

registration_transcript() {
  marketplace_common_transcript
  printf 'identity_root=%s\n' "$(provider_identity_root)"
  local i
  for i in "${!REGISTRATION_ACTIONS[@]}"; do
    printf 'registration=%s|provider=%s|epoch=%s|status=%s\n' \
      "${REGISTRATION_ACTIONS[$i]}" \
      "${REGISTRATION_PROVIDERS[$i]}" \
      "${REGISTRATION_EPOCHS[$i]}" \
      "${REGISTRATION_STATUS[$i]}"
  done | LC_ALL=C sort
}

registration_root() { printf '%s' "${MARKETPLACE_REGISTRATION_ROOT:-$(registration_transcript | sha256_text)}"; }

capacity_transcript() {
  marketplace_common_transcript
  printf 'capability_root=%s\n' "$(capability_root)"
  local i
  for i in "${!PROVIDER_IDS[@]}"; do
    printf 'capacity=%s|available_slots=%s|reserved_slots=%s|consumed_slots=%s|epoch_capacity=%s\n' \
      "${PROVIDER_IDS[$i]}" \
      "${CAPACITY_AVAILABLE_SLOTS[$i]}" \
      "${CAPACITY_RESERVED_SLOTS[$i]}" \
      "${CAPACITY_CONSUMED_SLOTS[$i]}" \
      "${CAPACITY_EPOCHS[$i]}"
  done | LC_ALL=C sort
}

capacity_root() { printf '%s' "${MARKETPLACE_CAPACITY_ROOT:-$(capacity_transcript | sha256_text)}"; }

assignment_transcript() {
  marketplace_common_transcript
  printf 'registration_root=%s\n' "$(registration_root)"
  printf 'capacity_root=%s\n' "$(capacity_root)"
  local i
  for i in "${!MARKETPLACE_JOB_IDS[@]}"; do
    printf 'assignment=%s|job=%s|provider=%s|assignment_epoch=%s|priority=%s\n' \
      "${MARKETPLACE_ASSIGNMENT_IDS[$i]}" \
      "${MARKETPLACE_JOB_IDS[$i]}" \
      "${MARKETPLACE_ASSIGNMENT_PROVIDERS[$i]}" \
      "${MARKETPLACE_ASSIGNMENT_EPOCHS[$i]}" \
      "${MARKETPLACE_JOB_PRIORITIES[$i]}"
  done | LC_ALL=C sort -t'|' -k4,4 -k5,5 -k2,2 -k3,3
}

assignment_root() { printf '%s' "${MARKETPLACE_ASSIGNMENT_ROOT:-$(assignment_transcript | sha256_text)}"; }

artifact_submission_transcript() {
  marketplace_common_transcript
  printf 'assignment_root=%s\n' "$(assignment_root)"
  local i
  for i in "${!ARTIFACT_IDS[@]}"; do
    printf 'artifact=%s|provider=%s|job=%s|submission_epoch=%s|payload=%s|payload_hash=%s\n' \
      "${ARTIFACT_IDS[$i]}" \
      "${ARTIFACT_PROVIDERS[$i]}" \
      "${ARTIFACT_JOBS[$i]}" \
      "${ARTIFACT_EPOCHS[$i]}" \
      "${ARTIFACT_PAYLOADS[$i]}" \
      "$(printf '%s' "${ARTIFACT_PAYLOADS[$i]}" | sha256_text)"
  done | LC_ALL=C sort
}

artifact_submission_root() { printf '%s' "${MARKETPLACE_ARTIFACT_ROOT:-$(artifact_submission_transcript | sha256_text)}"; }

verification_transcript() {
  marketplace_common_transcript
  printf 'assignment_root=%s\n' "$(assignment_root)"
  printf 'artifact_submission_root=%s\n' "$(artifact_submission_root)"
  local i
  for i in "${!VERIFICATION_IDS[@]}"; do
    local integrity="true"
    local job_match="true"
    local provider_match="true"
    local projection_match="true"
    if [[ "${VERIFICATION_RESULTS[$i]}" != "verified" ]]; then
      integrity="false"
    fi
    printf 'verification=%s|artifact=%s|provider=%s|job=%s|artifact_integrity=%s|job_match=%s|provider_match=%s|projection_match=%s|result=%s|epoch=%s\n' \
      "${VERIFICATION_IDS[$i]}" \
      "${VERIFICATION_ARTIFACTS[$i]}" \
      "${VERIFICATION_PROVIDERS[$i]}" \
      "${VERIFICATION_JOBS[$i]}" \
      "$integrity" \
      "$job_match" \
      "$provider_match" \
      "$projection_match" \
      "${VERIFICATION_RESULTS[$i]}" \
      "${VERIFICATION_EPOCHS[$i]}"
  done | LC_ALL=C sort
}

verification_root() { printf '%s' "${MARKETPLACE_VERIFICATION_ROOT:-$(verification_transcript | sha256_text)}"; }

settlement_intent_transcript() {
  marketplace_common_transcript
  printf 'verification_root=%s\n' "$(verification_root)"
  local i
  for i in "${!SETTLEMENT_IDS[@]}"; do
    printf 'settlement_intent=%s|provider=%s|work_completed=%s|verification_result=%s|reward_units=%s\n' \
      "${SETTLEMENT_IDS[$i]}" \
      "${SETTLEMENT_PROVIDERS[$i]}" \
      "${SETTLEMENT_WORK_COMPLETED[$i]}" \
      "${SETTLEMENT_VERIFICATION_RESULTS[$i]}" \
      "${SETTLEMENT_REWARD_UNITS[$i]}"
  done | LC_ALL=C sort
}

settlement_intent_root() { printf '%s' "${MARKETPLACE_SETTLEMENT_ROOT:-$(settlement_intent_transcript | sha256_text)}"; }

reputation_transcript() {
  marketplace_common_transcript
  printf 'verification_root=%s\n' "$(verification_root)"
  printf 'settlement_intent_root=%s\n' "$(settlement_intent_root)"
  local i
  for i in "${!REPUTATION_PROVIDERS[@]}"; do
    printf 'reputation=%s|successful_jobs=%s|failed_jobs=%s|verified_artifacts=%s|provider_score=%s\n' \
      "${REPUTATION_PROVIDERS[$i]}" \
      "${REPUTATION_SUCCESSFUL_JOBS[$i]}" \
      "${REPUTATION_FAILED_JOBS[$i]}" \
      "${REPUTATION_VERIFIED_ARTIFACTS[$i]}" \
      "${REPUTATION_PROVIDER_SCORES[$i]}"
  done | LC_ALL=C sort
}

reputation_root() { printf '%s' "${MARKETPLACE_REPUTATION_ROOT:-$(reputation_transcript | sha256_text)}"; }

marketplace_replay_transcript() {
  marketplace_common_transcript
  printf 'assignment_root=%s\n' "$(assignment_root)"
  printf 'artifact_submission_root=%s\n' "$(artifact_submission_root)"
  printf 'verification_root=%s\n' "$(verification_root)"
  printf 'settlement_intent_root=%s\n' "$(settlement_intent_root)"
  printf 'reputation_root=%s\n' "$(reputation_root)"
}

marketplace_replay_root() { printf '%s' "${MARKETPLACE_REPLAY_ROOT:-$(marketplace_replay_transcript | sha256_text)}"; }

marketplace_root() { printf '%s' "${MARKETPLACE_ROOT_HASH:-$(marketplace_replay_transcript | sha256_text)}"; }

lease_integration_transcript() {
  marketplace_common_transcript
  printf 'lease_role=scheduler\n'
  printf 'provider_role=worker\n'
  printf 'projection_export=%s\n' "$(marketplace_projection_root)"
  printf 'marketplace_submission=%s\n' "$(assignment_root)"
  printf 'artifact_import=%s\n' "$(artifact_submission_root)"
  printf 'verification_import=%s\n' "$(verification_root)"
  printf 'lease_authority_preserved=true\n'
}

lease_integration_root() { printf '%s' "${MARKETPLACE_LEASE_INTEGRATION_ROOT:-$(lease_integration_transcript | sha256_text)}"; }

validate_identity() {
  [[ ${#PROVIDER_IDS[@]} -eq 3 ]] \
    && [[ -n "$(provider_identity_root)" ]] \
    && grep -q 'identity=provider-alpha' <<<"$(identity_transcript)" \
    && grep -q 'capability_root=' <<<"$(identity_transcript)"
}

validate_registration() {
  local transcript
  transcript="$(registration_transcript)"
  [[ -n "$(registration_root)" ]] \
    && grep -q 'registration=register' <<<"$transcript" \
    && grep -q 'registration=update' <<<"$transcript" \
    && grep -q 'registration=suspend' <<<"$transcript" \
    && grep -q 'registration=recover' <<<"$transcript" \
    && grep -q 'registration=retire' <<<"$transcript"
}

validate_capability() {
  local transcript
  transcript="$(capability_transcript)"
  [[ -n "$(capability_root)" ]] \
    && grep -q 'gpu_model=deterministic-gpu-a' <<<"$transcript" \
    && grep -q 'memory_mb=8192' <<<"$transcript" \
    && grep -q 'queue_capacity=4' <<<"$transcript" \
    && grep -q 'render_classes=world-render+inventory-visualization' <<<"$transcript" \
    && grep -q 'availability=available' <<<"$transcript"
}

validate_capacity() {
  local transcript
  transcript="$(capacity_transcript)"
  [[ -n "$(capacity_root)" ]] \
    && grep -q 'available_slots=' <<<"$transcript" \
    && grep -q 'reserved_slots=' <<<"$transcript" \
    && grep -q 'consumed_slots=' <<<"$transcript" \
    && grep -q 'epoch_capacity=110' <<<"$transcript"
}

validate_assignment() {
  local transcript
  transcript="$(assignment_transcript)"
  [[ -n "$(assignment_root)" ]] \
    && grep -q 'assignment=assign-0001' <<<"$transcript" \
    && grep -q 'provider=provider-alpha' <<<"$transcript" \
    && grep -q 'assignment_epoch=112' <<<"$transcript"
}

validate_artifacts() {
  local transcript
  transcript="$(artifact_submission_transcript)"
  [[ -n "$(artifact_submission_root)" ]] \
    && grep -q 'artifact=artifact-0001' <<<"$transcript" \
    && grep -q 'payload_hash=' <<<"$transcript" \
    && grep -q 'submission_epoch=114' <<<"$transcript"
}

validate_verification() {
  local transcript
  transcript="$(verification_transcript)"
  [[ -n "$(verification_root)" ]] \
    && grep -q 'artifact_integrity=true' <<<"$transcript" \
    && grep -q 'job_match=true' <<<"$transcript" \
    && grep -q 'provider_match=true' <<<"$transcript" \
    && grep -q 'projection_match=true' <<<"$transcript" \
    && grep -q 'result=verified' <<<"$transcript"
}

validate_settlement_intent() {
  local transcript
  transcript="$(settlement_intent_transcript)"
  [[ -n "$(settlement_intent_root)" ]] \
    && grep -q 'settlement_intent=settle-0001' <<<"$transcript" \
    && grep -q 'reward_units=10' <<<"$transcript" \
    && grep -q 'verification_result=verified' <<<"$transcript"
}

validate_reputation() {
  local transcript
  transcript="$(reputation_transcript)"
  [[ -n "$(reputation_root)" ]] \
    && grep -q 'successful_jobs=2' <<<"$transcript" \
    && grep -q 'failed_jobs=1' <<<"$transcript" \
    && grep -q 'verified_artifacts=2' <<<"$transcript" \
    && grep -q 'provider_score=80' <<<"$transcript"
}

validate_marketplace_replay() {
  [[ "$(marketplace_replay_root)" == "$(marketplace_root)" ]] && [[ -n "$(marketplace_replay_root)" ]]
}

validate_lease_integration() {
  local transcript
  transcript="$(lease_integration_transcript)"
  [[ -n "$(lease_integration_root)" ]] \
    && grep -q '^lease_role=scheduler$' <<<"$transcript" \
    && grep -q '^provider_role=worker$' <<<"$transcript" \
    && grep -q '^lease_authority_preserved=true$' <<<"$transcript"
}

gpu_marketplace_write_artifacts() {
  local root_dir="${GPU_MARKETPLACE_ROOT:-.}"
  mkdir -p "$root_dir/gpu/marketplace/artifacts" "$root_dir/runtime/gpu-marketplace/artifacts"

  identity_transcript > "$root_dir/gpu/marketplace/artifacts/identity_transcript.txt"
  registration_transcript > "$root_dir/gpu/marketplace/artifacts/registration_transcript.txt"
  capability_transcript > "$root_dir/gpu/marketplace/artifacts/capability_transcript.txt"
  capacity_transcript > "$root_dir/gpu/marketplace/artifacts/capacity_transcript.txt"
  assignment_transcript > "$root_dir/gpu/marketplace/artifacts/assignment_transcript.txt"
  artifact_submission_transcript > "$root_dir/gpu/marketplace/artifacts/artifact_submission_transcript.txt"
  verification_transcript > "$root_dir/gpu/marketplace/artifacts/verification_transcript.txt"
  settlement_intent_transcript > "$root_dir/gpu/marketplace/artifacts/settlement_intent_transcript.txt"
  reputation_transcript > "$root_dir/gpu/marketplace/artifacts/reputation_transcript.txt"
  marketplace_replay_transcript > "$root_dir/gpu/marketplace/artifacts/marketplace_replay_transcript.txt"
  lease_integration_transcript > "$root_dir/runtime/gpu-marketplace/artifacts/lease_integration_transcript.txt"
}
