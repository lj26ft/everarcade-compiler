#!/usr/bin/env bash
set -euo pipefail

# Deployment proof runner for operator baseline validation.
# Default mode is dry-run to ensure merge-safe checks in CI/local dev.

DRY_RUN="${DRY_RUN:-1}"
ENABLE_XRPL_LIVE="${ENABLE_XRPL_LIVE:-0}"
ENABLE_IPFS_LIVE="${ENABLE_IPFS_LIVE:-0}"
ENABLE_EVERNODE_LIVE="${ENABLE_EVERNODE_LIVE:-0}"

run() {
  echo "+ $*"
  "$@"
}

if [[ "$DRY_RUN" == "1" ]]; then
  export EVERARCADE_DRY_RUN=1
  export XRPL_ADAPTER=dry-run
  export IPFS_ADAPTER=dry-run
  export EVERNODE_ADAPTER=dry-run
else
  export EVERARCADE_DRY_RUN=0
  export XRPL_ADAPTER="${XRPL_ADAPTER:-live}"
  export IPFS_ADAPTER="${IPFS_ADAPTER:-live}"
  export EVERNODE_ADAPTER="${EVERNODE_ADAPTER:-live}"
fi

echo "deploy_proof: dry_run=$EVERARCADE_DRY_RUN xrpl_adapter=$XRPL_ADAPTER ipfs_adapter=$IPFS_ADAPTER evernode_adapter=$EVERNODE_ADAPTER"

echo "== hygiene =="
run ./scripts/check_no_empty_tests.sh
run ./scripts/check_no_generated_artifacts_tracked.sh

echo "== operator baseline stack =="
run ./scripts/linux_vm_smoke.sh
run ./scripts/linux_vm_recovery.sh
run ./scripts/linux_vm_stress.sh
run ./scripts/distributed_execution.sh
run ./scripts/distributed_execution_recovery.sh
run ./scripts/distributed_receipt_sync.sh
run ./scripts/local_cluster.sh
run ./scripts/local_cluster_recovery.sh
run ./scripts/workload_partition.sh
run ./scripts/networked_receipt_propagation.sh
run ./scripts/operator_failover.sh
run ./scripts/reassignment_recovery.sh
run ./scripts/networked_cluster.sh
run ./scripts/networked_checkpoint_sync.sh
run ./scripts/network_partition.sh
run ./scripts/resumable_sync.sh
run ./scripts/stale_node_recovery.sh

echo "== optional live integrations (feature flagged) =="
if [[ "$ENABLE_XRPL_LIVE" == "1" ]]; then
  run ./scripts/local_xrpl_testnet.sh
else
  echo "skip xrpl-live (set ENABLE_XRPL_LIVE=1 to enable)"
fi

if [[ "$ENABLE_IPFS_LIVE" == "1" ]]; then
  run ./scripts/local_ipfs_publish.sh
else
  echo "skip ipfs-live (set ENABLE_IPFS_LIVE=1 to enable)"
fi

if [[ "$ENABLE_EVERNODE_LIVE" == "1" ]]; then
  echo "evernode-live adapter hook enabled (no dedicated live script in this repo)"
else
  echo "skip evernode-live (set ENABLE_EVERNODE_LIVE=1 to enable adapter hooks)"
fi

echo "deploy_proof=ok"
