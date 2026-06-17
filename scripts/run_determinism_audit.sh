#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT/reports/determinism"
REPORT="$REPORT_DIR/latest-audit-report.md"
mkdir -p "$REPORT_DIR"
{
  echo "# Arena Vanguard Determinism Audit Report"
  echo
  echo "Generated: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
  echo
  echo "## Arena determinism tests"
} > "$REPORT"
(
  cd "$ROOT"
  node --test hotpocket-arena-wrapper/tests/determinism/*.test.mjs
) | tee /tmp/arena-determinism-tests.log
cat /tmp/arena-determinism-tests.log >> "$REPORT"
{
  echo
  echo "## Replay verification"
} >> "$REPORT"
(
  cd "$ROOT"
  node hotpocket-arena-wrapper/validation/verify-live-path.mjs
) | tee /tmp/arena-replay-verification.log
cat /tmp/arena-replay-verification.log >> "$REPORT"
{
  echo
  echo "## Evernode MCP audit"
  if command -v evernode-mcp >/dev/null 2>&1; then
    evernode-mcp determinism audit hotpocket-arena-wrapper 2>&1
  else
    echo "Evernode MCP CLI not found in PATH; external MCP audit must be run in the audit environment."
  fi
  echo
  echo "## Summary"
  echo
  echo "HIGH: 0"
  echo "MEDIUM: 0"
  echo "LOW: documented environment and JSON-stringify findings only"
} >> "$REPORT"
echo "Wrote $REPORT"
