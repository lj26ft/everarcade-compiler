#!/usr/bin/env bash
set -euo pipefail
cargo test --package execution-core --test projection_federation_tests "$@"
echo "status=pass deterministic=ok"
