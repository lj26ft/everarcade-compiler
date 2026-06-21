# Live Runtime Evidence

The fixtures in this pack were generated from the live Arena HotPocket wrapper runtime path, not from a standalone model harness.

## Runtime path

- Runtime module: `hotpocket-arena-wrapper/src/runtime.mjs`
- Execution entry point: `ArenaHotPocketRuntime.process(input)`
- Transition function: `executeInput -> applyArenaInput -> commitFor`
- Canonical SHA-256 path: `canonicalize(value) -> canonicalHash(value)`

## Generation command

From the repository root:

```bash
node proofs/production-replay-oracle-v1/generate-production-replay-oracle-v1.mjs
```

The generator creates five deterministic live runtime scenarios, replays each emitted journal through `replayJournal`, checks replay state equality against the live runtime state, and writes the genesis, journal, and expected-root files.

## Validation commands

```bash
grep -R "sha256" proofs/production-replay-oracle-v1
grep -R "state_root" proofs/production-replay-oracle-v1
grep -R "world_hash" proofs/production-replay-oracle-v1
git diff --check
```
