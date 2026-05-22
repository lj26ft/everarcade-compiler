#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

pass() { echo "✅ PASS: $1"; }
fail() { echo "❌ FAIL: $1"; echo "   Fix: $2"; }

status=0

if command -v rustc >/dev/null 2>&1 && command -v cargo >/dev/null 2>&1; then
  pass "Rust toolchain installed"
else
  fail "Rust toolchain missing" "curl https://sh.rustup.rs -sSf | sh"
  status=1
fi

if [[ -d vendor ]]; then
  pass "vendor/ exists"
else
  fail "vendor/ missing" "./scripts/vendor_deps.sh"
  status=1
fi

if cargo check --workspace -q; then
  pass "workspace builds"
else
  fail "workspace build check failed" "cargo check --workspace"
  status=1
fi

if cargo build -p everarcade-cli -q; then
  pass "everarcade-cli compiles"
else
  fail "everarcade-cli compile failed" "cargo build -p everarcade-cli"
  status=1
fi

if cargo build -p everarcade-host -q; then
  pass "everarcade-host compiles"
else
  fail "everarcade-host compile failed" "cargo build -p everarcade-host"
  status=1
fi

if [[ -d runtime ]]; then
  pass "runtime/ exists"
else
  fail "runtime/ missing" "cargo run -p everarcade-cli -- start"
  status=1
fi

for f in \
  runtime/world/status.txt \
  runtime/replay/latest/frame-0001.json \
  runtime/games/2d-arena/game.toml \
  runtime/config/runtime.toml \
  clients/web-reference/index.html; do
  if [[ -f "$f" ]]; then
    pass "artifact present: $f"
  else
    fail "artifact missing: $f" "cargo run -p everarcade-cli -- start"
    status=1
  fi
done

exit "$status"
