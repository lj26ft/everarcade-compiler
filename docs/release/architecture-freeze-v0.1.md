# EverArcade Architecture Freeze v0.1

## Freeze Decision

EverArcade v0.1 architecture is declared complete and frozen for external validation.

```text
ARCHITECTURE FREEZE V0.1: PASS
```

This is an architecture freeze. It is not a code freeze, bug-fix freeze, documentation freeze, testing freeze, deployment tooling freeze, or verification freeze.

## Purpose

The v0.1 freeze stops introduction of new protocol surfaces so the existing system can be validated by external operators, developers, deployments, and formal verification reviewers.

The frozen architecture should now be exercised through onboarding, deployment, replay, restore, migration, certification, and proof review rather than expanded with new protocol layers.

## Frozen Architecture Scope

The following v0.1 architecture areas are frozen:

- Runtime architecture
- Canonicalizer architecture
- Verification boundary
- World Contract architecture
- RustRig architecture
- World Package architecture
- World Registry architecture
- Certification architecture

## Frozen Runtime Components

Runtime v0.1 is frozen around these components:

- Replay
- Federation
- Restore
- Migration

Renderer, history, and federation-adjacent runtime surfaces remain scaffold-level unless a release report explicitly certifies a narrower behavior.

## Frozen Verification Components

Verification v0.1 is frozen around these components:

- Canonicalizer Spec
- Canonicalizer Kernel
- Root Integrity Certification
- JS ↔ Kernel Equivalence
- Proof Mapping Framework
- Formal Proof Target Package

Formal review should validate this boundary rather than expand it.

## Frozen Protocol Components

Protocol v0.1 is frozen around these components:

- World Contract Framework
- World Package Specification
- World Package Certification Framework
- World Registry Specification

New package formats, registry formats, trust boundaries, or certification frameworks require v0.2 planning.

## Certified RustRigs

The certified v0.1 RustRig set is frozen as:

- `inventory.transfer()`
- `combat.attack()`
- `market.trade()`
- `governance.vote()`

Additional RustRig families are allowed only as v0.2-planned protocol expansion or as non-protocol examples that do not change the certified v0.1 surface.

## Frozen Reference Assets

The frozen v0.1 reference asset set is:

- Reference Certified World v1
- Reference World Registry
- V0.1 Release Certification Gate
- V0.1 Release Notes

## Allowed Changes After Freeze

The following changes remain allowed after the architecture freeze:

- Bug fixes
- Documentation improvements
- Testing improvements
- Operator onboarding improvements
- Developer onboarding improvements
- Build system improvements
- Deployment tooling improvements
- Verification improvements

Allowed changes must preserve the frozen v0.1 protocol surface and trust boundaries.

## Disallowed Changes Without v0.2 Planning

The following changes are disallowed without explicit v0.2 planning:

- New protocol layers
- New certification frameworks
- New package formats
- New registry formats
- New trust boundaries
- Major architecture rewrites

## External Validation Goals

### Operator Validation

Target: 3–5 external operators.

Operators should validate:

- Package verification
- Deployment
- Replay
- Restore
- Migration
- Certification understanding

### Developer Validation

Target: 3–10 external developers.

Developers should validate:

- World Contracts
- RustRig usage
- Package workflow
- Documentation clarity

### Verification Validation

Primary reviewer: HugeGreenCandle.

Formal review should validate:

- Canonicalizer
- Proof package
- Proof mapping
- Certification chain

## Success Criteria

EverArcade v0.1 Architecture Freeze remains complete when:

- Repository is public.
- Reference Certified World deploys successfully.
- External operators validate the deployment flow.
- External developers validate the development flow.
- Formal verification review is completed.
- No critical architectural defects are discovered.

Critical architectural defects discovered during validation should reopen v0.1 architecture only for narrowly scoped corrections or move the change into v0.2 planning.
