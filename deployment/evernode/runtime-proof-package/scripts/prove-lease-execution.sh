#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="${EVERARCADE_PROOF_REPORT_DIR:-$ROOT/reports}"
REMOTE_ROOT="${EVERNODE_REMOTE_ROOT:-~/everarcade-runtime-proof-package}"
ARCHIVE="${EVERARCADE_PROOF_ARCHIVE:-/tmp/everarcade-runtime-proof-package.tar.gz}"
mkdir -p "$REPORT_DIR"

require_env() {
  local missing=0
  for key in EVERNODE_LEASE_ID EVERNODE_HOST_ID EVERNODE_SSH_TARGET XRPL_TESTNET_SEED XRPL_TESTNET_DESTINATION; do
    if [[ -z "${!key:-}" ]]; then
      printf 'missing required environment variable: %s\n' "$key" >&2
      missing=1
    fi
  done
  return "$missing"
}
json_escape() { python3 -c 'import json,sys; print(json.dumps(sys.stdin.read().rstrip("\n")))'; }
write_fail_report() {
  local file="$1" message="$2"
  cat > "$REPORT_DIR/$file" <<JSON
{
  "schema": "everarcade.evernode.failure-report.v0.1",
  "generated_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "failure": $(printf '%s' "$message" | json_escape),
  "status": "FAIL"
}
JSON
}

if ! require_env; then
  write_fail_report lease_discovery_report.json "required real Evernode lease and XRPL Testnet environment variables were not provided"
  exit 1
fi
case "$EVERNODE_SSH_TARGET" in
  localhost|127.*|::1|0.0.0.0|*.local)
    write_fail_report lease_discovery_report.json "EVERNODE_SSH_TARGET must identify a reachable real Evernode lease, not a local/mock target"
    exit 1
    ;;
esac

SSH_OPTS=(-o BatchMode=yes -o StrictHostKeyChecking=accept-new -o ConnectTimeout=20)
remote_probe='set -euo pipefail
printf "hostname=%s\n" "$(hostname)"
printf "kernel=%s\n" "$(uname -a)"
printf "cpu_count=%s\n" "$(nproc 2>/dev/null || getconf _NPROCESSORS_ONLN)"
printf "memory_kb=%s\n" "$(awk "/MemTotal/ {print \$2}" /proc/meminfo)"
printf "storage_kb=%s\n" "$(df -Pk . | awk "NR==2 {print \$4}")"
printf "node=%s\n" "$(command -v node || true)"
printf "npm=%s\n" "$(command -v npm || true)"
printf "hotpocket=%s\n" "$(command -v hotpocket || command -v hp || true)"
printf "docker_env=%s\n" "$([[ -e /.dockerenv ]] && echo yes || echo no)"'
if ! probe_output="$(ssh "${SSH_OPTS[@]}" "$EVERNODE_SSH_TARGET" "$remote_probe" 2>&1)"; then
  write_fail_report lease_discovery_report.json "lease is not reachable over SSH: $probe_output"
  exit 1
fi
get_probe() { printf '%s\n' "$probe_output" | awk -F= -v k="$1" '$1==k {sub($1"=",""); print; exit}'; }
node_path="$(get_probe node)"
npm_path="$(get_probe npm)"
hotpocket_path="$(get_probe hotpocket)"
reachable=true
runtime_capability=PASS
[[ -n "$node_path" && -n "$npm_path" && -n "$hotpocket_path" ]] || runtime_capability=FAIL
cat > "$REPORT_DIR/lease_discovery_report.json" <<JSON
{
  "schema": "everarcade.evernode.lease-discovery-report.v0.1",
  "generated_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "lease_identifier": "$EVERNODE_LEASE_ID",
  "host_identifier": "$EVERNODE_HOST_ID",
  "ssh_target": "$EVERNODE_SSH_TARGET",
  "active_lease_state": "reachable",
  "runtime_capability": "$runtime_capability",
  "available_storage_kb": "$(get_probe storage_kb)",
  "available_memory_kb": "$(get_probe memory_kb)",
  "available_cpu": "$(get_probe cpu_count)",
  "node": "$node_path",
  "npm": "$npm_path",
  "hotpocket_runtime": "$hotpocket_path",
  "container_marker": "$(get_probe docker_env)",
  "status": "$runtime_capability"
}
JSON
[[ "$runtime_capability" == PASS ]] || exit 1

