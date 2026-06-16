# Technical Overview

This page is the engineer path into EverArcade. It summarizes the moving parts and links to the implementation docs instead of duplicating them.

## World Contracts

World Contracts define the rules that change world state. They are the boundary between a game idea and a verifiable persistent world.

- [World contract architecture](/docs/architecture/diagrams/world-contract-architecture)
- [Contract development](/docs/architecture/sdk/contract-development)
- [ABI boundary](/docs/architecture/abi-boundary)

## Runtime Packages

Runtime packages keep rules, metadata, and operating expectations portable. Packaging is what lets a world move from local development to hosted operation without becoming trapped in one launcher.

- [World packaging](/docs/architecture/world-packaging)
- [Runtime distribution strategy](/docs/release/runtime_distribution_strategy)
- [Release packaging](/docs/release/release-packaging-v0.1)

## Deterministic Replay

EverArcade treats replay as a trust primitive. If a world claims a history, participants should have a path to check the relevant execution window.

- [Replay engine](/docs/architecture/runtime/replay-engine)
- [Replay attack surfaces](/docs/security/replay_attack_surfaces)
- [Historical replay](/docs/architecture/renderer/historical-replay)

## Verification

Verification connects runtime execution, receipts, release integrity, and operator practice. It exists so communities do not have to rely on one server's memory forever.

- [Receipt boundary](/docs/architecture/receipt-boundary)
- [Receipt system](/docs/architecture/runtime/receipt-system)
- [Runtime release integrity](/docs/release/reproducible_builds)

## Federation

Federation lets operational responsibility extend beyond a single permanent host. The goal is continuity, recoverability, and independent checking rather than a single magic server.

- [Federation architecture](/docs/architecture/federation)
- [Federation runtime](/docs/federation-runtime)
- [Future federation risks](/docs/security/future_federation_risks)

## World Packaging

Packaging is the handoff between creators, operators, marketplaces, and verification tooling.

- [Canonical package format](/docs/canonical-package-format)
- [Game package format](/docs/GAME_PACKAGE_FORMAT)
- [Deployment readiness matrix](/docs/architecture/deployment-readiness-matrix)

## Continuity

Continuity is the user-facing promise: worlds remember what happened and can continue after upgrades, incidents, migrations, and operator changes.

- [Continuity Engine](/continuity-engine)
- [Checkpoint system](/docs/architecture/runtime/checkpoint-system)
- [Restoration](/docs/architecture/restoration)
- [Operational recovery](/docs/architecture/deployment/operational-recovery)

## Next steps

- Check the [Capability Matrix](/developers/capabilities) for maturity.
- Read [Developers](/developers) for the creator journey.
- Use the [Architecture](/architecture) page as the full docs index.
