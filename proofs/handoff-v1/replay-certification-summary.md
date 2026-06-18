# Replay Certification Summary

## Methodology

Replay certification treats the recorded input stream and genesis/checkpoint state as deterministic inputs. A replay engine reconstructs the same `ArenaState` sequence and compares canonical bytes or state roots at selected checkpoints and at the terminal tick.

## Artifacts

* `canonicalizer-spec.md`: normative byte and root specification.
* `canonical-fixtures/`: fixture states, canonical byte hex, and expected roots.
* `proof-targets.md`: formal replay equivalence property.

## Pass criteria

Replay is `PASS` when replayed state at the certified tick produces byte-identical canonical bytes to the live state and therefore the same `state_root`, receipt commitments, continuity commitments, and `world_hash`.
