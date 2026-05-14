#!/usr/bin/env bash
set -euo pipefail

cargo build --workspace
cargo test --workspace

cargo run -p everarcade-host \
  -- init \
  --state .everarcade-smoke

cargo run -p everarcade-host \
  -- generate-fixture \
  --output everarcade-host/tests/fixtures/civilization_package.bin

cargo run -p everarcade-host \
  -- run \
  --package everarcade-host/tests/fixtures/civilization_package.bin \
  --state .everarcade-smoke

cargo run -p everarcade-host \
  -- verify \
  --state .everarcade-smoke

cargo run -p everarcade-host \
  -- status \
  --state .everarcade-smoke
