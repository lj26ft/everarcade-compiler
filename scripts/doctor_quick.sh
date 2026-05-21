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

if [[ -d .everarcade-dev ]]; then
  pass "runtime directory .everarcade-dev exists"
else
  fail "runtime directory .everarcade-dev missing" "./scripts/everarcade_start.sh"
  status=1
fi

exit "$status"
