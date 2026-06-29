# Benchmark Receipt Root MMR

Benchmarking is a follow-up for the runtime/performance repo, likely `everarcade`, not the primary deliverable of `everarcade-compiler`.

Compare:

- current receipt-root method;
- full rebuild Merkle root;
- incremental MMR append;
- MMR proof generation;
- MMR proof verification.

Use receipt counts: `1`, `10`, `100`, `1,000`, `10,000`, and `100,000`.

Report wall-clock time, allocations, peak memory, proof size, and root/proof determinism. Benchmarks must use the domain tags and root ordering from `docs/specs/RECEIPT_MMR_V1.md`.
