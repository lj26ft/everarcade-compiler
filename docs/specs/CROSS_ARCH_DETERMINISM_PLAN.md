# Cross-Architecture Determinism Plan

Goal: the same world, same inputs, and same protocol versions produce the same `state_root`, `receipt_root`, `continuity_root`, and `world_hash` on x86 Linux, ARM Linux, macOS ARM, and Windows x86.

## Platforms

| Platform | Required runner | Status |
| --- | --- | --- |
| x86 Linux | glibc Linux host or CI runner | pending independent run |
| ARM Linux | aarch64 Linux host or CI runner | pending independent run |
| macOS ARM | Apple Silicon macOS runner | pending independent run |
| Windows x86 | Windows x86_64 runner | pending independent run |

## Procedure

1. Check out the same commit.
2. Use the fixture in `fixtures/determinism/` without editing JSON.
3. Run the verifier command from `fixtures/determinism/VERIFY.md`.
4. Record Node/Rust/toolchain versions, CPU architecture, OS, and root output.
5. Compare `state_root`, `receipt_root`, `continuity_root`, and `world_hash` byte-for-byte against `expected-roots.json`.
6. Treat any mismatch as a determinism failure until root-caused.

## Controls

- Disable host randomness, wall-clock input, locale-sensitive sorting, and unordered map iteration in root derivation.
- Use integer or fixed-point arithmetic only for committed state.
- Keep renderer, history, and federation domains scaffold-level unless separately certified.
- Preserve raw failing inputs and outputs as artifacts; commit only curated summaries.

## Exit criteria

Cross-hardware determinism may be claimed only after all target platforms reproduce the same roots from the same fixture and at least one independent party rederives every root from the public specification.
