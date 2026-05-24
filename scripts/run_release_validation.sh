#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
mkdir -p "$ROOT/deployment/reports" "$ROOT/deployment/reports/logs"
REPORT="$ROOT/deployment/reports/release_validation_report.md"
LOG="$ROOT/deployment/reports/logs/release_validation.log"
START="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
if cargo check --workspace --locked --frozen --offline >"$LOG" 2>&1 && cargo build --release --locked --frozen --offline -p everarcade-host >>"$LOG" 2>&1; then STATUS="✅ pass"; else STATUS="❌ fail"; fi
cat > "$REPORT" <<REPORT
# Release Validation Report

- Generated: $START
- Status: $STATUS
- Commands:
  - cargo check --workspace --locked --frozen --offline
  - cargo build --release --locked --frozen --offline -p everarcade-host
- Log: deployment/reports/logs/release_validation.log
REPORT
[[ "$STATUS" == "✅ pass" ]]
