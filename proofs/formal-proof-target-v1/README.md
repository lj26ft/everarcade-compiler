# EverArcade Formal Proof Target Package v1

Status: **proof-ready**  
Primary target: **HugeGreenCandle proof work**

## Package purpose

This package is the first official EverArcade formal proof target. It is a self-contained verification artifact for formal verification partners, auditors, proof-system developers, and future protocol certification work. It contains the canonical specification, compact ArenaState model, proof targets, canonical fixtures, certification evidence, and a standalone canonicalizer kernel snapshot.

The package is designed so a verifier can reason about the canonicalization boundary without the full EverArcade runtime, HotPocket contract, federation stack, deployment scripts, renderer, or operator workflows.

## Verification boundary

The protocol verification surface is:

```text
ArenaState
  -> Canonical Bytes
  -> State Root
  -> World Hash
```

The boundary proves that the live Arena/HotPocket implementation, isolated proof kernel, and canonical specification converge on the same invariant: equal valid ArenaState values produce identical canonical bytes, identical state roots, and identical world hashes.

## Certification ladder

1. **Replay Certification: PASS** — replayed state converges to the same canonical bytes and roots as live state.
2. **Federation Certification: PASS** — independent nodes converge on matching roots and world hashes.
3. **Restore Certification: PASS** — restored checkpoints preserve canonical bytes, roots, and continuity commitments.
4. **Migration Certification: PASS** — migrated state preserves or explicitly commits continuity and canonical root equivalence.
5. **Root Integrity Certification: PASS** — fixture and report evidence reproduce expected canonical bytes, state roots, and world hashes.
6. **JS ↔ Kernel Equivalence Certification: PASS** — JavaScript reference logic and kernel snapshot agree on canonical bytes, state roots, and world hashes.

## Proof targets

The formal targets are defined in `proof-targets.md`:

* Property 1: Canonicalizer Determinism
* Property 2: State Root Integrity
* Property 3: World Hash Integrity
* Property 4: Replay Equivalence
* Property 5: Restore Equivalence
* Property 6: Migration Equivalence
* Property 7: JS ↔ Kernel Equivalence

## Current status

| Certification | Status |
| --- | --- |
| Replay Certification | PASS |
| Federation Certification | PASS |
| Restore Certification | PASS |
| Migration Certification | PASS |
| Root Integrity Certification | PASS |
| JS ↔ Kernel Equivalence Certification | PASS |

## Contents

* `canonicalizer-spec.md` — normative canonicalization specification.
* `arena-state-model.md` — compact state model, required/optional fields, ordering, and consensus-safe value types.
* `utf8-ordering-fixture.md` — byte-lexicographic UTF-8 ordering fixture; not locale, ICU, or runtime-default ordering.
* `canonical-fixtures/` — fixture states, canonical bytes, and expected roots.
* `root-integrity/` — root-integrity evidence.
* `js-kernel-equivalence/` — JS ↔ kernel equivalence evidence.
* `certifications/` — certification summaries for replay, federation, restore, migration, root integrity, and JS ↔ kernel equivalence.
* `kernel/` — isolated canonicalizer kernel snapshot only: ArenaState types, `canonicalize()`, `state_root()`, `world_hash()`, fixture tests, and UTF-8 ordering tests.
