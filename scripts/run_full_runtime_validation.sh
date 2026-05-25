#!/usr/bin/env bash
set -euo pipefail

ARGS=("$@")

bash scripts/vendor_deps.sh
cargo test --package execution-core --test wasm_runtime_tests "${ARGS[@]}"
cargo test --package execution-core --test replay_validation_tests "${ARGS[@]}"
cargo test --package execution-core --test wasm_stateful_execution_tests "${ARGS[@]}"
cargo test --package execution-core --test guest_runtime_tests "${ARGS[@]}"
cargo test --package execution-core --test checkpoint_exchange_tests "${ARGS[@]}"
cargo test --package execution-core --test tenant_isolation_tests "${ARGS[@]}"
cargo test --package execution-core --test world_epoch_tests "${ARGS[@]}"
cargo test --package execution-core --test recovery_operations_tests "${ARGS[@]}"
cargo test --package execution-core --test epoch_transition_tests "${ARGS[@]}"
cargo test --package execution-core --test world_partition_tests "${ARGS[@]}"
cargo test --package execution-core --test incremental_world_runtime_tests "${ARGS[@]}"
cargo test --package execution-core --test scheduler_tests "${ARGS[@]}"
cargo test --package execution-core --test execution_trace_tests "${ARGS[@]}"
cargo test --package execution-core --test runtime_validation_stress_tests "${ARGS[@]}"
cargo test --package execution-core --test world_load_simulation_tests "${ARGS[@]}"
cargo test --package execution-core --test jurisprudence_lineage_tests "${ARGS[@]}"
cargo test --package execution-core --test root_recompute_tests "${ARGS[@]}"

printf 'Full runtime validation complete.\n'
