# Repository Readiness Report

## Repository Role

`everarcade-compiler` is the open-source reference implementation for deterministic `world.evr` packaging, local verification, proof modeling, runtime primitives, and SDK boundaries. It should remain the smallest understandable proof that a world package can be deterministically built, checked, replayed, and reasoned about.

Key conclusion: **This repo should remain the minimal reference implementation, not the commercial platform.**

## Current State

The repository contains a broad reference surface: world package and certification documentation, deterministic runtime primitives, RustRig contract APIs, SDK modules, local proof harnesses, runtime proof reports, deployment proof models, and a deterministic world-factory CI workflow.

The reference implementation is strongest where it models deterministic inputs, canonical serialization, replay equivalence, package validation, root integrity, and local proof reports. It is weaker where the tree contains platform-shaped scaffold domains that look larger than a minimal compiler/reference repo.

## Completed Work

- Deterministic RustRig and ABI primitives exist in `contract-api`, including deterministic execution metadata and append-only replay-safe ABI policy.
- SDK helpers cover deterministic manifest checks, package byte equivalence, deterministic input ordering, replay records, deterministic state hashing, and nondeterminism detection examples.
- `docs/world-package-certification-framework-v1.md` defines a deterministic evidence pipeline for `world.evr` packages and describes operator verification commands.
- Local proof harnesses exist for gameplay, HotPocket-style runtime integration, migration, continuity anchoring, XRPL/Xahau publication modeling, and root integrity.
- A GitHub Actions workflow exists for deterministic world-factory validation.

## Partial Work

- `world.evr` Core/Extended boundaries are documented and modeled, but the repository still mixes core proof material with many advanced runtime, deployment, federation, marketplace, and public-platform concepts.
- Verifier behavior exists as documentation, SDK helpers, proof scripts, and reports; it is not yet presented as one minimal, canonical verifier contract with stable CLI guarantees.
- Certification behavior is locally modeled, but hosted verification services, badge programs, reviewer marketplaces, application workflows, and paid certification products are explicitly outside this reference repository.
- Federation, renderer/history, GPU marketplace, Evernode deployment, and transport paths should be treated as scaffold-level runtime domains rather than production claims.

## Missing Work

- A single concise reference boundary document that states what belongs in this repository and what must remain in commercial/platform repositories.
- A frozen `world.evr` Core/Extended conformance matrix tied to concrete files, commands, fixtures, and expected outputs.
- A minimal verifier CLI contract that separates required Core checks from optional Extended/proof checks.
- CI labeling that distinguishes production-grade gates from scaffold/proof-model gates.
- Repo hygiene to keep generated proof outputs and large/generated artifacts from obscuring the small reference implementation.

## CI / Verification Status

Visible CI includes `.github/workflows/deterministic-world-factory.yml`, which runs `bash scripts/ci/run-deterministic-world-factory.sh`. Repository tests and proof scripts indicate targeted validation paths for SDK/runtime behavior, deterministic replay, package determinism, root integrity, and proof reports.

Per user instruction, this audit did **not** run `cargo test --workspace`. Validation for this change used repository inspection and a targeted audit-bundle safety check only.

## Risks

- Scope creep: commercial coordination, hosted certification, registry operations, trust-provider workflows, and marketplace economics could accidentally migrate into the reference repo.
- Signal dilution: scaffold domains may be mistaken for production runtime guarantees unless explicitly labeled.
- Verifier ambiguity: multiple proof harnesses can make it unclear which checks are required for minimal `world.evr` conformance.
- Artifact risk: proof and runtime directories can tempt contributors to commit binaries, archives, WASM, generated reports, or dependency trees.

## Dependencies

- Commercial platform repo (`everarcade`) should own coordination, platform trust, certification workflows, registry services, commercial boundaries, and product implementation.
- `everarcade-rustrigs` should own deterministic RustRig hardening and reusable gameplay rig certification.
- `everarcade-frontend` should consume public registry/proof metadata rather than defining protocol truth.
- `everarcade-hq` should coordinate internal operations and review without becoming the protocol source of truth.

## Recommended Next Actions

1. Freeze and publish a concise `world.evr` Core/Extended matrix.
2. Add a reference-boundary document that keeps this repository focused on minimal deterministic proof.
3. Promote one canonical verifier path for Core package checks.
4. Mark renderer/history/federation/GPU/deployment domains as scaffold/partial wherever reports could be misread as production readiness.
5. Keep commercial registry, hosted certification, trust-provider coordination, launch workflows, and product UX out of this repository.

## Readiness Score

**3 = functional internal**

The repository has functional deterministic reference material and CI/proof scaffolding, but it is not yet a clean public-beta reference boundary because Core/Extended conformance, verifier behavior, and scaffold-vs-production labels need sharper separation.
