# Receipt Canonicalization

`receipt_hash` is:

`sha256(canonical_receipt_bytes)`

Where canonical receipt bytes are deterministic JSON bytes of `DeterministicExecutionReceipt` field order.

Invariants:
- no debug/string formatting inputs
- no map iteration dependent ordering
- no platform newline influence
- hash input is bytes only, not display text

Checkpoint lineage hash input is:

`prior_checkpoint_hash + previous_state_root + new_state_root + execution_receipt_hash + mutation_hash + module_hash`

serialized canonically before hashing.

## Checkpoint lineage semantics

- Genesis executions use canonical prior hash constant:
  `genesis:0000000000000000000000000000000000000000000000000000000000000000`.
- Non-genesis executions must pass `prior_checkpoint_hash` explicitly via
  `execute_contract_with_checkpoint`.
- Replay equivalence requires stable receipt bytes, status, mutation hash, stdout hash, fuel used,
  new root, checkpoint hash, and continuity proof hash.
