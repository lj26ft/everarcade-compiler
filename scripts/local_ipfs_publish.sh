#!/usr/bin/env bash
set -euo pipefail
cargo test -p everarcade-host --features ipfs-live --test ipfs_publication_tests
