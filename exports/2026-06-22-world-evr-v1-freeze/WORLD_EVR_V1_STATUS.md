# WORLD_EVR V1 Status

## Frozen Spec

`WORLD_EVR_PACKAGE_SPEC_V1` is frozen as the canonical package-artifact contract for V1. Implementations must conform to the spec, not the other way around.

## Verification Outcome

### Python verifier

Status: current verifier surface for CLI-style package validation.

### TypeScript verifier

Status: current embeddable verifier surface for portal or application integration.

### Cross-implementation agreement

Status: required V1 trust condition. Python and TypeScript verifiers must agree for canonical V1 artifacts.

### Adversarial repair pass

Status: completed for the V1 verifier surface. Known disagreements discovered during repair are treated as spec or implementation defects that must be resolved before public trust claims.

## Trust Model

- Spec: frozen V1 package rules.
- Artifact: concrete `world.evr` package instance and its canonical bytes / manifest / hashes.
- Independent Verifiers: Python and TypeScript implementations that agree on V1 validity.

## Future Rules

Any implementation disagreement against V1 becomes a spec defect until resolved by a recorded V1 erratum or future version process.

## Current Verifier Surfaces

- Python CLI verifier.
- TypeScript embeddable verifier for portal and application flows.
