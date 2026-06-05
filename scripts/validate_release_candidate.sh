#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/release_candidate_validation_report.txt"
mkdir -p "$REPORT_DIR"

hash_dir() {
  local dir_rel="$1"
  local root_file="$2"
  python3 - "$ROOT_DIR" "$dir_rel" "$root_file" <<'PY'
from pathlib import Path
import hashlib
import sys
root = Path(sys.argv[1])
dir_rel = sys.argv[2]
root_file = sys.argv[3]
d = root / dir_rel
h = hashlib.sha256()
for path in sorted(d.iterdir()):
    if not path.is_file() or path.name == root_file:
        continue
    rel = path.relative_to(root).as_posix()
    h.update(rel.encode() + b"\0" + path.read_bytes() + b"\0")
print(h.hexdigest())
PY
}

check_root() {
  local label="$1"
  local dir_rel="$2"
  local root_file="$3"
  local required_count="$4"
  local root_path="$ROOT_DIR/$dir_rel/$root_file"

  if [[ ! -d "$ROOT_DIR/$dir_rel" ]]; then
    printf '%s: FAIL (missing directory %s)\n' "$label" "$dir_rel"
    return 1
  fi

  local count
  count="$(find "$ROOT_DIR/$dir_rel" -maxdepth 1 -type f ! -name "$root_file" | wc -l | tr -d ' ')"
  if (( count < required_count )); then
    printf '%s: FAIL (expected at least %s files, found %s)\n' "$label" "$required_count" "$count"
    return 1
  fi

  if [[ ! -f "$root_path" ]]; then
    printf '%s: FAIL (missing %s)\n' "$label" "$dir_rel/$root_file"
    return 1
  fi

  local expected actual
  expected="$(tr -d '[:space:]' < "$root_path")"
  actual="$(hash_dir "$dir_rel" "$root_file")"
  if [[ "$expected" != "$actual" ]]; then
    printf '%s: FAIL (root mismatch)\n' "$label"
    printf '  expected: %s\n' "$expected"
    printf '  actual:   %s\n' "$actual"
    return 1
  fi

  printf '%s: PASS\n' "$label"
}

check_file() {
  local label="$1"
  local path_rel="$2"
  if [[ -s "$ROOT_DIR/$path_rel" ]]; then
    printf '%s: PASS\n' "$label"
  else
    printf '%s: FAIL (missing %s)\n' "$label" "$path_rel"
    return 1
  fi
}

run_checks() {
  check_root "Manifest" "release/manifest" "RELEASE_MANIFEST_ROOT" 5
  check_root "Freeze" "release/interface-freeze" "INTERFACE_FREEZE_ROOT" 6
  check_root "Upgrade" "release/upgrade-certification" "UPGRADE_ROOT" 2
  check_root "Recovery" "release/recovery-certification" "RECOVERY_ROOT" 2
  check_root "Distribution" "release/distribution-certification" "DISTRIBUTION_ROOT" 2
  check_root "Testnet Readiness" "release/testnet-readiness" "TESTNET_READINESS_ROOT" 2
  check_file "Deployment Runbook" "docs/runtime-platform/deployment-runbook-v0.1.md"
  check_file "Operator Runbook" "docs/runtime-platform/operator-runbook-v0.1.md"
  check_file "Recovery Runbook" "docs/runtime-platform/recovery-runbook-v0.1.md"
  check_file "Upgrade Runbook" "docs/runtime-platform/upgrade-runbook-v0.1.md"
  check_file "Developer Runbook" "docs/runtime-platform/developer-runbook-v0.1.md"
  check_file "Release Documentation" "docs/runtime-platform/release-candidate-v0.1.md"
}

timestamp="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
status="PASS"
output="$(run_checks 2>&1)" || status="FAIL"

{
  printf 'EverArcade Release Candidate Validation Report\n'
  printf 'Version: v0.1-rc1\n'
  printf 'Timestamp: %s\n\n' "$timestamp"
  printf '%s\n\n' "$output"
  printf 'Release Candidate Validation: %s\n' "$status"
} > "$REPORT_PATH"

cat "$REPORT_PATH"
[[ "$status" == "PASS" ]]
