#!/usr/bin/env bash
set -euo pipefail
mkdir -p deployment/reports
bash scripts/vendor_deps.sh
cargo metadata --offline --locked >/dev/null
cargo test --package execution-core --test runtime_operations_tests --offline --locked
bash scripts/run_bundle_generation.sh --offline --locked
bash scripts/run_restore_validation.sh --offline --locked
bash scripts/run_release_pipeline.sh --offline --locked
bash scripts/run_stdout_runtime_validation.sh --offline --locked
cat > deployment/reports/runtime_validation_pipeline_report.md <<RPT
# Runtime Validation Pipeline Report
- vendor_preflight: ok
- offline_dependency_validation: ok
- runtime_operations_tests: ok
- bundle_generation_validation: ok
- restoration_validation: ok
- release_verification: ok
- stdout_determinism_validation: ok
RPT
