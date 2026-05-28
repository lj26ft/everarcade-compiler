#!/usr/bin/env bash
set -euo pipefail
cargo test --package execution-core --test live_peer_io_tests "$@"
echo "validation=ok script=$(basename "$0") replay_only=true non_authoritative=true"
