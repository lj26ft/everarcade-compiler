# Task 4C2 State V4 package binding

## Status: INSTALLED

The canonical request and Runtime IR now bind the State V4 policy, Treasury v2 schema, commitment policy, segmentation policy, initial compact state, activation policy, action inventory, and receipt schema inventory. The exact V4/v2 tuple is enforced; V4/v1, V3/v2, and unknown policies reject.

Deterministic rebuild passes. Runtime IR: `sha256:1651fb5bc2c0a0a013fee0e65ac07606b17dd1555f5a7f704024b71c264e6da4`. Package: `sha256:7637c9e8e3c3badff221fac0e0084534700ad8dcd9137498bb5bbf2969f1d28e`. A changed initial Treasury state produces Runtime IR `sha256:f278a74e26d0520fdad073f12c63518bd47656f02a2575366a136868a70a9806` and package `sha256:1b079bdc2fa65c5894a388e1b50a84b7d795889f2b8ac4bf8794e62b0d562aae`.

Validation: `npm run test:state-v4-package` and `cargo check --manifest-path src-bin-everarcade/Cargo.toml`.
