#!/usr/bin/env bash
set -euo pipefail
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"
CARGO_FLAGS=()
for arg in "$@"; do case "$arg" in --offline|--locked|--frozen) CARGO_FLAGS+=("$arg");; *) echo "unsupported argument: $arg" >&2; exit 2;; esac; done
REPORT="deployment/reports/failure_recovery_report.md"
echo "[everarcade] failure/recovery validation"
CARGO_BUILD_JOBS="$CARGO_BUILD_JOBS" cargo test -p execution-core --test two_node_certification_tests test_node_failure_survival "${CARGO_FLAGS[@]}"
CARGO_BUILD_JOBS="$CARGO_BUILD_JOBS" cargo test -p execution-core --test two_node_certification_tests test_node_recovery_convergence "${CARGO_FLAGS[@]}"
cat > "$REPORT" <<'REPORT'
# Failure Recovery Report

## evidence
- Node failure survival and recovery convergence tests executed through the two-node harness.
- Surviving node continues deterministic world and replay progression; failed node restores from survivor checkpoint.

## test coverage
- node termination
- survivor continuation
- checkpoint restoration
- replay/state convergence after restore

## known limitations
- Process failure is modeled in memory; no daemon supervisor, disk crash, or multi-host storage fault is claimed here.

## remaining scaffolds
- Operator failover automation.
- Persistent crash-recovery replay store validation.

## next risks
- Real deployment restart behavior and stale checkpoint eviction policies need separate validation.
REPORT
