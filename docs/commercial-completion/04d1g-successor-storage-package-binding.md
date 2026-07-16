# Task 4D.1G successor package binding

The compiler recognizes and validates the exact State V8 tuple:

- `everarcade.ptw-state-components.v8`
- `everarcade.ptw-treasury-state.v6`
- `everarcade.ptw-treasury-commitment.v6`
- `everarcade.ptw-treasury-segmented-storage.v2`
- `everarcade.ptw-treasury-frontier-checkpoint.v1`
- `everarcade.ptw-treasury-storage-v1-to-v2-migration.v1`

The Treasury genesis object additionally binds storage-page, directory, index-page, record-reference, and semantic policy identifiers. Deterministic rebuild passes; mutations of storage, checkpoint, and migration policy fields reject. State V7 packages remain unchanged and pass their historical fixture.
