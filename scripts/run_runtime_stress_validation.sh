#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
cargo test --offline --locked --frozen test_runtime_stress_validation "$@"
cat > deployment/reports/runtime_stress_validation_report.md <<'RPT'
# Runtime Stress Validation Report
- status: pass
- deterministic_ordering: true
- replay_equivalence: true
RPT
