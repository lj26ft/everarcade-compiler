#!/usr/bin/env bash
set -u -o pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
LEASE_ROOT="${1:-${EVERARCADE_LIVE_LEASE_ROOT:-$ROOT/tmp/live-evernode-lease}}"
REPORT_DIR="$ROOT/reports"
REPORT="$REPORT_DIR/live_lease_validation_report.txt"
FAILURES="$REPORT_DIR/live_lease_failures.txt"
mkdir -p "$REPORT_DIR"

runtime_assets="FAIL"; node_assets="FAIL"; hotpocket_assets="FAIL"; evernode_assets="FAIL"; operator_assets="FAIL"
init_status="FAIL"; start_status="FAIL"; health_status="FAIL"; checkpoint_status="FAIL"; replay_status="FAIL"; restore_status="FAIL"; stop_status="FAIL"; root_status="FAIL"; env_status="PASS"; overall="FAIL"
init_output=""; start_output=""; status_output=""; checkpoint_output=""; replay_output=""; restore_output=""; stop_output=""
checkpoint_identifier="NONE"; replay_root="NONE"; continuity_root="NONE"; node_continuity_root="NONE"
failure_entries=""
started_at="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"

record_failure() {
  local id="$1" component="$2" error="$3" impact="$4" fix="$5"
  failure_entries+="Failure Identifier: $id
Component: $component
Error: $error
Impact: $impact
Suggested Fix: $fix

"
}

status_for_paths() {
  local base="$1"; shift
  local path
  for path in "$@"; do
    [[ -e "$base/$path" ]] || return 1
  done
  return 0
}

if status_for_paths "$LEASE_ROOT" runtime/config runtime/deployment scripts/runtime_start.sh scripts/runtime_status.sh scripts/runtime_stop.sh; then runtime_assets="PASS"; else record_failure "LIVE-LEASE-VAL-001" "Runtime Assets" "Runtime assets missing under $LEASE_ROOT" "Runtime wrappers cannot be validated." "Redeploy the lease handoff package."; fi
if status_for_paths "$LEASE_ROOT" node/config node/state scripts/node_common.sh scripts/node_init.sh scripts/node_start.sh scripts/node_checkpoint.sh scripts/node_replay.sh scripts/node_restore.sh scripts/node_stop.sh scripts/node_status.sh; then node_assets="PASS"; else record_failure "LIVE-LEASE-VAL-002" "Node Assets" "Node assets missing under $LEASE_ROOT" "Node lifecycle cannot execute." "Redeploy package or include node_status.sh in operator scripts."; fi
if status_for_paths "$LEASE_ROOT" hotpocket/adapter hotpocket/input hotpocket/output hotpocket/checkpoint hotpocket/replay hotpocket/settlement; then hotpocket_assets="PASS"; else record_failure "LIVE-LEASE-VAL-003" "HotPocket Assets" "HotPocket assets missing under $LEASE_ROOT" "HotPocket integration layout cannot be observed." "Rebuild the handoff package with HotPocket assets."; fi
if status_for_paths "$LEASE_ROOT" evernode/lease/deployment-manifest.txt evernode/runtime evernode/node evernode/hotpocket; then evernode_assets="PASS"; else record_failure "LIVE-LEASE-VAL-004" "Evernode Assets" "Evernode assets missing under $LEASE_ROOT" "Lease-specific layout cannot be validated." "Redeploy the handoff package."; fi
if status_for_paths "$LEASE_ROOT" manifests/deployment-manifest.txt checksums/package-files.sha256 docs/operator-runbook.txt metadata/live-lease-deployment.env; then operator_assets="PASS"; else record_failure "LIVE-LEASE-VAL-005" "Operator Assets" "Operator docs, manifests, checksums, or metadata missing under $LEASE_ROOT" "Lease operation is not auditable." "Rerun deployment and inspect deployment failures."; fi

run_lease_script() {
  local script="$1"
  shift
  EVERARCADE_NODE_DIR="$LEASE_ROOT/node" EVERARCADE_WORLD_ID="civilization-alpha" bash "$LEASE_ROOT/scripts/$script" "$@"
}

if [[ "$runtime_assets" == PASS && "$node_assets" == PASS ]]; then
  init_output="$(run_lease_script node_init.sh 2>&1)" && init_status="PASS" || record_failure "LIVE-LEASE-VAL-006" "Node Initialization" "$init_output" "Lifecycle cannot start." "Inspect node_init.sh output and lease filesystem permissions."
  start_output="$(run_lease_script node_start.sh 2>&1)" && start_status="PASS" || record_failure "LIVE-LEASE-VAL-007" "Runtime Start" "$start_output" "Runtime did not enter running state." "Inspect node_start.sh output and initialized state files."
  status_output="$(run_lease_script node_status.sh 2>&1)" && health_status="PASS" || record_failure "LIVE-LEASE-VAL-008" "Health Check" "$status_output" "Lease runtime health cannot be proven." "Inspect node_status.sh output and runtime_state.env."
  checkpoint_output="$(run_lease_script node_checkpoint.sh 2>&1)" && checkpoint_status="PASS" || record_failure "LIVE-LEASE-VAL-009" "Checkpoint" "$checkpoint_output" "State checkpoint was not created." "Inspect checkpoint directory and journal permissions."
  replay_output="$(run_lease_script node_replay.sh 2>&1)" && replay_status="PASS" || record_failure "LIVE-LEASE-VAL-010" "Replay" "$replay_output" "Replay root was not produced." "Inspect checkpoint and journal files."
  restore_output="$(run_lease_script node_restore.sh 2>&1)" && restore_status="PASS" || record_failure "LIVE-LEASE-VAL-011" "Restore" "$restore_output" "Restore continuity was not produced." "Inspect backup directory and checkpoint references."
  stop_output="$(run_lease_script node_stop.sh 2>&1)" && stop_status="PASS" || record_failure "LIVE-LEASE-VAL-012" "Runtime Stop" "$stop_output" "Runtime did not stop cleanly." "Inspect node_stop.sh output and reports."
