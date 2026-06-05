# EverArcade v0.1 Operator Runbook

## Purpose
Operate a release-candidate node with stable interfaces, auditable records, and deterministic recovery points.

## Daily Checks
1. Run node status and runtime status commands.
2. Verify checkpoint creation.
3. Verify receipt propagation and replay records.
4. Review governance and federation health.
5. Confirm GPU marketplace and settlement records remain auditable.

## Incident Response
1. Stop new deployments if continuity evidence is missing.
2. Preserve logs, checkpoints, receipts, and replay records.
3. Run the recovery runbook before resuming federation traffic.
4. Escalate if roots do not match the release certification chain.

## PASS Criteria
- Node health is green.
- Checkpoint, replay, settlement, and federation records are present.
- Operator actions follow frozen CLI and layout contracts.
