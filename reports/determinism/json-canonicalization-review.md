# Arena Vanguard JSON Canonicalization Review

## False Positives

- `canonicalize()` intentionally uses `JSON.stringify()` only for primitive JSON values and object keys after sorting `Object.keys(value)`. This is the canonical byte generator used by `canonicalHash()`.
- `clone()` uses `JSON.parse(JSON.stringify(value))` for deep-copying validated JSON-compatible state. The result is not used as unordered hash input without canonicalization.
- `persist()` and HTTP/CLI/report writers use pretty `JSON.stringify()` for operator-readable files or transport responses. Commitment generation uses canonical hashes, not the pretty-printed byte order.

## Valid Issues Remediated

- `inputId()` previously appended `randomUUID()`. It now returns `arena-${canonicalHash(envelope)}` with no randomness.

## Required Invariant

Any value that contributes to `state_root`, `receipt_root`, `world_hash`, `continuity_root`, journal hashes, or proof comparison must be passed through `canonicalize()`/`canonicalHash()` rather than relying on raw `JSON.stringify()` object insertion order.
