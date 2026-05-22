#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

STEP=0
TOTAL=5

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
run_step "Starting runtime flow..." "cargo run -p everarcade-cli -- start" cargo run -p everarcade-cli -- start

step "Checking runtime artifacts..."
for f in \
  runtime/world/status.txt \
  runtime/replay/latest/frame-0001.json \
  runtime/games/2d-arena/game.toml \
  clients/web-reference/index.html; do
  [[ -f "$f" ]] || fail "missing required artifact: $f" "cargo run -p everarcade-cli -- start"
  echo "  ✅ $f"
done

echo
cat <<'OUT'
✅ EverArcade bootstrapped successfully

Artifacts:
  runtime/world/status.txt
  runtime/replay/latest/frame-0001.json
  runtime/games/2d-arena/
  clients/web-reference/index.html

Next commands:
  cargo run -p everarcade-cli -- start
  cargo run -p everarcade-cli -- start-game 2d-arena
  cargo run -p everarcade-cli -- list-games
OUT
