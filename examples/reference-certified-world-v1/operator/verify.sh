#!/usr/bin/env bash
set -euo pipefail
ROOT="${1:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}"
for f in "$ROOT"/certification/*-report.txt "$ROOT"/proofs/formal/*-proof.txt; do grep -Eq 'PASS|EQUIVALENT' "$f"; done
grep -q 'REFERENCE CERTIFIED WORLD V1: PASS' "$ROOT/certification/final-report.txt"
grep -q 'world_arena_vanguard' "$ROOT/registry/reference-world-registry-entry.json"
printf 'REFERENCE CERTIFIED WORLD V1: PASS\n'
