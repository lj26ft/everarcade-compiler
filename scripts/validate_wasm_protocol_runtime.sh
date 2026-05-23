#!/usr/bin/env bash
set -euo pipefail
bash scripts/verify_wasm_execution_equivalence.sh
bash scripts/verify_wasm_dag_equivalence.sh
echo "wasm protocol runtime validation passed"
