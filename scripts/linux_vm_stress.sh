#!/usr/bin/env bash
set -euo pipefail

rm -rf .everarcade-stress
cargo build --workspace
cargo test --workspace

cargo run -p everarcade-host -- init --state .everarcade-stress
cargo run -p everarcade-host -- generate-fixture --output everarcade-host/tests/fixtures/civilization_package.bin

for i in $(seq 1 50); do
  cargo run -p everarcade-host -- run --package everarcade-host/tests/fixtures/civilization_package.bin --state .everarcade-stress
  cargo run -p everarcade-host -- verify --state .everarcade-stress
done

cargo run -p everarcade-host -- status --state .everarcade-stress --storage
