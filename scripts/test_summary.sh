#!/usr/bin/env bash
set -euo pipefail

list_output=$(cargo test --workspace -- --list)
printf '%s\n' "$list_output"

test_count=$(printf '%s\n' "$list_output" | rg -c ': test$' || true)
echo ""
echo "discoverable_tests=$test_count"
