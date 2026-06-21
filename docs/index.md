# EverArcade Compiler Documentation

EverArcade compiler documentation is focused on implementation-critical material for deterministic runtime, proof/oracle specs, world packaging, CLI, registry primitives, capability marketplace primitives, treasury execution, and runtime verification.

Public websites, `docs.everarcade.games`, `vision.everarcade.games`, and the world portal live in `everarcade-frontend`. See [Public Frontend Surface](./public-frontend-surface.md).

## Start here

- [Documentation README](./README.md) — canonical compiler documentation order.
- [First World](./first-world.md) — local world creation and validation flow.
- [Runtime capabilities](./runtime-capabilities.md) — maturity and capability overview.
- [Proof mapping](./proofs/proof-mapping-v1.md) — proof/oracle documentation map.
- [World package spec](./world-package-spec-v1.md) — package format and verification boundary.
- [CLI quickstart](./CLI_QUICKSTART.md) — compiler and SDK command surface.
- [Capability Marketplace](./capability-marketplace.md) — compiler-owned marketplace primitives.
- [Treasury](./treasury.md) — compiler-owned treasury execution model.

## Public frontend handoff

Use `public-surface-export/` for the migration bundle intended for `everarcade-frontend`. This repository should link outward to public sites instead of owning Docusaurus, Vercel, vision-site, or portal presentation code.
