# Gameplay Recovery


- Gameplay execution continuity: preserved by deterministic runtime ticks.
- Scheduler status: authoritative ordering is monotonic and recoverable.
- Session continuity: sovereign session roots are stable and restored from checkpoints.
- Multiplayer synchronization: player inputs are sorted deterministically by frame.
- Replay continuity: append-only windows/checkpoints are authority-produced artifacts.
- Observer hydration state: reconstruction-only; authority writes are rejected.
- Recovery readiness: checkpoints, sessions, replay continuity, and coordination can be restored.
- Operational limitations: renderer/history/federation remain scaffold-level runtime domains; persistence is in-memory for validation.
