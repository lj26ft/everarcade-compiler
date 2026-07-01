# Risks

## Scope risks

- Commercial platform logic could leak into the open-source reference repo.
- Hosted verification, paid certification, public badge programs, registry operations, trust provider coordination, and launch workflows do not belong here.

## Technical risks

- Multiple proof harnesses may obscure the canonical verifier path.
- Scaffold-level domains may be interpreted as production claims.
- Generated proof reports or runtime outputs may be committed accidentally.
- Existing broad repository surface can make the minimal deterministic reference harder to understand.

## Mitigations

- Keep `world.evr` Core/Extended conformance small and explicit.
- Label scaffold runtime domains clearly.
- Enforce PR-safety rules for generated artifacts, lockfiles, archives, binaries, dependency folders, and screenshots.
- Move product, trust, operator, and commercial coordination responsibilities to the appropriate commercial/internal repositories.
