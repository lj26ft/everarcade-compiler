# Executive Summary — EverArcade Compiler

## What this repository is

EverArcade Compiler is an **open-source candidate** for a **deterministic game/world runtime** and **creator tooling**. The core idea: persistent worlds should emit **auditable execution evidence** — receipts, journals, checkpoints, and replay proofs — so outcomes can be verified after the fact.

**Tagline:** Build Worlds. Not Just Games.

**Explicit non-goals today:** production multiplayer federation, live XRPL/Xahau settlement, GPU marketplace, commercial revenue ops, public testnet, production renderer, hosted player gateway.

## What actually works (v0.1 supported path)

An external developer can, on a networked machine with Rust + Node:

1. Create a local world via **Creator SDK** (`creator-sdk/cli/everarcade.mjs`)
2. Build and package it (`world init` → `run` → `package` → `verify`)
3. Run **everarcade-runtime** against `dist/runtime-package/`
4. Inspect receipts, journal, checkpoints, replay reports
5. Verify the **reference certified world** at `examples/reference-certified-world-v1/`

Expected verification string: `WORLD VERIFY: PASS` or `REFERENCE CERTIFIED WORLD V1: PASS`.

## What the repo looks like (scale)

| Metric | Approximate value |
|--------|-------------------|
| Total repo size | ~1.1 GB |
| Rust files | ~9,400 |
| Shell scripts | ~560 |
| Markdown docs | ~474 |
| Report files | ~407 |
| Archive files | ~428 |
| Cargo workspace members | 44 crates/packages |
| Validation scripts | 65 `validate_*.sh` |
| Top-level directories | ~80 |

Largest directories by file count: `vendor/`, `execution-core/`, `runtime/`, `everarcade-host/`, `scripts/`, `docs/`, `reports/`.

## Maturity in one table

| Rating | Subsystems |
|--------|------------|
| **ALPHA** (usable locally) | Execution Core, Runtime, Creator SDK, World Packages, RustRigs |
| **EXPERIMENTAL** | World Contracts, Evernode Deployment |
| **SCAFFOLD** | Federation, Renderer, XRPL, Xahau Hooks, GPU, Portals, Commercial Revenue, Player Gateway |
| **PLANNED** | Public Testnet |
| **PRODUCTION** | **None** |

Source of truth: `MATURITY.md`.

## Open-source readiness verdict

**CONDITIONAL READY** per `scripts/validate_open_source_readiness.sh` and `reports/open_source_readiness_report.txt`.

| Strength | Gap |
|----------|-----|
| Honest top-level docs | Incomplete `vendor/` breaks offline Cargo |
| Security audit PASS (no committed secrets) | No GitHub Actions CI |
| Reference world + onboarding script | `public-testnet/` missing at root but referenced |
| 65 targeted validation scripts | PASS reports can misread as production proof |
| MIT license, CONTRIBUTING, SECURITY | ~560 scripts — easy to run wrong certify script |
| Active development (World Factory, Attestation RC1) | Dual execution stacks confuse integrators |

## Recent development trajectory (git, 2026-06)

Recent merges on `main` (newest first):

- Operator Identity Registry RC1 (#447)
- World Factory MVP Phase 4 (#446)
- World Release Attestation V0.1 RC1 (#445)
- World Release Attestation V0 bundle (#444–443)
- World Factory MVP Phases 1–3 (#440–442)
- world.evr package spec freeze (#438)
- Public frontend moved to `everarcade-frontend` (#435)

Trajectory: tightening **world.evr packaging**, **certification/attestation**, and **export bundles** — not shipping live platform services.

## How SuperGrok should reason about this repo

1. **Start from the proven path**, not directory names.
2. **Treat scaffold PASS reports as model certification**, not product shipping.
3. **Prefer `MATURITY.md` over `release/manifest/components.tsv`** for subsystem status.
4. **Assume two parallel stacks** until explicitly unified (runtime appliance vs everarcade-host).
5. **Assume two package shapes** (`runtime-package` vs `world.evr`) until bridged.
6. **Do not recommend production deployment** without explicit new validation evidence.

## Suggested next work (from prior analysis)

**Contributor Trust Lane:** automated CI gate for `validate_developer_onboarding.sh` + reference world verify, fix `public-testnet/` path, resolve vendor/offline policy, add issue/PR templates.