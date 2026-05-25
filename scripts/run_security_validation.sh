#!/usr/bin/env bash
set -euo pipefail

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
