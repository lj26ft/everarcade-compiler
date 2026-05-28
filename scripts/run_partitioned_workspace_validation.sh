#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"
mkdir -p runtime/logs deployment/reports
log="runtime/logs/partitioned_workspace_validation.log"
: > "$log"
cleanup=false
cargo_args=()
for arg in "$@"; do
  case "$arg" in
    --cleanup-before|--cleanup-before-validation) cleanup=true ;;
    --offline|--locked|--frozen) cargo_args+=("$arg") ;;
    *) cargo_args+=("$arg") ;;
  esac
done
if [[ "$cleanup" == true ]]; then
  bash scripts/run_validation_cleanup.sh | tee -a "$log"
fi
bash scripts/run_validation_disk_preflight.sh | tee -a "$log"
partitions=(
  "execution-core runtime_activation_tests"
  "execution-core runtime_live_networking_tests"
  "execution-core runtime_operational_networking_tests"
  "execution-core runtime_node_tests"
)
for partition in "${partitions[@]}"; do
  read -r package test_name <<<"$partition"
  echo "partition_start package=$package test=$test_name jobs=$CARGO_BUILD_JOBS args=${cargo_args[*]}" | tee -a "$log"
  CARGO_BUILD_JOBS="$CARGO_BUILD_JOBS" cargo test --package "$package" --test "$test_name" "${cargo_args[@]}" 2>&1 | tee -a "$log"
  echo "partition_ok package=$package test=$test_name" | tee -a "$log"
done
cat > deployment/reports/runtime_validation_scalability_report.md <<RPT
# Runtime Validation Scalability Report
- partitioned_validation: enabled
- cargo_build_jobs_default: ${CARGO_BUILD_JOBS}
- offline_locked_supported: yes
- disk_preflight: scripts/run_validation_disk_preflight.sh
- cleanup_before_validation: --cleanup-before-validation
- log: runtime/logs/partitioned_workspace_validation.log
- monolithic_workspace_test: intentionally avoided for this milestone
RPT
echo "partitioned_workspace_validation=ok" | tee -a "$log"
