# Verification Status Matrix

This matrix distinguishes observed determinism from proof. Current repository artifacts support local reproduction and specification hardening; cross-hardware proof remains pending.

| Root | Observed | Reproduced | Cross-build verified | Cross-hardware verified | Formally specified |
| --- | --- | --- | --- | --- | --- |
| `state_root` | Yes | Yes, fixture verifier | Yes, debug/release fixture verifier | Pending | Partially; fixture canonicalization documented |
| `receipt_root` | Yes | Yes, fixture verifier | Yes, debug/release fixture verifier | Pending | Partially; Receipt MMR V1 remains authoritative where applicable |
| `continuity_root` | Yes | Yes, fixture verifier | Yes, debug/release fixture verifier | Pending | Yes, `docs/specs/CONTINUITY_ROOT_V1.md` |
| `world_hash` | Yes | Yes, fixture verifier | Yes, debug/release fixture verifier | Pending | Yes, `docs/specs/WORLD_HASH_V1.md` |
| `checkpoint_hash` | Yes | Yes, fixture verifier | Yes, debug/release fixture verifier | Pending | Fixture-level only; full checkpoint spec pending |

## Required language

Use: "determinism observed on current hardware; cross-hardware proof pending."

Use: "deterministic roots are the mechanism for verifying bounded slices once cross-hardware reproduction is established."
