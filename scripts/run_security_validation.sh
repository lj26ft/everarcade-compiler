#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
"$ROOT/scripts/preflight_vendor.sh"

echo "capability validation"
cargo test --package execution-core --test security_runtime_tests "$@"
echo "isolation validation"
cargo test --package execution-core --test adversarial_runtime_tests "$@"
echo "governance validation"
cargo test --package execution-core --test runtime_abuse_simulation_tests "$@"
echo "quarantine validation"
cargo test --package execution-core --test runtime_security_tests "$@"
echo "validation-root equivalence"
cargo test --package execution-core --test security_runtime_tests "$@"

echo "governance enforcement validation"
cargo test --package execution-core --test governance_enforcement_tests "$@"
echo "capability enforcement validation"
cargo test --package execution-core --test governance_enforcement_tests "$@" test_capability_violation_enforcement
echo "isolation enforcement validation"
cargo test --package execution-core --test governance_enforcement_tests "$@" test_isolation_violation_enforcement
echo "overflow validation"
cargo test --package execution-core --test governance_enforcement_tests "$@" test_event_budget_enforcement
echo "abuse-scale validation"
cargo test --package execution-core --test governance_abuse_scale_tests "$@"
