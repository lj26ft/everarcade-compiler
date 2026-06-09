#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PROOF_ROOT="/tmp/everarcade-local-launch-proof"
PROJECT_DIR="$PROOF_ROOT/audit-arena"
RUNTIME_ROOT="$PROOF_ROOT/runtime-root"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_FILE="$REPORT_DIR/local_game_launch_validation_report.txt"

rm -rf "$PROOF_ROOT"
mkdir -p "$REPORT_DIR"

run_step() {
  local label="$1"
  shift
  local output
  if output="$($@ 2>&1)"; then
    printf '%s\n' "$output"
    printf '%s: PASS\n' "$label"
    printf '%s: PASS\n%s\n\n' "$label" "$output" >> "$REPORT_FILE.tmp"
  else
    local status=$?
    printf '%s\n' "$output"
    printf '%s: FAIL\n' "$label"
    printf '%s: FAIL\n%s\n\n' "$label" "$output" >> "$REPORT_FILE.tmp"
    exit "$status"
  fi
}

: > "$REPORT_FILE.tmp"
{
  printf 'Local Game Launch Validation Report\n'
  printf 'Classification: Runtime Boot Proven\n'
  printf 'Proof root: %s\n\n' "$PROOF_ROOT"
} >> "$REPORT_FILE.tmp"

cd "$ROOT_DIR"
run_step "Creator Create" node creator-sdk/cli/everarcade.mjs new --template arena --name audit-arena --dir "$PROJECT_DIR"
run_step "Creator Build" node creator-sdk/cli/everarcade.mjs build --project "$PROJECT_DIR"
run_step "Creator Test" node creator-sdk/cli/everarcade.mjs test --project "$PROJECT_DIR"
run_step "Runtime Package" node creator-sdk/cli/everarcade.mjs package --project "$PROJECT_DIR"
run_step "Runtime Start" node creator-sdk/cli/everarcade.mjs launch-local --project "$PROJECT_DIR" --runtime-root "$RUNTIME_ROOT"

require_file() {
  local label="$1"
  local file="$2"
  if [[ -f "$file" ]]; then
    printf '%s: PASS\n' "$label"
    printf '%s: PASS (%s)\n' "$label" "$file" >> "$REPORT_FILE.tmp"
  else
    printf '%s: FAIL missing %s\n' "$label" "$file"
    printf '%s: FAIL missing %s\n' "$label" "$file" >> "$REPORT_FILE.tmp"
    exit 1
  fi
}

require_file "Manifest Evidence" "$PROJECT_DIR/dist/runtime-package/manifest.json"
require_file "WASM Evidence" "$PROJECT_DIR/dist/runtime-package/world.wasm"
require_file "World Metadata Evidence" "$PROJECT_DIR/dist/runtime-package/world.json"
require_file "Launch Report Evidence" "$PROJECT_DIR/dist/local-launch-report.json"
require_file "Runtime Start Report" "$RUNTIME_ROOT/reports/runtime_start_report.json"
require_file "Session Evidence" "$RUNTIME_ROOT/worlds/audit-arena-world/sessions/session-0001.json"
require_file "Projection Evidence" "$RUNTIME_ROOT/worlds/audit-arena-world/projections/projection-0001.json"

PROJECT_DIR="$PROJECT_DIR" RUNTIME_ROOT="$RUNTIME_ROOT" node <<'NODE'
const fs = require('fs');
const path = require('path');
const projectDir = process.env.PROJECT_DIR;
const runtimeRoot = process.env.RUNTIME_ROOT;
const manifest = JSON.parse(fs.readFileSync(path.join(projectDir, 'dist/runtime-package/manifest.json'), 'utf8'));
const world = JSON.parse(fs.readFileSync(path.join(projectDir, 'dist/runtime-package/world.json'), 'utf8'));
const start = JSON.parse(fs.readFileSync(path.join(runtimeRoot, 'reports/runtime_start_report.json'), 'utf8'));
const session = JSON.parse(fs.readFileSync(path.join(runtimeRoot, 'worlds/audit-arena-world/sessions/session-0001.json'), 'utf8'));
const projection = JSON.parse(fs.readFileSync(path.join(runtimeRoot, 'worlds/audit-arena-world/projections/projection-0001.json'), 'utf8'));
if (!/^[a-f0-9]{64}$/.test(manifest.wasm_hash)) throw new Error('invalid wasm_hash');
if (manifest.signature !== `sha256:${manifest.wasm_hash}`) throw new Error('invalid signature');
if (manifest.package_id !== 'audit-arena') throw new Error('invalid package_id');
if (manifest.runtime_compatibility !== 'everarcade-runtime-v0.1') throw new Error('invalid runtime compatibility');
if (world.classification !== 'deterministic-placeholder-wasm') throw new Error('invalid world classification');
for (const evidence of [start, session, projection]) {
  for (const key of ['world_id', 'package_id', 'package_hash', 'runtime_version', 'status', 'classification']) {
    if (!evidence[key]) throw new Error(`missing evidence field ${key}`);
  }
}
if (projection.non_authoritative_projection !== true) throw new Error('projection must be non-authoritative');
NODE
printf 'Runtime Evidence Fields: PASS\n'
printf 'Runtime Evidence Fields: PASS\n' >> "$REPORT_FILE.tmp"
printf '\nLocal Game Launch Validation: PASS\n' | tee -a "$REPORT_FILE.tmp"
mv "$REPORT_FILE.tmp" "$REPORT_FILE"
