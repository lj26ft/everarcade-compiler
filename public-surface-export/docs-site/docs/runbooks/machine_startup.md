# Machine Startup Runbook

## Purpose

Start an Arena Vanguard cross-machine runtime without shared process, memory, or filesystem assumptions.

## Preconditions

- Machine A and Machine B have separate runtime roots.
- Machine A and Machine B have separate storage, checkpoint, replay, and log directories.
- Operators know the TCP address that Machine B will listen on and Machine A will dial.

## Procedure

1. Create or verify these directories on each host: `runtime/`, `storage/`, `checkpoints/`, `replay/`, and `logs/`.
2. Start Machine A as the Arena Vanguard authority.
3. Start Machine B with its own runtime root and storage root.
4. Configure Machine B to join by checkpoint and replay transfer over TCP.
5. Confirm process IDs are distinct and runtime roots are not shared.
6. Run the cross-machine certification script before accepting the runtime as operational.

## Validation

- World roots match.
- Replay roots match.
- Checkpoint roots match.
- Continuity roots match.
- No symlink or bind mount points one machine root at the other.
