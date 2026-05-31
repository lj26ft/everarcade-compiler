# EverNode Operator Deploy Guide

## Purpose
Deploy Arena Vanguard using deployment artifacts rather than certification-only shortcuts.

## Deployment Steps
1. Verify all package hashes and signatures.
2. Unpack `arena-vanguard-deployment.tar.gz` into the deployment directory.
3. Start Node A with the runtime and world packages.
4. Start Node B with the same deployment package and configure TCP reachability to Node A.
5. Confirm federation join, checkpoint sync, replay sync, and recovery readiness.
6. Run the multi-node federation load gate before handing the deployment to operators.

## Verification
Run `bash scripts/run_evernode_deployment_gate.sh --offline --locked` and `bash scripts/run_multinode_federation_load_gate.sh --offline --locked` and archive `deployment/reports/evernode_deployment_gate_run.md`.
