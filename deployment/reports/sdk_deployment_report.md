# SDK Deployment Report

## Guarantees
- SDK continuity guarantees are enforced through ordered deterministic inputs.
- Replay guarantees preserve append-only reconstruction and reject mutation.
- Deployment guarantees preserve authority boundaries and lineage compatibility.
- Packaging guarantees preserve reproducible artifact hashes.

## Operational limitations
- Renderer/history/federation domains remain scaffold-level runtime domains.

## Developer ergonomics assessment
- Developers use SDK abstractions instead of replay lineage internals or federation plumbing.
