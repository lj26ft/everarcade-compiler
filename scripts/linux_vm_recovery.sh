#!/usr/bin/env bash
set -euo pipefail

rm -rf .everarcade-recovery

cargo run -p everarcade-host -- init --state .everarcade-recovery
cargo run -p everarcade-host -- generate-fixture --output everarcade-host/tests/fixtures/civilization_package.bin
cargo run -p everarcade-host -- run --package everarcade-host/tests/fixtures/civilization_package.bin --state .everarcade-recovery
rm -f .everarcade-recovery/node_manifest.json
cargo run -p everarcade-host -- repair-manifest --state .everarcade-recovery
cargo run -p everarcade-host -- verify --state .everarcade-recovery
