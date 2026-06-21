# EverArcade v0.1 Upgrade Runbook

## Purpose
Upgrade a release-candidate installation without changing v0.1 interfaces or losing deterministic replay continuity.

## Preconditions
- `release/upgrade-certification/UPGRADE_ROOT` is present.
- Current runtime has a fresh checkpoint.
- Replay records and receipts are preserved.

## Procedure
1. Record current version, checkpoint root, and replay root.
2. Stop runtime services.
3. Install the release-candidate package.
4. Restart services with the frozen CLI and layout.
5. Replay from the recorded checkpoint boundary.
6. Verify upgraded replay and recovery roots.

## PASS Criteria
- Install, upgrade, restart, replay, and recovery all pass.
- No command names, required arguments, or layouts change.
- Upgraded node rejoins federation with matching continuity evidence.
