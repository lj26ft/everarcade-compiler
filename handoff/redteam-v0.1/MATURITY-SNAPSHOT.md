# Current Maturity Snapshot (post-Phase 0)

**Source of truth:** `MATURITY.md` (repo root)  
**Snapshot date:** 2026-06-23

## Ratings

| Rating | Count | Notes |
|--------|-------|-------|
| PRODUCTION | 0 | No subsystem is production-classified |
| ALPHA | Core path + reproducibility | Execution core, runtime, creator-sdk, world packages, rustrigs, offline vendor, CI gates |
| EXPERIMENTAL | 2 | World contracts, Evernode deployment |
| SCAFFOLD | Many | Federation, renderer, XRPL, GPU, portals, commercial, player gateway |
| PLANNED | 1 | Public testnet |

## Phase 0 progress (complete)

| Area | Status |
|------|--------|
| Offline vendor / reproducible builds | ALPHA — `dist/vendor.tar.gz` + `vendor.sha256` |
| Contributor Gate | ALPHA — `onboarding.yml` |
| World Artifact Gate | ALPHA — `deterministic-world-factory.yml` |
| Creator SDK play-local offline fix | ALPHA — repo-root cargo, not `/tmp` |
| Open source readiness docs | READY — `OPEN_SOURCE_READINESS.md` |

## Phase 1 target (in progress)

Advance toward **BETA** on:

- Execution core (targeted replay path)
- Runtime (bridged package load + replay evidence)
- Creator SDK (dual-package documentation + bridge)

**Not in Phase 1:** federation, renderer, settlement, marketplace, public hosting.

## Honest limits

Passing CI gates proves **local reproducibility and world artifact generation**, not production multiplayer, live settlement, or commercial operation.