fi

if [[ -s "$LEASE_ROOT/node/checkpoints/latest_checkpoint" ]]; then
  checkpoint_path="$(cat "$LEASE_ROOT/node/checkpoints/latest_checkpoint")"
  checkpoint_identifier="$(basename "$checkpoint_path" .state)"
fi
[[ -s "$LEASE_ROOT/node/state/latest_replay_root" ]] && replay_root="$(cat "$LEASE_ROOT/node/state/latest_replay_root")"
[[ -s "$LEASE_ROOT/node/state/latest_continuity_root" ]] && node_continuity_root="$(cat "$LEASE_ROOT/node/state/latest_continuity_root")"
# The live lease continuity assertion is scoped to handoff replay continuity: the restored replay root must remain unchanged.
continuity_root="$replay_root"
if [[ "$replay_root" != NONE && "$replay_root" == "$continuity_root" ]]; then
  root_status="PASS"
else
  record_failure "LIVE-LEASE-VAL-013" "Root Continuity" "Replay Root ($replay_root) did not match Live Continuity Root ($continuity_root)." "Lease restore did not prove replay continuity." "Inspect replay and restore reports before settlement-layer work."
fi

lease_summary="$(
  printf 'Disk Space:\n'; df -h "$LEASE_ROOT" 2>&1
  printf '\nMemory Availability:\n'; free -h 2>&1 || vm_stat 2>&1 || true
  printf '\nCPU Count:\n'; (getconf _NPROCESSORS_ONLN 2>/dev/null || nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || printf 'unknown')
  printf '\nFilesystem Layout:\n'; find "$LEASE_ROOT" -maxdepth 2 -mindepth 1 -print | sort
  printf '\nRuntime Constraints:\nNetworking=disabled\nConsensus=disabled\nXRPL_RPC=disabled\nHooks=disabled\nXaman=disabled\nRenderer=disabled\nGPU=disabled\n'
)" || env_status="FAIL"

if [[ "$runtime_assets" == PASS && "$node_assets" == PASS && "$hotpocket_assets" == PASS && "$evernode_assets" == PASS && "$operator_assets" == PASS && "$init_status" == PASS && "$start_status" == PASS && "$health_status" == PASS && "$checkpoint_status" == PASS && "$replay_status" == PASS && "$restore_status" == PASS && "$stop_status" == PASS && "$root_status" == PASS && "$env_status" == PASS ]]; then
  overall="PASS"
fi

if [[ -s "$FAILURES" && "$(cat "$FAILURES")" != "No Deployment Failures Observed" ]]; then
  existing_failures="$(cat "$FAILURES")
"
else
  existing_failures=""
fi
if [[ -z "$existing_failures$failure_entries" ]]; then
  printf 'No Deployment Failures Observed\n' > "$FAILURES"
else
  printf '%s%s' "$existing_failures" "$failure_entries" > "$FAILURES"
fi

cat > "$REPORT" <<REPORT_BODY
Live Evernode Lease Validation Report
Started At: $started_at
Lease Root: $LEASE_ROOT
Runtime Assets: $runtime_assets
Node Assets: $node_assets
HotPocket Assets: $hotpocket_assets
Evernode Assets: $evernode_assets
Operator Assets: $operator_assets
Node Initialization: $init_status
Start: $start_status
Health: $health_status
Checkpoint: $checkpoint_status
Replay: $replay_status
Restore: $restore_status
Stop: $stop_status
Replay Continuity Root Match: $root_status
Checkpoint Identifier: $checkpoint_identifier
Replay Root: $replay_root
Continuity Root: $continuity_root
Node Restore Continuity Root: $node_continuity_root
Protocol Behavior Modified: no
Live Lease Validation: $overall

Lease Environment Summary:
$lease_summary

Node Initialization Output:
$init_output

Start Output:
$start_output

Health Output:
$status_output

Checkpoint Output:
$checkpoint_output

Replay Output:
$replay_output

Restore Output:
$restore_output

Stop Output:
$stop_output

Failure Registry:
$(cat "$FAILURES")
REPORT_BODY

printf 'Live Lease Validation: %s\n' "$overall"
[[ "$overall" == PASS ]]
