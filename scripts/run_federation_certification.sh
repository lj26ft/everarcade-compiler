#!/usr/bin/env bash
set -euo pipefail
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"
CARGO_FLAGS=()
for arg in "$@"; do case "$arg" in --offline|--locked|--frozen) CARGO_FLAGS+=("$arg");; *) echo "unsupported argument: $arg" >&2; exit 2;; esac; done
REPORT="deployment/reports/federation_certification_report.md"
echo "[everarcade] federation certification audit"
CARGO_BUILD_JOBS="$CARGO_BUILD_JOBS" cargo test -p execution-core --test two_node_certification_tests test_node_join_certification "${CARGO_FLAGS[@]}"
CARGO_BUILD_JOBS="$CARGO_BUILD_JOBS" cargo test -p execution-core --test runtime_federation_tests "${CARGO_FLAGS[@]}"
cat > "$REPORT" <<'REPORT'
# Federation Certification Report

## evidence
- Two-node join certification and runtime federation tests executed by this script.

## test coverage
- implemented: in-process checkpoint join, reconstruction, replay/root comparison, peer health equivalence, checkpoint distribution, corruption detection.
- scaffold: distributed transport routing, replay stream propagation, federation topology recovery.
- placeholder: public federation, EverNode operator registration, live WAN fault injection.

## known limitations
- Certification is deterministic and local; no public federation or production deployment is claimed.

## remaining scaffolds
- External peer discovery.
- Multi-operator trust policy.
- Live federation transport and storage.

## next risks
- Federation status must remain honestly classified until cross-host evidence exists.
REPORT
