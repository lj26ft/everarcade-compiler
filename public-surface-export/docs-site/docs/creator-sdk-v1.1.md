# Creator SDK v1.1: First World Experience

Creator SDK v1.1 reduces Time To First World by moving the first visible path to creator-first lifecycle commands:

```bash
everarcade world init --template frontier
everarcade world rustrigs
everarcade world run
everarcade world package
everarcade world verify
everarcade world deploy
```

## What changed

- `everarcade world init` creates a World from a template alias while preserving the existing template implementation.
- `everarcade world templates` and `everarcade world init --list-templates` show the available World choices.
- `everarcade world rustrigs` lists certified and candidate RustRig modules with descriptions.
- `everarcade world verify` runs the existing test, certification, and certificate re-check flow and ends with `WORLD VERIFY: PASS`.
- `everarcade world package` writes `dist/world.evr` as the creator-facing World Package entry point while retaining `dist/runtime-package/` compatibility.
- `everarcade world project` exposes the projection entry point.

## Vocabulary

Creator-facing docs and commands now prefer World, World Package, World Contract, RustRig, Projection, and Deployment. Legacy terms remain where compatibility requires them.

## Start here

Use the one-page quick start: [First World Quick Start](first-world.md).

Use the post-create project map: [World Project Map](creator-sdk/world-project-map.md).
