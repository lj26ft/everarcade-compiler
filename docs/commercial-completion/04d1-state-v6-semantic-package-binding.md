# Task 4D.1 — State V6 semantic package binding

Status: **PARTIAL**.

The canonical request and Runtime IR now carry the State V6/Treasury v4 semantic-policy fields, governance authorization policy, and trusted receipt issuer policy. Compilation accepts only the exact V6 tuple and rejects semantic declarations under earlier State policies. The semantic aggregate and authorization commitments are pinned to the Task 4D.0 values.

The deterministic fixture produced identical `world.evr` bytes on two builds. Its current deterministic identities are:

- Runtime IR: `sha256:29cde6d7d68db54dd6220d127294ceb3c31e5ac497ebe249f13470e53a8444ca`
- package: `sha256:40c067551becc1bb38a7757ce44346937aa256f60af7abcc15ab564df1e3ade2`

Mutation of the semantic-policy commitment is rejected before package construction. Historical State V5 deterministic package fixtures remain byte-identical (`sha256:e11c0f860b0997d026ba6d9171b07d0c60f2ed73745d08bc723bed52ac763e88`). Broader per-field mutation coverage and a repository-native compiler-only V6 fixture remain required.
