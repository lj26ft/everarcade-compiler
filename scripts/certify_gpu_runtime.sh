#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/gpu_runtime_certification_report.txt"

cd "$ROOT_DIR"
export GPU_MODEL_ROOT="$ROOT_DIR"
# shellcheck source=../gpu/jobs/gpu_model.sh
source "$ROOT_DIR/gpu/jobs/gpu_model.sh"

export GPU_PROJECTION_ROOT="$(projection_root)"
export GPU_JOB_ROOT="$(gpu_job_root)"
export GPU_DEVICE_ROOT="$(device_root)"
export GPU_WORKER_ROOT="$(worker_root)"
export GPU_QUEUE_ROOT="$(queue_root)"
export GPU_ARTIFACT_ROOT="$(artifact_root)"
export GPU_RENDER_ROOT="$(render_root)"
export GPU_VERIFICATION_ROOT="$(verification_root)"
export GPU_REPLAY_RENDER_ROOT="$(replay_render_root)"
export GPU_INTEGRATION_ROOT="$(integration_root)"
mkdir -p "$REPORT_DIR"

gpu_write_artifacts

jobs_status="FAIL"
workers_status="FAIL"
devices_status="FAIL"
queues_status="FAIL"
artifacts_status="FAIL"
verification_status="FAIL"
replay_status="FAIL"
integration_status="FAIL"
overall_status="FAIL"

validate_jobs && jobs_status="PASS"
validate_workers && workers_status="PASS"
validate_devices && devices_status="PASS"
validate_queues && queues_status="PASS"
validate_artifacts && artifacts_status="PASS"
validate_verification && verification_status="PASS"
validate_gpu_replay && replay_status="PASS"
validate_renderer_integration && integration_status="PASS"

if [[ "$jobs_status" == "PASS" \
  && "$workers_status" == "PASS" \
  && "$devices_status" == "PASS" \
  && "$queues_status" == "PASS" \
  && "$artifacts_status" == "PASS" \
  && "$verification_status" == "PASS" \
  && "$replay_status" == "PASS" \
  && "$integration_status" == "PASS" ]]; then
  overall_status="PASS"
fi

cat > "$REPORT_PATH" <<REPORT
GPU Runtime Certification Report
Version: $GPU_RUNTIME_VERSION
Runtime ID: $GPU_RUNTIME_ID
Authority: $GPU_RUNTIME_AUTHORITY
Canonical Ordering: $GPU_CANONICAL_ORDERING
Replay Safe: $GPU_REPLAY_SAFE
Live Hardware Inspection: $GPU_LIVE_HARDWARE_INSPECTION

Jobs: $jobs_status
Workers: $workers_status
Devices: $devices_status
Queues: $queues_status
Artifacts: $artifacts_status
Verification: $verification_status
Replay: $replay_status
Renderer Integration: $integration_status

Projection Root: $(gpu_projection_root)
GPU Job Root: $(gpu_job_root)
Worker Root: $(worker_root)
Device Root: $(device_root)
Queue Root: $(queue_root)
Artifact Root: $(artifact_root)
Render Root: $(render_root)
Verification Root: $(verification_root)
Replay Render Root: $(replay_render_root)
Integration Root: $(integration_root)
Replay Equals Render: $(if [[ "$(replay_render_root)" == "$(render_root)" ]]; then printf 'true'; else printf 'false'; fi)

GPU Runtime: $overall_status
REPORT

cat "$REPORT_PATH"
[[ "$overall_status" == "PASS" ]]
