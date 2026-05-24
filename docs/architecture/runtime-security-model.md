# Runtime Security Model

Security validation must never introduce nondeterministic runtime behavior.

Fault isolation exists to preserve replay continuity and runtime survivability,
not to implement cloud sandbox orchestration.

## Deterministic threat model
- Inputs may be malformed, reordered, truncated, duplicated, or forged.
- Same input bytes must produce same rejection and diagnostics.

## Trust boundaries
- Checkpoints are trusted only when lineage + root validation pass.
- Archive and restoration manifests are untrusted until validated.
- Federation peers are untrusted by default and can be quarantined deterministically.

## Runtime assumptions
- WASM executes under deterministic fuel and memory ceilings.
- Scheduler ordering is deterministic and abuse is rejection-safe.
- Crash/restart restores to equivalent continuity state or refuses restore.
