#!/usr/bin/env bash
set -euo pipefail
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"
args=()
for arg in "$@"; do
  case "$arg" in
    --offline|--locked|--frozen) args+=("$arg") ;;
    *) echo "unsupported argument: $arg" >&2; exit 2 ;;
  esac
done
mkdir -p deployment/reports
{
  echo "# Cross-Machine Certification Run"
  echo
  echo "Command: CARGO_BUILD_JOBS=$CARGO_BUILD_JOBS cargo test -p execution-core --test cross_machine_certification_tests ${args[*]}"
  echo "Started: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
} > deployment/reports/cross_machine_certification_run.md
CARGO_BUILD_JOBS="$CARGO_BUILD_JOBS" cargo test -p execution-core --test cross_machine_certification_tests "${args[@]}"
{
  echo "Finished: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
  echo "Result: pass"
} >> deployment/reports/cross_machine_certification_run.md
