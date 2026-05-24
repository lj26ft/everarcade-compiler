# Receipt Canonicalization

`receipt_hash` is defined as:

`sha256(canonical_receipt_bytes)`

Where canonical receipt bytes are JSON bytes from deterministic field order serialization of
`DeterministicExecutionReceipt`.

Invariants:
- no debug/string formatting inputs
- no map iteration dependent ordering
- no platform newline influence
- hash input is bytes only, not display text.

Checkpoint lineage hash input is:

`prior_checkpoint_hash + previous_state_root + new_state_root + execution_receipt_hash + mutation_hash + module_hash`

serialized canonically before hashing.
