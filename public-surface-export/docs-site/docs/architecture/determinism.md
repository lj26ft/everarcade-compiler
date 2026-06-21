# Arena Vanguard Determinism

Arena Vanguard is the deterministic reference world for EverArcade runtime development. Consensus-critical code must produce the same state, receipts, world hash, and continuity roots for the same ordered inputs on every node.

## Allowed

- Canonical hashing over canonical JSON bytes.
- Sorted-key serialization for objects before hashing.
- Deterministic state mutations driven only by validated input envelopes and prior state.
- Replay verification that recomputes commitments from journaled canonical actions.
- Persisted-state rebuilds that preserve existing commitments.

## Forbidden in Consensus Paths

- `Date.now()`
- `new Date()`
- `Math.random()`
- `randomUUID()`
- `fetch()`
- `axios`
- `setTimeout()`
- `setInterval()`
- Locale-dependent formatting
- Host environment dependencies such as `process.env`, `process.cwd()`, and `process.argv`
- Floating-point consensus logic

## Runtime Configuration Boundary

Host configuration may select ports, CLI arguments, and file locations before inputs enter the runtime. Those values must not be included in state roots, receipt roots, world hashes, continuity roots, journals, proof bundles, or replayed action payloads.
