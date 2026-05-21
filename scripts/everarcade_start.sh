#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

STEP=0
TOTAL=9

step() {
  STEP=$((STEP + 1))
  echo "[$STEP/$TOTAL] $1"
}

fail() {
  echo
  echo "❌ Bootstrap failed: $1"
  echo "Run: $2"
  exit 1
}

run_step() {
  local desc="$1"
  local fix_cmd="$2"
  shift 2
  step "$desc"
  if ! "$@"; then
    fail "$desc" "$fix_cmd"
  fi
}

echo "🚀 EverArcade Developer Bootstrap"

echo
step "Checking environment..."
for dep in cargo rustc git bash; do
  if command -v "$dep" >/dev/null 2>&1; then
    echo "  ✅ $dep"
  else
    fail "$dep is not installed" "install $dep and re-run ./scripts/everarcade_start.sh"
  fi
done

step "Vendoring dependencies (if needed)..."
if [[ ! -d vendor ]]; then
  echo "  vendor/ missing; running ./scripts/vendor_deps.sh"
  if ! ./scripts/vendor_deps.sh; then
    fail "unable to vendor dependencies" "./scripts/vendor_deps.sh"
  fi
else
  echo "  ✅ vendor/ exists"
fi

run_step "Building core binaries (everarcade-cli, everarcade-host)..." "cargo build -p everarcade-cli -p everarcade-host" cargo build -p everarcade-cli -p everarcade-host
run_step "Initializing example world..." "cargo run -p everarcade-cli -- init-game first-world" cargo run -p everarcade-cli -- init-game first-world
run_step "Building game..." "cargo run -p everarcade-cli -- build-game" cargo run -p everarcade-cli -- build-game
run_step "Packaging game..." "cargo run -p everarcade-cli -- package-game" cargo run -p everarcade-cli -- package-game
run_step "Launching local federation..." "cargo run -p everarcade-cli -- run-local-federation" cargo run -p everarcade-cli -- run-local-federation
run_step "Generating replay artifacts..." "cargo run -p everarcade-cli -- replay-world" cargo run -p everarcade-cli -- replay-world
run_step "Inspecting simulation..." "cargo run -p everarcade-cli -- inspect-simulation" cargo run -p everarcade-cli -- inspect-simulation

echo
cat <<'OUT'
✅ EverArcade bootstrapped successfully

Artifacts:
  .everarcade-dev/
  .everarcade-dev/replay.log
  .everarcade-dev/simulation.inspect

Next commands:
  cargo run -p everarcade-cli -- run-local-federation
  cargo run -p everarcade-cli -- replay-world
  cargo run -p everarcade-cli -- inspect-simulation
OUT
