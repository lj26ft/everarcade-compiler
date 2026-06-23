# Executive Summary for Redteam

**Repository:** everarcade-compiler  
**Tagline:** Build Worlds. Not Just Games.  
**Positioning:** v0.1 open-source candidate — local deterministic world runtime proofs, **not** a production platform.

## What this is

EverArcade is an experimental **single-world deterministic toolkit** with replay evidence. Persistent worlds emit auditable execution artifacts: receipts, journals, checkpoints, and replay proofs.

## Phase status

| Phase | Status |
|-------|--------|
| Phase 0 — Contributor Trust Lane | **Complete** |
| Phase 1 — Single-world path hardening | **Active** |

**Phase 0 delivered:**

- Canonical offline vendor (`dist/vendor.tar.gz`, `vendor.sha256`, `vendor-manifest.json`)
- Contributor Gate (`.github/workflows/onboarding.yml`)
- World Artifact Gate (`.github/workflows/deterministic-world-factory.yml`)
- `OPEN_SOURCE_READINESS.md` gate documentation
- Locale-independent vendor tree hashing (`LC_ALL=C`) for CI parity

## Current focus (Phase 1)

Bridge dual package shapes, align canonicalizer vs runtime hashing, and strengthen full replay evidence across all WASM classifications. Target: ALPHA → BETA on core path subsystems.

**Explicitly out of scope:** federation, renderer, settlement, marketplace, public hosting, public-testnet production claims.

## What works today

- Reference certified world verifies: `examples/reference-certified-world-v1/`
- Developer onboarding gate: `scripts/validate_developer_onboarding.sh`
- World Factory pipeline: `scripts/ci/run-deterministic-world-factory.sh`
- Offline builds after `bash scripts/ensure_vendor_offline.sh`

## Authority hierarchy

1. `MATURITY.md` + `docs/14-v0.1-architecture-freeze.md`
2. `handoff/supergrok/` (full baseline, 2026-06-23)
3. `OPEN_SOURCE_READINESS.md` (gate model)
4. This redteam bundle (Phase 1 slice)

## Redteam lens

Review for:

- Dual-package confusion (runtime-package vs `world.evr`)
- Root/hash mismatches between canonicalizer and runtime loop
- Replay evidence completeness across placeholder/template/guest paths
- Claims that exceed ALPHA maturity

**Trust the process, not the machine.**