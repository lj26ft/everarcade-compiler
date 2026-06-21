# Production Replay Oracle Input Pack v1

This directory is the production-root fixture pack for an independent ArenaState replay oracle.

Dane should independently re-execute each fixture as:

```text
genesis + journal entries -> final ArenaState -> canonical bytes -> SHA-256 state_root/world_hash
```

No shared implementation code is required for verification. The replay oracle only needs the transition rules in `TRANSITION_SPEC.md`, the genesis fixture for a journal, and the ordered `entries[].action` values from the matching journal file.

## Directory layout

- `genesis/`: exact initial ArenaState for every journal fixture.
- `journals/`: production journal entries emitted by `ArenaHotPocketRuntime.process`, plus rejected input records where applicable.
- `expected-roots/`: live roots to compare after replay: `state_root`, `receipt_root`, `continuity_root`, `world_hash`, `tick`, and `journal_hash`.
- `live-runtime-evidence.md`: commands and runtime path used to generate the pack.

## Verification procedure

1. Load `genesis/journal-NNN-*.genesis.json` and take its `state` object as the starting ArenaState.
2. Load `journals/journal-NNN-*.json` and replay each `entries[].action` in order using the transition spec.
3. Ignore `rejected_inputs` for state mutation; confirm they do not produce receipt or journal entries.
4. Canonicalize the final full ArenaState and hash it with SHA-256 to get `state_root`.
5. Canonicalize `{ tick, players, combat_events }` and hash it with SHA-256 to get `world_hash`.
6. Compare those values with the matching file under `expected-roots/`.
7. Optionally reconstruct receipts and journal hashes to compare `receipt_root`, `continuity_root`, and `journal_hash`.
