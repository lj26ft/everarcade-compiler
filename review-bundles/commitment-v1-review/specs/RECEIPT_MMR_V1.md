# Receipt MMR V1

## Overview

`world.evr.receipt_mmr.v1` defines the default append-only receipt history commitment for commitment profile `world.evr.commitment.v1`. It provides O(log n) append, deterministic root derivation, deterministic proof serialization, inclusion proofs for historical receipts, and cross-language verifier compatibility using SHA-256 only.

## Hash encoding and domain separation

All hash inputs use raw bytes. JSON serializations encode hashes as lowercase hex strings representing exactly 32 bytes. Domain tags are ASCII bytes with no length prefix.

### Receipt leaf hash

```text
receipt_leaf_hash =
  SHA256(
    "world.evr.receipt.leaf.v1" ||
    canonical_receipt_bytes
  )
```

`canonical_receipt_bytes` MUST bind at least:

- `world_id`
- `tick`
- `sequence`
- `action_hash` or `input_hash`
- resulting `state_root`
- receipt payload
- protocol version
- commitment profile

The conformance vectors in `test-vectors/commitments/receipt-mmr-v1.json` use canonical JSON bytes: UTF-8 JSON with sorted object keys and no insignificant whitespace. Production profiles MAY define another canonical receipt encoding, but the encoded bytes MUST be unambiguous and included in the profile.

### Internal node hash

```text
receipt_node_hash =
  SHA256(
    "world.evr.receipt.node.v1" ||
    left_child_hash ||
    right_child_hash
  )
```

Both child hashes are raw 32-byte hashes.

### Empty receipt root

```text
empty_receipt_root = SHA256("world.evr.receipt.empty.v1")
```

No implementation may invent another empty root.

### MMR root hash

For non-empty logs:

```text
receipt_root =
  SHA256(
    "world.evr.receipt.root.v1" ||
    mmr_size ||
    peak_hash_1 ||
    peak_hash_2 ||
    ...
  )
```

`mmr_size` is the receipt count encoded as an unsigned 64-bit big-endian integer. Peaks MUST be ordered from largest/oldest to smallest/newest. If two peaks have the same height, the older peak appears first; a valid binary decomposition of a receipt count has at most one peak per height.

## MMR construction

Receipts are appended at zero-based indices. A leaf starts as a peak of height 0. While the two newest peaks have the same height, they are merged into one parent using `receipt_node_hash(left, right)`, where `left` is the older peak and `right` is the newer peak. The resulting parent has height `height + 1` and becomes the newest peak. This process repeats until the newest peak height is unique.

The root for zero receipts is `empty_receipt_root`. The root for non-zero receipts is the MMR root hash over the ordered peaks and receipt count.

## Inclusion proof

A receipt inclusion proof contains:

- the receipt index;
- the receipt count used to compute the root;
- the leaf hash;
- an ordered list of sibling hashes for rebuilding the containing peak from the leaf;
- sibling positions relative to the running hash (`left` or `right`);
- all ordered peaks for the MMR root;
- the claimed `receipt_root`.

A verifier rebuilds the peak hash from the leaf and siblings, replaces the matching peak in the provided ordered peak list, recomputes the MMR root, and compares it with the claimed `receipt_root`. Proof serialization is specified in [Proof Format V1](./PROOF_FORMAT_V1.md).
