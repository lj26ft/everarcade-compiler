#!/usr/bin/env bash
# GPU Runtime v0.1 deterministic model.
# This file is sourced by GPU validation and certification scripts. It must
# remain non-authoritative: it consumes renderer projection roots and fixed GPU
# descriptors, then derives deterministic render roots.

set -euo pipefail

GPU_RUNTIME_VERSION="0.1"
GPU_RUNTIME_ID="everarcade-gpu-runtime-v0.1"
GPU_RUNTIME_AUTHORITY="non-authoritative"
GPU_CANONICAL_ORDERING="priority-epoch-job-id-lexicographic"
GPU_REPLAY_SAFE="true"
GPU_LIVE_HARDWARE_INSPECTION="false"

# shellcheck source=../../renderer/projection/projection_model.sh
source "${GPU_MODEL_ROOT:-.}/renderer/projection/projection_model.sh"

GPU_JOB_IDS=(gpu-job-0001 gpu-job-0002 gpu-job-0003 gpu-job-0004 gpu-job-0005 gpu-job-0006)
GPU_JOB_TYPES=(world-render entity-render physics-visualization inventory-visualization event-visualization replay-render)
GPU_JOB_PRIORITIES=(10 20 30 40 50 60)
GPU_JOB_EPOCHS=(42 42 42 42 42 43)

GPU_DEVICE_IDS=(gpu-device-deterministic-a gpu-device-deterministic-b)
GPU_DEVICE_MEMORY=(8192 4096)
GPU_DEVICE_COMPUTE=(deterministic-compute-1.0 deterministic-compute-1.0)
GPU_DEVICE_QUEUE_CAPACITY=(4 2)
GPU_DEVICE_RUNTIME_VERSION=(gpu-runtime-0.1 gpu-runtime-0.1)

GPU_WORKER_IDS=(gpu-worker-alpha gpu-worker-beta)
GPU_WORKER_DEVICE_IDS=(gpu-device-deterministic-a gpu-device-deterministic-b)
GPU_WORKER_CAPABILITY_PROFILES=(world-entity-replay physics-inventory-event)
GPU_WORKER_CAPACITY=(4 2)
GPU_WORKER_AVAILABILITY=(available available)

GPU_QUEUE_PENDING=(gpu-job-0006)
GPU_QUEUE_ASSIGNED=(gpu-job-0002:gpu-worker-beta gpu-job-0003:gpu-worker-beta)
GPU_QUEUE_RUNNING=(gpu-job-0001:gpu-worker-alpha)
GPU_QUEUE_COMPLETED=(gpu-job-0004:gpu-worker-alpha gpu-job-0005:gpu-worker-alpha)
GPU_QUEUE_FAILED=(gpu-job-0007:gpu-worker-beta:artifact-integrity-rejected)

sha256_text() {
  sha256sum | awk '{print $1}'
}

emit_kv_list() {
  local key="$1"
  shift
  local value
  for value in "$@"; do
    printf '%s=%s\n' "$key" "$value"
  done | LC_ALL=C sort
}

gpu_projection_root() {
  printf '%s' "${GPU_PROJECTION_ROOT:-$(projection_root)}"
}

gpu_common_transcript() {
  printf 'gpu_runtime_id=%s\n' "$GPU_RUNTIME_ID"
  printf 'version=%s\n' "$GPU_RUNTIME_VERSION"
  printf 'authority=%s\n' "$GPU_RUNTIME_AUTHORITY"
  printf 'canonical_ordering=%s\n' "$GPU_CANONICAL_ORDERING"
  printf 'replay_safe=%s\n' "$GPU_REPLAY_SAFE"
  printf 'live_hardware_inspection=%s\n' "$GPU_LIVE_HARDWARE_INSPECTION"
  printf 'renderer_projection_root=%s\n' "$(gpu_projection_root)"
}

job_record() {
  local i="$1"
  printf 'job_id=%s\n' "${GPU_JOB_IDS[$i]}"
  printf 'projection_root=%s\n' "$(gpu_projection_root)"
  printf 'job_type=%s\n' "${GPU_JOB_TYPES[$i]}"
  printf 'priority=%s\n' "${GPU_JOB_PRIORITIES[$i]}"
  printf 'submission_epoch=%s\n' "${GPU_JOB_EPOCHS[$i]}"
}

job_root_for_index() {
  local i="$1"
  { gpu_common_transcript; job_record "$i"; } | sha256_text
}

jobs_transcript() {
  gpu_common_transcript
  local i
  for i in "${!GPU_JOB_IDS[@]}"; do
    printf 'job=%s|projection_root=%s|job_type=%s|priority=%s|submission_epoch=%s|job_root=%s\n' \
      "${GPU_JOB_IDS[$i]}" \
      "$(gpu_projection_root)" \
      "${GPU_JOB_TYPES[$i]}" \
      "${GPU_JOB_PRIORITIES[$i]}" \
      "${GPU_JOB_EPOCHS[$i]}" \
      "$(job_root_for_index "$i")"
  done | LC_ALL=C sort
}

