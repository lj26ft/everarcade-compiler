# Arena Vanguard Reference Certified World v1 Operator Guide

This package is self-contained. Operators do not need to read source code.

1. Build or download `world.evr`; when working from this repository, run `./operator/build-world-evr.sh .`.
2. If you received an archive, extract it: `tar -xzf world.evr`.
3. Verify package and certifications: `./operator/verify.sh .`.
4. Deploy federation using `continuity/policies.toml`.
5. Replay from `genesis/initial-receipts.json`.
6. Migrate using the certified migration policy.
7. Restore from `genesis/continuity-state.json`.
8. Compare roots in `genesis/world-roots.toml`; the certified world hash is `sha256:ef5409866bb75211145a0da901611621c57237bc79dad0c0c2cdde1dc3873883`.

Expected output: `REFERENCE CERTIFIED WORLD V1: PASS`.
