# Machine Recovery Runbook

## Purpose

Recover a failed machine from a surviving Arena Vanguard peer.

## Procedure

1. Confirm the failed process is stopped and will not write to its old runtime root.
2. Preserve the failed machine logs for audit.
3. Start the replacement runtime with the same machine identity and a clean process.
4. Request the latest checkpoint from the surviving machine over TCP.
5. Request replay records after the checkpoint replay tip.
6. Restore state from the transferred checkpoint.
7. Replay transferred records until continuity root matches the surviving machine.
8. Keep the recovered machine non-authoritative until convergence is confirmed.

## Success Criteria

- The recovered checkpoint hash equals the source checkpoint hash.
- Replay hash and lineage match the survivor.
- Continuity root matches the survivor.
- Recovery time and transfer counts are written to the recovery report.
