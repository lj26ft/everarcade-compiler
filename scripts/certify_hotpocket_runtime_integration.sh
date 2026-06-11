#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
node runtime/hotpocket-runtime-proof/validation/hotpocket-runtime-proof.js validate >/tmp/everarcade_hotpocket_runtime_validation.out
cat /tmp/everarcade_hotpocket_runtime_validation.out
REPORT="runtime/hotpocket-runtime-proof/reports/hotpocket_runtime_validation_report.txt"
required=(
  "Deployment proof: PASS"
  "Cluster discovery: PASS"
  "Runtime execution proof: PASS"
  "Receipt proof: PASS"
  "Journal proof: PASS"
  "Checkpoint proof: PASS"
  "Validator agreement proof: PASS"
  "Replay proof: PASS"
  "Restore proof: PASS"
  "Round-trip proof: PASS"
)
for needle in "${required[@]}"; do
  if ! grep -Fq "$needle" "$REPORT"; then
    echo "Missing certification gate: $needle" >&2
    exit 1
  fi
done
if ! grep -Fq "EverArcade Runtime ↔ HotPocket Integration Proof v0.1: PASS" "$REPORT"; then
  echo "Runtime integration proof did not pass" >&2
  exit 1
fi
echo "EverArcade Runtime ↔ HotPocket Integration Proof v0.1: PASS"
