# Testnet Launch Page Draft

This source document captures the public testnet launch content without creating a generated website artifact.

## What exists today

- A Docusaurus public website with role-based entry points.
- Documentation under `/docs` for runtime, SDK, operator, deployment, and verification details.
- World template and package scaffolds for early experimentation.
- Runtime, federation, renderer, and history domains that should be treated as scaffold-level unless a specific document states otherwise.

## How to participate

1. Read the [Worlds page](src/pages/worlds.md).
2. Choose a role: developer, operator, player, or contributor.
3. Follow the relevant documentation path under `/docs`.
4. Open GitHub issues for gaps, confusing onboarding steps, or broken assumptions.

## How to contribute

- Keep public language world-first and beginner friendly.
- Keep implementation details in `/docs`.
- Prefer small source-only changes.
- Do not commit generated website output, dependency folders, coverage, screenshots, videos, or compiled assets.

## How to run a world

Start with the developer and operator documentation:

- `docs/GAME_DEVELOPER_START.md`
- `docs/08-sdk-development.md`
- `docs/canonical-package-format.md`
- `docs/13-runtime-operations-manual.md`
- `docs/linux-vm-operator-quickstart.md`

## How to provide feedback

Use GitHub issues and discussions as the placeholder feedback channel until a dedicated community workflow exists.

## Launch checklist

- [ ] Confirm all public pages build in Docusaurus.
- [ ] Confirm no generated artifacts are committed.
- [ ] Confirm files over 500 KB are reviewed and excluded if generated.
- [ ] Confirm docs links point to existing implementation references.
- [ ] Confirm launch blockers are captured in the public narrative report.
