#!/usr/bin/env bash
set -euo pipefail

# Deterministic runtime federation validation wrapper. Arguments are forwarded
# by milestone automation but this scaffold keeps execution offline/locked in
# the targeted execution-core test surface.
CARGO_BUILD_JOBS=${CARGO_BUILD_JOBS:-1} cargo test --package execution-core --test runtime_live_networking_tests --offline --locked
