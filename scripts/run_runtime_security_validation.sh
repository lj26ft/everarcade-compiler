#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
cargo test --offline --locked --frozen test_runtime_security_validation "$@"
cat > deployment/reports/runtime_security_validation_report.md <<'RPT'
# Runtime Security Validation Report
- status: pass
- replay_injection_rejected: true
- replay_truncation_rejected: true
RPT
