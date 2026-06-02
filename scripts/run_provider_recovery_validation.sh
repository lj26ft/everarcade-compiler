#!/usr/bin/env bash
set -euo pipefail
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"
CARGO_FLAGS=()
for arg in "$@"; do
  case "$arg" in
    --offline|--locked|--frozen) CARGO_FLAGS+=("$arg") ;;
    *) echo "unsupported argument: $arg" >&2; exit 2 ;;
  esac
done
mkdir -p deployment/reports
REPORT="deployment/reports/provider_recovery_readiness.md"
{
  echo "# Provider Recovery Validation"
  echo
  echo "Classification: Partially Ready"
  echo
  echo "Validation command: CARGO_BUILD_JOBS=$CARGO_BUILD_JOBS cargo test -p execution-core --test evernode_provider_tests ${CARGO_FLAGS[*]}"
  echo
} > "$REPORT"
if CARGO_BUILD_JOBS="$CARGO_BUILD_JOBS" cargo test -p execution-core --test evernode_provider_tests "${CARGO_FLAGS[@]}"; then
  echo "Result: PASS" >> "$REPORT"
else
  status=$?
  echo "Result: FAIL ($status)" >> "$REPORT"
  exit "$status"
fi
