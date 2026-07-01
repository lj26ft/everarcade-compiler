# Repository Inventory

## Repository

- Name: `everarcade-compiler`
- Role: open-source world.evr reference implementation and deterministic proof surface.
- Audit date: 2026-07-01

## High-signal areas

- `docs/` contains world package, certification, SDK, runtime model, deployment, and integration documentation.
- `runtime/everarcade-runtime/` identifies the small runnable deterministic runtime core and explicitly excludes non-authoritative renderer, player gateway, creator template, GPU, XRPL, and Xaman records.
- `contract-api/` contains deterministic ABI and RustRig contract primitives.
- `sdk/` contains deterministic SDK, manifest, package, validation, replay, deployment, and runtime API surfaces.
- `.github/workflows/deterministic-world-factory.yml` runs the deterministic world factory CI script.
- `runtime/*-proof/` directories contain local proof harnesses for HotPocket gameplay/runtime/migration, continuity anchors, XRPL/Xahau anchor modeling, and related evidence.

## Scaffold / non-production domains

Renderer, history/observer, federation, GPU marketplace, transport, deployment automation, and commercial certification services are treated as scaffold-level or partial domains unless a specific production gate says otherwise.

## PR safety

This audit bundle is markdown and json only. It does not add generated binaries, archives, dependency folders, screenshots, local machine paths, secrets, `.env` files, or lockfile changes.
