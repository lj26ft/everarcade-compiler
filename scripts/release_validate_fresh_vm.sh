#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

phase() {
  echo
  echo "==> $*"
}

require_file() {
  local path="$1"
  if [[ ! -s "$path" ]]; then
    echo "ERROR: required file missing or empty: $path" >&2
    exit 1
  fi
}

require_json() {
  local path="$1"
  python3 - <<'PY' "$path"
import json, pathlib, sys
p = pathlib.Path(sys.argv[1])
json.loads(p.read_text())
PY
}

phase "Clean generated runtime state"
rm -rf runtime/.everarcade-dev runtime/world runtime/replay/latest runtime/logs
mkdir -p runtime/replay

phase "Vendor dependencies"
bash scripts/vendor_deps.sh

phase "Format check"
cargo fmt --all --check

phase "CLI tests"
cargo test -p everarcade-cli -q

phase "Source-tree runtime start"
cargo run -p everarcade-cli -- start

phase "Doctor check"
bash scripts/doctor_quick.sh

phase "Validate runtime distribution"
bash scripts/validate_runtime_distribution.sh

phase "Build VM runtime appliance"
bash scripts/build_vm_runtime_appliance.sh

phase "Fresh bootstrap path regression"
bash scripts/test_fresh_bootstrap_paths.sh

phase "Validate required runtime artifacts"
for file in \
  runtime/config/federation.toml \
  runtime/config/runtime.toml \
  runtime/config/storage.toml \
  runtime/games/2d-arena/README.md \
  runtime/games/2d-arena/assets.toml \
  runtime/games/2d-arena/client.toml \
  runtime/games/2d-arena/game.toml \
  runtime/games/2d-arena/simulation.toml \
  runtime/games/2d-arena/world.toml \
  runtime/manifests/xahau-hooks.toml \
  runtime/replay/latest/frame-0001.json \
  runtime/scripts/start-everarcade.sh \
  runtime/world/status.txt \
  clients/web-reference/index.html
  do
  require_file "$file"
done

phase "Validate replay JSON"
require_json runtime/replay/latest/frame-0001.json

phase "Unknown command must fail"
if cargo run -p everarcade-cli -- definitely-not-a-command; then
  echo "ERROR: unknown command unexpectedly succeeded" >&2
  exit 1
fi

phase "Fresh VM release validation complete"
