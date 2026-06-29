# Verify Bundle

Run all commands from the repository root after extracting this bundle.

## 1. Inspect Included Artifacts

```sh
find review-bundles/commitment-v1-review -type f | sort
```

## 2. Run Reference Implementation

The reference implementation is a Python 3 script:

```sh
python3 review-bundles/commitment-v1-review/reference/receipt_mmr_v1/receipt_mmr_v1.py
```

By default, the script writes generated vectors to `test-vectors/commitments/receipt-mmr-v1.json` relative to the repository root. To avoid modifying repository files, run it in a disposable clone or copy the script and compare its generated JSON to the bundled vector file.

## 3. Verify Conformance Vectors

Regenerate vectors with the reference implementation, then compare:

```sh
python3 review-bundles/commitment-v1-review/reference/receipt_mmr_v1/receipt_mmr_v1.py
cmp test-vectors/commitments/receipt-mmr-v1.json review-bundles/commitment-v1-review/test-vectors/receipt-mmr-v1.json
```

## 4. Regenerate Roots

For each positive vector:

1. Canonically serialize each receipt using sorted JSON keys and no whitespace.
2. Compute each leaf as `SHA256("world.evr.receipt.leaf.v1" || receipt_bytes)`.
3. Append leaves to the MMR.
4. Compute internal nodes as `SHA256("world.evr.receipt.node.v1" || left || right)`.
5. Compute the receipt root as `SHA256("world.evr.receipt.root.v1" || uint64_be(receipt_count) || peak_hashes)`.
6. For the empty log, verify the root is `SHA256("world.evr.receipt.empty.v1")`.

## 5. Regenerate Proofs

For each proof:

1. Recompute the leaf hash from the supplied canonical receipt bytes.
2. Apply each sibling hash in order using its `left` or `right` position.
3. Confirm the resulting peak appears exactly once in the ordered peak list.
4. Recompute the receipt root from the ordered peaks and receipt count.
5. Compare the recomputed root to `receipt_root`.

## 6. Compare Outputs

All regenerated roots, peaks, leaf hashes, proofs, and negative-case rejections should match the bundled conformance vectors. Any divergence should be treated as either an implementation bug, a vector bug, or an ambiguity in the specification requiring review.
## 7. Inspect Phase II Continuum Benchmarking

Phase II Continuum benchmark reports are bundled under:

```sh
find review-bundles/commitment-v1-review/reports/phase-ii -type f | sort
```

Review these reports for coverage of real hardware limits, CPU saturation, memory saturation, disk I/O saturation, replay interval cost, determinism repeatability, GPU exploration, and catastrophe/adversarial behavior. Raw local artifacts are not committed; reruns should write raw outputs under `.everarcade-continuum-phase-ii-review/artifacts/`.

## 8. Regenerate Local Archives

Distribution archives are not committed to the repository. After verifying the bundle contents, regenerate local archive artifacts with:

```sh
review-bundles/commitment-v1-review/PACKAGE_BUNDLE.sh
```

Then optionally validate them with:

```sh
tar -tzf review-bundles/commitment-v1-review.tar.gz >/tmp/commitment-v1-review.tar.list
unzip -tq review-bundles/commitment-v1-review.zip
```

