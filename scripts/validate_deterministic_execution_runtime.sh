#!/usr/bin/env bash
set -euo pipefail

bash scripts/verify_execution_equivalence.sh
bash scripts/verify_execution_replay.sh
echo "deterministic execution runtime validation passed"
