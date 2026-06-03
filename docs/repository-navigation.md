# Repository Navigation Guide

This guide helps new engineers locate subsystems quickly.

## Workspace Structure

| Path | Purpose |
|---|---|
| `execution-core/` | Deterministic execution core. |
| `runtime/everarcade-runtime/` | Runtime lifecycle, operator commands, checkpoints, replay, recovery, backup, upgrade, health, metrics. |
| `everarcade-host/` | Host-side runtime, federation, distributed execution, receipts, convergence, checkpoint sync, identity, archives. |
| `everarcade-abi/` | ABI types and boundaries. |
| `contract-api/` | Contract-facing API. |
| `sdk/` | Creator and tooling SDK crates. |
| `contracts/` | Example and test contracts. |
| `examples/` | Example worlds and demos. |
| `provider-evernode/` | Evernode provider integration. |
| `control-plane/` | Control-plane foundations. |
| `frontend/` | Player portal, operator console, creator dashboard, shared frontend packages. |
| `runtime/client/` | Runtime client crate. |
| `runtime/renderer-client/` | Renderer/projection client domain. |
| `tools/` | Repository tooling. |
| `docs/` | Canonical docs, runbooks, references, historical evidence. |
| `deployment/` | Deployment reports and provider evidence. |
| `release/` | Release manifests and checksums. |
| `certification/` | Certification run artifacts. |

## Runtime Crates

- `runtime/everarcade-runtime`: direct runtime operator binary and library.
- `everarcade-host`: broader host/federation runtime modules.
- `runtime/client`: client-facing runtime integration.
- `runtime/renderer-client`: projection and renderer-facing client work.

## SDK Crates

- `sdk/everarcade-sdk`
- `sdk/everarcade-world-sdk`
- `sdk/everarcade-entity-sdk`
- `sdk/everarcade-simulation-sdk`
- `sdk/everarcade-economy-sdk`
- `sdk/everarcade-governance-sdk`
- `sdk/client-bridge`

## Example Contracts

Look in `contracts/` for small contract fixtures and `examples/` for larger world examples. Use these for onboarding and targeted tests, not as proof of production readiness.

## Test Suites

- `execution-core/tests/`: deterministic execution tests.
- `everarcade-host/tests/`: host, federation, receipt, checkpoint, recovery, and deployment-facing tests.
- `sdk/tests/`: SDK/runtime integration tests.
- `frontend/tests/`: browser and frontend integration tests.

Run targeted crate tests rather than full workspace tests unless explicitly required.

## Validation Scripts

Use `scripts/` for release validation, diagnostics, bootstrap, and targeted validation workflows. Before relying on a script for certification, confirm it is named in `10-release-certification.md` or release notes.

## Reports

Use `deployment/reports/`, `benchmarks/reports/`, and older `docs/**` reports as evidence. Reports are not authoritative architecture unless referenced by the canonical numbered docs.

## Runbooks

Operator runbooks live in `docs/runbooks/` and `docs/operators/`. The authoritative operator entry point is `13-runtime-operations-manual.md`.

## Release Artifacts

Release artifacts and verification files live in `release/`, including manifest and checksum files. Certification evidence may also appear in `certification/` and deployment reports.
