# EverArcade Tiny Proof Handoff v1

This package is a minimal, self-contained verification handoff for the EverArcade canonicalization boundary, with Arena Vanguard as the reference verification target. It is intended for HugeGreenCandle formal verification work, auditors, researchers, and proof-system developers who need to reason about canonical state bytes and roots without cloning the full EverArcade repository.

## Arena Vanguard overview

Arena Vanguard is the reference arena-style consensus state used by this handoff. The arena contains players, entities, deterministic integer components, receipt commitments, continuity commitments, and consensus metadata. Presentation, rendering, networking, and operator workflow state are intentionally outside this package unless their commitments are already embedded in the canonical ArenaState.

## Canonicalization boundary

The trust boundary is the function that accepts a valid `ArenaState`, validates its schema and deterministic value domain, emits canonical JSON bytes, and hashes those bytes. Any downstream certification artifact is only as meaningful as this boundary.

The normative specification is `canonicalizer-spec.md`. The compact state model is `arena-state-model.md`.

## State → Canonical Bytes → Root model

```text
ArenaState
  -> validate required fields, types, uniqueness, integer-only values, roots, and UTF-8 strings
  -> order schema fields and sort dynamic keys by raw UTF-8 byte lexicographic order
  -> emit whitespace-free canonical JSON
  -> UTF-8 encode canonical JSON
  -> SHA256(canonical_bytes)
  -> state_root
```

World hash composition is:

```text
world_hash = SHA256(state_root_bytes || receipt_root_bytes || continuity_root_bytes)
```

## Root Integrity Certification summary

`root-integrity-report.txt` records the included certification status. The required acceptance line is:

```text
ROOT INTEGRITY CERTIFICATION: PASS
```

The certification checks that every included fixture's canonical byte hex decodes to the expected canonical JSON bytes and that each expected `state_root` equals `SHA256(canonical_bytes)`.

## Scope limitations

This handoff does not prove game transition semantics, renderer behavior, production federation liveness, economic settlement, host deployment security, or wall-clock operational workflows. It only packages the canonicalization boundary and adjacent replay/federation/restore/migration certification summaries needed to begin formal work.

## Verification goals

1. Prove canonicalizer determinism for all valid `ArenaState` values.
2. Prove state-root integrity as SHA-256 over exact canonical bytes.
3. Prove world-hash integrity over binary-decoded state, receipt, and continuity roots.
4. Check replay equivalence against canonical bytes and roots.
5. Check migration/restore equivalence by preserving canonical bytes, roots, and continuity commitments.

## Archive generation

The repository stores only text proof artifacts so pull requests remain reviewable. To recreate the handoff archive locally, run:

```sh
tar -czf proofs/handoff-v1.tar.gz -C proofs handoff-v1
```

The generated archive is intentionally not committed.
