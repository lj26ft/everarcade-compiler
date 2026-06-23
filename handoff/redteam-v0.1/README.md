# Redteam Handoff Bundle — EverArcade Compiler v0.1 Single-World Path

**Generated**: 2026-06-23 (post-Phase 0)  
**Purpose**: Focused snapshot for redteam review of the single-world deterministic runtime path.  
**Status**: Phase 0 complete (Contributor Gate + World Artifact Gate + offline vendor). Entering Phase 1 hardening.

## How to use

1. Start with [`00-EXECUTIVE-SUMMARY.md`](00-EXECUTIVE-SUMMARY.md)
2. Review [`01-SINGLE-WORLD-PATH.md`](01-SINGLE-WORLD-PATH.md) for current authority, gates, and gaps
3. Read [`PHASE1-TASKS.md`](PHASE1-TASKS.md) for the active work plan
4. Validate with the canonical gate (commands in `01-SINGLE-WORLD-PATH.md`)

## Key guardrails (never violate)

- **Default proof path:** `creator-sdk/cli/everarcade.mjs` → `dist/runtime-package/` → `runtime/everarcade-runtime`
- **No PRODUCTION subsystems** per `MATURITY.md`
- **Respect v0.1 architecture freeze** (`docs/14-v0.1-architecture-freeze.md`) — single-world only
- **Offline/reproducible vendor policy** enforced (`dist/vendor.tar.gz`, `vendor.sha256`, CI gates)
- **Dual-package awareness:** runtime-package vs `world.evr` V1 are not interchangeable without explicit bridging (Phase 1 target)

## Related bundles

| Bundle | Scope |
|--------|-------|
| `handoff/supergrok/` | Full repo orientation (2026-06-23 baseline) |
| `handoff/redteam-v0.1/` | This bundle — single-world path + Phase 1 only |

## Packaging

From repo root:

```bash
tar -czf handoff/redteam-handoff-v0.1.tar.gz -C handoff redteam-v0.1
cp handoff/redteam-handoff-v0.1.tar.gz redteam-handoff-v0.1.tar.gz
```

Committed transport copy: `redteam-handoff-v0.1.tar.gz` (repo root).