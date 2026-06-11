# HotPocket Adapter Template

This template wires a creator contract entrypoint to the canonical EverArcade HotPocket adapter.

Supported proof actions:

- `{"action":"ping"}` returns `{"status":"ok"}`.
- `{"action":"join_player","player_id":"player-1","nonce":"unique"}` increments deterministic `player_count` and emits receipt, journal, checkpoint, and replay proof artifacts.

Use `scripts/run_hotpocket_package_proof.sh` from the repository root to stage the reproducible package bundle.
