# Verification Overview

This is the verifier-facing story for EverArcade v0.1. Verification asks whether the artifact, declared rules, deterministic execution, and continuity evidence agree.

## World Contracts

A World Contract declares active mutations, authorities, state domains, invariants, and RustRig references. Verifiers use it to compare intent against package contents and runtime evidence.

## RustRig invariants

RustRig invariants describe what must remain true for reusable gameplay mutations. A combat RustRig might preserve deterministic damage and health bounds; a market RustRig might preserve integer accounting and receipt determinism.

## Certification

Certification binds package contents, contract declarations, RustRig references, genesis state, proof mappings, replay results, restore results, migration evidence, and root integrity into a reviewable status.

## Replay

Replay re-executes ordered inputs from known genesis or checkpoint state and compares resulting state roots, receipts, journals, and continuity roots with the expected evidence.

## Restore

Restore proves that a world can resume from a checkpoint without losing continuity. The restored state must match declared checkpoint roots and continue producing deterministic evidence.

## Migration

Migration proves that a world can move from one package or version to another while preserving declared lineage, compatible state roots, and migration reports.

## Independent verification

Independent verification means a reviewer can inspect the World Package and evidence without relying on private operator state. The verifier should be able to repeat structural checks and reason about replay, restore, and migration claims.

## Artifact change rule

```text
Modify Artifact
↓
Certification Invalidated
↓
Re-certification Required
```

If a package, contract, RustRig, genesis file, continuity policy, proof mapping, or certification artifact changes, previous certification no longer applies to the modified artifact.
