# State Diff Boundary

- State diff derivation is pure and deterministic in `execution-core` from provided pre/post state records.
- Host owns persistence and distribution of state diff artifacts.
- Deterministic ordering of diff entries is required prior to hashing or receipt embedding.
- Invalid divergence includes missing entries, reordered canonical sequences, or mismatched digest roots during replay.
