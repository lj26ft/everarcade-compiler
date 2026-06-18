# Federation Certification Summary

## Node equivalence checks

Federation certification compares independently produced node outputs by canonical bytes and roots, not by host-native object layouts. Nodes must agree on `state_root`, `receipt_root`, `continuity_root`, and `world_hash` for the same certified tick.

## Replay validation

Each node's certified state must be replay-verifiable from the same genesis/checkpoint and input commitment. Replay validation is evidence that node agreement is caused by deterministic execution rather than accidental root copying.

## Root matching

Federation is `PASS` when all certified nodes produce matching canonical bytes or matching `state_root` values and matching receipt/continuity commitments for the comparison window.
