#!/usr/bin/env bash
set -euo pipefail
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"
CARGO_FLAGS=()
for arg in "$@"; do case "$arg" in --offline|--locked|--frozen) CARGO_FLAGS+=("$arg");; *) echo "unsupported argument: $arg" >&2; exit 2;; esac; done
REPORT="deployment/reports/convergence_report.md"
echo "[everarcade] convergence validation"
CARGO_BUILD_JOBS="$CARGO_BUILD_JOBS" cargo test -p execution-core --test two_node_certification_tests test_world_convergence "${CARGO_FLAGS[@]}"
CARGO_BUILD_JOBS="$CARGO_BUILD_JOBS" cargo test -p execution-core --test two_node_certification_tests test_replay_convergence "${CARGO_FLAGS[@]}"
CARGO_BUILD_JOBS="$CARGO_BUILD_JOBS" cargo test -p execution-core --test two_node_certification_tests test_checkpoint_convergence "${CARGO_FLAGS[@]}"
cat > "$REPORT" <<'REPORT'
# Convergence Report

## evidence
- Deterministic two-node convergence tests were executed by this script.
- Metrics recorded by the harness include sync ticks, checkpoint transfers, replay transfers, recovery ticks, and convergence ticks.

## test coverage
- world root convergence
- replay root/hash convergence
- checkpoint root convergence
- continuity root convergence

## known limitations
- Metrics are informational and measured in deterministic harness ticks, not wall-clock distributed network time.

## remaining scaffolds
- Cross-host transport timings.
- Operator-level convergence dashboards.

## next risks
- Long-lived storage and real transport jitter can still expose non-harness convergence risks.
REPORT
