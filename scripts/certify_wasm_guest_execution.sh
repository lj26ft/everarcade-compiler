#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VALIDATION_REPORT="$ROOT/reports/wasm_guest_execution_validation_report.txt"
CERT_REPORT="$ROOT/reports/wasm_guest_execution_certification_report.txt"
mkdir -p "$ROOT/reports"
bash "$ROOT/scripts/validate_wasm_guest_execution.sh" >/tmp/everarcade-wasm-guest-validation.out
cat "$VALIDATION_REPORT" > "$CERT_REPORT"
required=(
  "Guest Build: PASS"
  "Guest Package: PASS"
  "Guest Load: PASS"
  "Guest Execute: PASS"
  "Guest State Mutation: PASS"
  "Guest Receipt Generation: PASS"
  "Guest Journal Generation: PASS"
  "Guest Replay Generation: PASS"
  "Guest Replay Verification: PASS"
)
for line in "${required[@]}"; do
  grep -Fx "$line" "$VALIDATION_REPORT" >/dev/null
done
printf '\nWASM Guest Execution Proof v0.1: PASS\n' >> "$CERT_REPORT"
cat "$CERT_REPORT"