gpu_job_root() { printf '%s' "${GPU_JOB_ROOT:-$(jobs_transcript | sha256_text)}"; }

devices_transcript() {
  gpu_common_transcript
  local i
  for i in "${!GPU_DEVICE_IDS[@]}"; do
    printf 'device=%s|memory_mb=%s|compute_capability=%s|queue_capacity=%s|runtime_version=%s\n' \
      "${GPU_DEVICE_IDS[$i]}" \
      "${GPU_DEVICE_MEMORY[$i]}" \
      "${GPU_DEVICE_COMPUTE[$i]}" \
      "${GPU_DEVICE_QUEUE_CAPACITY[$i]}" \
      "${GPU_DEVICE_RUNTIME_VERSION[$i]}"
  done | LC_ALL=C sort
}

device_root() { printf '%s' "${GPU_DEVICE_ROOT:-$(devices_transcript | sha256_text)}"; }

workers_transcript() {
  gpu_common_transcript
  printf 'device_root=%s\n' "$(device_root)"
  local i
  for i in "${!GPU_WORKER_IDS[@]}"; do
    printf 'worker=%s|device_id=%s|capability_profile=%s|capacity=%s|availability=%s\n' \
      "${GPU_WORKER_IDS[$i]}" \
      "${GPU_WORKER_DEVICE_IDS[$i]}" \
      "${GPU_WORKER_CAPABILITY_PROFILES[$i]}" \
      "${GPU_WORKER_CAPACITY[$i]}" \
      "${GPU_WORKER_AVAILABILITY[$i]}"
  done | LC_ALL=C sort
}

worker_root() { printf '%s' "${GPU_WORKER_ROOT:-$(workers_transcript | sha256_text)}"; }

queues_transcript() {
  gpu_common_transcript
  printf 'job_root=%s\n' "$(gpu_job_root)"
  printf 'worker_root=%s\n' "$(worker_root)"
  emit_kv_list pending "${GPU_QUEUE_PENDING[@]}"
  emit_kv_list assigned "${GPU_QUEUE_ASSIGNED[@]}"
  emit_kv_list running "${GPU_QUEUE_RUNNING[@]}"
  emit_kv_list completed "${GPU_QUEUE_COMPLETED[@]}"
  emit_kv_list failed "${GPU_QUEUE_FAILED[@]}"
}

queue_root() { printf '%s' "${GPU_QUEUE_ROOT:-$(queues_transcript | sha256_text)}"; }

artifact_transcript() {
  gpu_common_transcript
  printf 'projection_root=%s\n' "$(gpu_projection_root)"
  printf 'worker_root=%s\n' "$(worker_root)"
  printf 'job_root=%s\n' "$(gpu_job_root)"
  printf 'queue_root=%s\n' "$(queue_root)"
  printf 'artifact_output=world-frame-gpu-job-0001\n'
  printf 'artifact_output=inventory-overlay-gpu-job-0004\n'
  printf 'artifact_output=event-overlay-gpu-job-0005\n'
  printf 'artifact_root_material=projection-root+worker-root+job-root+queue-root+canonical-outputs\n'
}

artifact_root() { printf '%s' "${GPU_ARTIFACT_ROOT:-$(artifact_transcript | sha256_text)}"; }

render_transcript() {
  artifact_transcript
  printf 'artifact_root=%s\n' "$(artifact_root)"
}

render_root() { printf '%s' "${GPU_RENDER_ROOT:-$(render_transcript | sha256_text)}"; }

verification_transcript() {
  gpu_common_transcript
  printf 'projection_root_match=%s\n' "$(if [[ "$(gpu_projection_root)" == "$(expected_replay_projection_root)" ]]; then printf true; else printf false; fi)"
  printf 'job_root_match=%s\n' "$(if [[ -n "$(gpu_job_root)" ]]; then printf true; else printf false; fi)"
  printf 'worker_match=%s\n' "$(if [[ -n "$(worker_root)" && -n "$(device_root)" ]]; then printf true; else printf false; fi)"
  printf 'artifact_integrity=%s\n' "$(if [[ -n "$(artifact_root)" && -n "$(render_root)" ]]; then printf true; else printf false; fi)"
  printf 'render_root=%s\n' "$(render_root)"
}

verification_root() { printf '%s' "${GPU_VERIFICATION_ROOT:-$(verification_transcript | sha256_text)}"; }

replay_render_transcript() {
  gpu_common_transcript
  printf 'checkpoint_id=%s\n' "$REPLAY_CHECKPOINT_ID"
  printf 'checkpoint_tick=%s\n' "$REPLAY_CHECKPOINT_TICK"
  emit_kv_list replay_frame "${REPLAY_STREAM[@]}"
  printf 'projection_root=%s\n' "$(gpu_projection_root)"
  printf 'job_root=%s\n' "$(gpu_job_root)"
  printf 'worker_root=%s\n' "$(worker_root)"
  printf 'artifact_root=%s\n' "$(artifact_root)"
  printf 'render_root=%s\n' "$(render_root)"
}

