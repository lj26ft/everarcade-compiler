# Proof Target: operation.restore

Maturity: CANDIDATE

This target defines the verifier handoff for `operation.restore`. It describes state, input, validation, transition, receipt, replay, and root behavior without claiming formal proof for the full standard library.

## Receipt accumulator requirement

Restore verifiers must derive `receipt_root` from bundle data only. The trusted input is the receipt accumulator:

```text
receipt_accumulator = final_receipt_hashes + last_temp_receipt_hash
receipt_root = canonical_hash([final_receipt_hashes, last_temp_receipt_hash])
```

A stored `roots.receipt_root`, `checkpoint.roots.receipt_root`, or `restored_roots.receipt_root` is comparison evidence only. It must not be treated as trusted truth unless it matches the recomputed accumulator root. This closes the hidden dependency where a verifier could accidentally trust a serialized receipt root instead of deriving it from the provided receipt hashes.

