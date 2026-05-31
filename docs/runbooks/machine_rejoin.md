# Machine Rejoin Runbook

## Purpose

Rejoin a machine after network loss or planned maintenance.

## Procedure

1. Verify the machine was isolated and did not continue as an unauthorized authority.
2. Start a TCP resume session with the current authority.
3. Send the last known replay cursor and continuity root.
4. Accept missing replay chunks from the authority.
5. Accept the latest checkpoint if the local checkpoint is stale.
6. Recompute world, replay, checkpoint, and continuity roots.
7. Rejoin as observer until authority assignment is explicitly changed.

## Failure Handling

If roots do not converge, mark the machine stale, preserve its replay for audit, and perform checkpoint restore instead of rejoin.
