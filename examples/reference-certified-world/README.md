# Reference Certified World

This directory is the canonical World Package Certification Framework v1 example. It extends the reference `world.evr` package layout with certification artifacts for a package that reaches Level 8: World Package Certified.

## Contents

- `manifest/world-manifest.toml` — reference world package manifest.
- `world-contract/world-contract.toml` — reference world contract declarations.
- `rustrigs/` — Inventory, Combat, Market, and Governance RustRig manifests.
- `genesis/` — genesis `ArenaState`, initial receipts, continuity state, and initial roots.
- `continuity/` — replay, migration, restore, retention, and epoch policy.
- `assets/` — data-only asset metadata.
- `proofs/` — package proof mappings and proof reports.
- `certification/` — CLASS-A through CLASS-J reports and final certification report.

## Certification Result

```text
WORLD PACKAGE CERTIFICATION: PASS
```

Use this directory as the reference certified package when implementing `everarcade certify-world`, operator verification, migration validation, replay validation, and proof mapping checks.
