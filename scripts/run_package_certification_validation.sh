#!/usr/bin/env bash
set -euo pipefail
CARGO_BUILD_JOBS=${CARGO_BUILD_JOBS:-1} cargo test -p tools --offline --locked
