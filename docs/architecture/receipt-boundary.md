# Receipt Boundary

- Receipt construction and verification rules are defined by `execution-core`.
- Host/runtime owns receipt storage, indexing, replication, and retrieval.
- Receipt hash guarantees depend on canonical serialization and deterministic ordering of included elements.
- Replay implications: any receipt hash mismatch for identical deterministic inputs indicates invalid divergence.