tar --exclude='./node_modules' --exclude='./artifacts' -C "$ROOT" -czf "$ARCHIVE" .
package_sha="$(sha256sum "$ARCHIVE" | awk '{print $1}')"
ssh "${SSH_OPTS[@]}" "$EVERNODE_SSH_TARGET" "rm -rf $REMOTE_ROOT && mkdir -p $REMOTE_ROOT"
scp "${SSH_OPTS[@]}" "$ARCHIVE" "$EVERNODE_SSH_TARGET:$REMOTE_ROOT/package.tar.gz"
ssh "${SSH_OPTS[@]}" "$EVERNODE_SSH_TARGET" "cd $REMOTE_ROOT && tar -xzf package.tar.gz && mkdir -p reports artifacts"
scp "${SSH_OPTS[@]}" "$REPORT_DIR/lease_discovery_report.json" "$EVERNODE_SSH_TARGET:$REMOTE_ROOT/reports/lease_discovery_report.json"

remote_run="cd $REMOTE_ROOT && npm install --omit=dev && EVERNODE_REAL_LEASE_PROOF=1 HOTPOCKET_RUNTIME_INTEGRATION_PROOF=1 EVERNODE_LEASE_ID='$EVERNODE_LEASE_ID' EVERNODE_HOST_ID='$EVERNODE_HOST_ID' EVERARCADE_PACKAGE_SHA256='$package_sha' npm start && XRPL_TESTNET_SEED='$XRPL_TESTNET_SEED' XRPL_TESTNET_DESTINATION='$XRPL_TESTNET_DESTINATION' XRPL_TESTNET_ENDPOINT='${XRPL_TESTNET_ENDPOINT:-wss://s.altnet.rippletest.net:51233}' npm run publish:xrpl"
if ! ssh "${SSH_OPTS[@]}" "$EVERNODE_SSH_TARGET" "$remote_run"; then
  scp "${SSH_OPTS[@]}" "$EVERNODE_SSH_TARGET:$REMOTE_ROOT/reports/*.json" "$REPORT_DIR/" 2>/dev/null || true
  exit 1
fi
scp "${SSH_OPTS[@]}" "$EVERNODE_SSH_TARGET:$REMOTE_ROOT/reports/*.json" "$REPORT_DIR/"
mkdir -p "$ROOT/artifacts"
scp "${SSH_OPTS[@]}" "$EVERNODE_SSH_TARGET:$REMOTE_ROOT/artifacts/*" "$ROOT/artifacts/" 2>/dev/null || true

python3 - "$REPORT_DIR" <<'PY'
import json, pathlib, sys, datetime
report_dir = pathlib.Path(sys.argv[1])
required = [
  'lease_discovery_report.json',
  'runtime_deployment_report.json',
  'lease_gameplay_execution_report.json',
  'lease_runtime_artifact_report.json',
  'lease_continuity_report.json',
  'lease_xrpl_anchor_report.json',
  'lease_replay_report.json',
]
rows = []
overall = True
for name in required:
    path = report_dir / name
    if not path.exists():
        rows.append((name, 'FAIL', 'missing'))
        overall = False
        continue
    data = json.loads(path.read_text())
    status = data.get('status')
    rows.append((name, status, 'present'))
    overall = overall and status == 'PASS'
lines = [
    'Evernode Lease Execution Proof v0.1 Certification Report',
    f'Generated At: {datetime.datetime.utcnow().replace(microsecond=0).isoformat()}Z',
]
lines.extend(f'{name}: {status} ({note})' for name, status, note in rows)
lines.append(f'Evernode Lease Execution Proof v0.1: {"PASS" if overall else "FAIL"}')
(report_dir / 'evernode_lease_execution_certification_report.txt').write_text('\n'.join(lines) + '\n')
print(lines[-1])
sys.exit(0 if overall else 1)
PY
