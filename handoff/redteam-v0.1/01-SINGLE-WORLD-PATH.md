# Single-World Deterministic Path v0.1

**Authority:** `MATURITY.md` + `docs/14-v0.1-architecture-freeze.md`

## Proven path (Stack 1 — default)

```text
creator-sdk/cli/everarcade.mjs
  → dist/runtime-package/
  → runtime/everarcade-runtime
  → journal / checkpoints / replay
```

**Do not default to** `everarcade-host` civilization bincode path unless explicitly scoped.

## Secondary path (World Factory / operators)

```text
world-blueprint.json + world-contract-plan.json
  → world.evr (V1 package)
  → verify → boot → run → replay → deploy → attest → release bundle
```

Commands live in `creator-sdk/cli/everarcade.mjs` world factory subcommands. See `OPEN_SOURCE_READINESS.md` world artifact workflow.

## Dual-package trap (Phase 1 target)

| Format | Typical path | Primary consumer |
|--------|--------------|------------------|
| Runtime package | `dist/runtime-package/manifest.json` + WASM | `everarcade-runtime` |
| world.evr V1 | `out/world.evr/`, examples | Certification, operators, specs |

These are **not interchangeable** without explicit bridging. `examples/reference-certified-world-v1/` demonstrates world.evr certification; onboarding uses runtime-package + play-local.

## Protected CI gates

| Gate | Workflow | Local | Expected |
|------|----------|-------|----------|
| Contributor Gate | `.github/workflows/onboarding.yml` | `bash scripts/validate_open_source_readiness.sh` | `READY` |
| World Artifact Gate | `.github/workflows/deterministic-world-factory.yml` | `bash scripts/ci/run-deterministic-world-factory.sh` | `PASS` |

**Contributor Level 0** (before first code change):

```bash
bash scripts/ensure_vendor_offline.sh
bash scripts/validate_open_source_readiness.sh
bash scripts/ci/run-deterministic-world-factory.sh
```

## Canonical validation gate

```bash
bash scripts/check_prerequisites.sh
CARGO_BUILD_JOBS=1 bash scripts/validate_developer_onboarding.sh
bash examples/reference-certified-world-v1/operator/verify.sh examples/reference-certified-world-v1
```

**Expected:** `Prerequisites: PASS`, onboarding PASS, `REFERENCE CERTIFIED WORLD V1: PASS` / `WORLD VERIFY: PASS`

## Current gaps (Phase 1)

- Dual package bridging (SDK runtime-package ↔ world.evr clarity and load path)
- Canonicalizer-kernel roots vs `runtime_loop.rs` serde_json hashing alignment
- Full replay across placeholder, official-template, and wasm-guest classifications
- Backup/restore foundations tied to journal
- Documentation comparison tables in key architecture docs

## Key files

| Role | Path |
|------|------|
| Creator CLI | `creator-sdk/cli/everarcade.mjs` |
| Runtime binary | `runtime/everarcade-runtime/` |
| Canonicalizer | `crates/canonicalizer-kernel/` |
| Runtime loop | `runtime/everarcade-runtime/src/runtime/runtime_loop.rs` |
| Reference world | `examples/reference-certified-world-v1/` |
| World Factory example | `examples/world-factory/frontier-settlement/` |
| Onboarding script | `scripts/validate_developer_onboarding.sh` |
| World Factory CI | `scripts/ci/run-deterministic-world-factory.sh` |