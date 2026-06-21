# EverArcade v0.1 Recovery Runbook

## Purpose
Recover a v0.1 release-candidate runtime while preserving checkpoint lineage and replay continuity.

## Preconditions
- Checkpoint records exist.
- Replay records exist.
- `release/recovery-certification/RECOVERY_ROOT` is present.

## Procedure
1. Stop affected runtime services.
2. Preserve current logs and receipt ranges.
3. Restore the last valid checkpoint.
4. Replay records from the checkpoint boundary.
5. Compare replay roots with the recorded continuity root.
6. Restart services only after continuity passes.

## PASS Criteria
- Restored checkpoint loads successfully.
- Replay root matches expected lineage.
- Settlement and federation records remain auditable.

## FAIL Criteria
- Missing checkpoint or replay range.
- Root mismatch after replay.
- Recovery requires changing frozen interfaces.
