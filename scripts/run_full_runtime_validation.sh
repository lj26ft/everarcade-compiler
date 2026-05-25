#!/usr/bin/env bash
set -euo pipefail

ARGS=("$@")

run_stage() {
  local label="$1"
  shift

  printf '[runtime-validation] START %s\n' "$label"
  "$@"
  printf '[runtime-validation] PASS  %s\n' "$label"
}

run_stage vendor_deps bash scripts/vendor_deps.sh
run_stage wasm_runtime_tests cargo test --package execution-core --test wasm_runtime_tests "${ARGS[@]}"
run_stage replay_validation_tests cargo test --package execution-core --test replay_validation_tests "${ARGS[@]}"
run_stage wasm_stateful_execution_tests cargo test --package execution-core --test wasm_stateful_execution_tests "${ARGS[@]}"
run_stage guest_runtime_tests cargo test --package execution-core --test guest_runtime_tests "${ARGS[@]}"
run_stage checkpoint_exchange_tests cargo test --package execution-core --test checkpoint_exchange_tests "${ARGS[@]}"
run_stage tenant_isolation_tests cargo test --package execution-core --test tenant_isolation_tests "${ARGS[@]}"
run_stage world_epoch_tests cargo test --package execution-core --test world_epoch_tests "${ARGS[@]}"
run_stage recovery_operations_tests cargo test --package execution-core --test recovery_operations_tests "${ARGS[@]}"
run_stage epoch_transition_tests cargo test --package execution-core --test epoch_transition_tests "${ARGS[@]}"
run_stage world_partition_tests cargo test --package execution-core --test world_partition_tests "${ARGS[@]}"
run_stage incremental_world_runtime_tests cargo test --package execution-core --test incremental_world_runtime_tests "${ARGS[@]}"
run_stage scheduler_tests cargo test --package execution-core --test scheduler_tests "${ARGS[@]}"
run_stage execution_trace_tests cargo test --package execution-core --test execution_trace_tests "${ARGS[@]}"
run_stage runtime_validation_stress_tests cargo test --package execution-core --test runtime_validation_stress_tests "${ARGS[@]}"
run_stage world_load_simulation_tests cargo test --package execution-core --test world_load_simulation_tests "${ARGS[@]}"
run_stage jurisprudence_lineage_tests cargo test --package execution-core --test jurisprudence_lineage_tests "${ARGS[@]}"
run_stage root_recompute_tests cargo test --package execution-core --test root_recompute_tests "${ARGS[@]}"

printf '[runtime-validation] COMPLETE all stages\n'
