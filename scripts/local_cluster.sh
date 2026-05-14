#!/usr/bin/env bash
set -euo pipefail
cargo test -p everarcade-host --test local_peer_tests --test cluster_sync_tests --test distributed_convergence_tests
