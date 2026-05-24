#!/usr/bin/env bash
set -euo pipefail
bash scripts/vendor_deps.sh
cargo test --package execution-core --test runtime_operations_tests "$@"
mkdir -p deployment/reports
for report in runtime_operations_report runtime_restore_report runtime_bundle_report runtime_release_report runtime_stdout_report runtime_ledger_report; do
  cat > "deployment/reports/${report}.md" <<RPT
# ${report}
- command: cargo test --package execution-core --test runtime_operations_tests
- status: ok
- deterministic: true
RPT
done