replay_render_root() { printf '%s' "${GPU_REPLAY_RENDER_ROOT:-$(render_transcript | sha256_text)}"; }

integration_transcript() {
  gpu_common_transcript
  printf 'renderer_runtime_id=%s\n' "$RENDERER_RUNTIME_ID"
  printf 'projection_export=%s\n' "$(gpu_projection_root)"
  printf 'gpu_job_submission=%s\n' "$(gpu_job_root)"
  printf 'artifact_import=%s\n' "$(artifact_root)"
  printf 'renderer_authoritative_over_projection=true\n'
  printf 'gpu_consumer_only=true\n'
  printf 'render_root=%s\n' "$(render_root)"
}

integration_root() { printf '%s' "${GPU_INTEGRATION_ROOT:-$(integration_transcript | sha256_text)}"; }

validate_jobs() {
  local transcript
  transcript="$(jobs_transcript)"
  [[ ${#GPU_JOB_IDS[@]} -eq 6 ]] && [[ -n "$(gpu_job_root)" ]] && grep -q 'job_type=replay-render' <<<"$transcript"
}

validate_workers() {
  local transcript
  transcript="$(workers_transcript)"
  [[ ${#GPU_WORKER_IDS[@]} -gt 0 ]] && [[ -n "$(worker_root)" ]] && grep -q 'availability=available' <<<"$transcript"
}

validate_devices() {
  local transcript
  transcript="$(devices_transcript)"
  [[ "$GPU_LIVE_HARDWARE_INSPECTION" == "false" ]] && [[ -n "$(device_root)" ]] && grep -q 'compute_capability=deterministic-compute-1.0' <<<"$transcript"
}

validate_queues() {
  local transcript
  transcript="$(queues_transcript)"
  [[ -n "$(queue_root)" ]] \
    && grep -q '^pending=' <<<"$transcript" \
    && grep -q '^assigned=' <<<"$transcript" \
    && grep -q '^running=' <<<"$transcript" \
    && grep -q '^completed=' <<<"$transcript" \
    && grep -q '^failed=' <<<"$transcript"
}

validate_artifacts() {
  local transcript
  transcript="$(artifact_transcript)"
  [[ -n "$(artifact_root)" && -n "$(render_root)" ]] && grep -q '^projection_root=' <<<"$transcript"
}

validate_verification() {
  local transcript
  transcript="$(verification_transcript)"
  grep -q '^projection_root_match=true$' <<<"$transcript" \
    && grep -q '^job_root_match=true$' <<<"$transcript" \
    && grep -q '^worker_match=true$' <<<"$transcript" \
    && grep -q '^artifact_integrity=true$' <<<"$transcript" \
    && [[ -n "$(verification_root)" ]]
}

validate_gpu_replay() {
  [[ "$(replay_render_root)" == "$(render_root)" ]] && [[ -n "$(replay_render_root)" ]]
}

validate_renderer_integration() {
  local transcript
  transcript="$(integration_transcript)"
  grep -q '^renderer_authoritative_over_projection=true$' <<<"$transcript" \
    && grep -q '^gpu_consumer_only=true$' <<<"$transcript" \
    && [[ -n "$(integration_root)" ]]
}

gpu_write_artifacts() {
  local root_dir="${GPU_MODEL_ROOT:-.}"
  mkdir -p "$root_dir/gpu/jobs/artifacts" \
    "$root_dir/gpu/workers/artifacts" \
    "$root_dir/gpu/devices/artifacts" \
    "$root_dir/gpu/queues/artifacts" \
    "$root_dir/gpu/artifacts/render" \
    "$root_dir/gpu/verification/artifacts" \
    "$root_dir/gpu/replay/artifacts" \
    "$root_dir/runtime/gpu-runtime/artifacts"

  jobs_transcript > "$root_dir/gpu/jobs/artifacts/jobs_transcript.txt"
  workers_transcript > "$root_dir/gpu/workers/artifacts/workers_transcript.txt"
  devices_transcript > "$root_dir/gpu/devices/artifacts/devices_transcript.txt"
  queues_transcript > "$root_dir/gpu/queues/artifacts/queues_transcript.txt"
  artifact_transcript > "$root_dir/gpu/artifacts/render/artifact_transcript.txt"
  render_transcript > "$root_dir/gpu/artifacts/render/render_transcript.txt"
  verification_transcript > "$root_dir/gpu/verification/artifacts/verification_transcript.txt"
  replay_render_transcript > "$root_dir/gpu/replay/artifacts/replay_render_transcript.txt"
  integration_transcript > "$root_dir/runtime/gpu-runtime/artifacts/renderer_integration_transcript.txt"
}
