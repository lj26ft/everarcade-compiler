# Checkpoint Restore Runbook

## Purpose

Restore runtime state from a transferred checkpoint.

## Procedure

1. Stop the target runtime process.
2. Copy the checkpoint only through the approved network transfer path.
3. Verify the checkpoint hash before writing it to local storage.
4. Restore world state from the checkpoint payload.
5. Reconstruct replay from the checkpoint replay tip.
6. Recompute the continuity root.
7. Restart the runtime in observer mode.

## Validation

- Checkpoint hash equals the source machine hash.
- Restored world root equals the source world root.
- Replay reconstruction produces the same replay root.
- Runtime logs include the restore timestamp and source machine.
