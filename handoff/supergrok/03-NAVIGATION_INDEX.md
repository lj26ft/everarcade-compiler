# Navigation Index — EverArcade Compiler

## Start here by goal

| Goal | Go to |
|------|-------|
| Understand what repo is | `README.md`, `docs/01-executive-overview.md` |
| What works today | `MATURITY.md`, `docs/runtime-capabilities.md` |
| Create first world | `docs/first-world.md`, `creator-sdk/cli/everarcade.mjs` |
| v0.1 public demo | `examples/reference-certified-world-v1/` |
| Runtime internals | `docs/04-runtime-architecture.md`, `runtime/everarcade-runtime/` |
| World package spec | `docs/world-package-spec-v1.md`, `specs/world-evr-package/` |
| Open source readiness | `OPEN_SOURCE_READINESS.md`, `docs/open-source/v0.1-public-release-readiness.md` |
| Gaps and roadmap | `docs/12-gap-analysis.md` |
| Architecture freeze | `docs/14-v0.1-architecture-freeze.md` |
| Directory ownership | `REPOSITORY_MAP.md` |
| Contribute | `CONTRIBUTING.md`, `docs/contributor-guide/` |
| Security | `SECURITY.md`, `reports/open-source-readiness/security-audit.txt` |
| Historical evidence | `archive/`, `reports/` (reference only) |

---

## Top-level directory map

| Directory | Files | Audience | Maturity |
|-----------|------:|----------|----------|
| `execution-core/` | ~1925 | Runtime/protocol contributors | ALPHA (uneven) |
| `runtime/` | ~847 | Runtime contributors | ALPHA |
| `everarcade-host/` | ~765 | Host/federation researchers | Mixed scaffold |
| `creator-sdk/` | ~86 | World developers | ALPHA |
| `crates/` | ~149 | Library contributors | ALPHA |
| `rustrigs/` | ~34 | Gameplay module authors | ALPHA |
| `sdk/` | ~62 | SDK consumers | ALPHA/EXPERIMENTAL |
| `contracts/` | — | Contract developers | EXPERIMENTAL |
| `compiler/` | — | Meta tooling | EXPERIMENTAL |
| `examples/` | ~166 | Learners, operators | Reference |
| `templates/` | ~164 | World starters | Prototype |
| `specs/` | ~114 | Spec authors | Authoritative specs |
| `docs/` | ~476 | Everyone | Canonical docs |
| `scripts/` | ~571 | Contributors, operators | Automation |
| `reports/` | ~407 | Auditors | Evidence (caution) |
| `archive/` | ~428 | Maintainers | Historical |
| `federation/` | ~30 | Federation researchers | SCAFFOLD |
| `renderer/` | ~10 | Renderer contributors | SCAFFOLD |
| `gpu/` | ~11 | GPU researchers | SCAFFOLD |
| `hooks/` | ~20 | XRPL hook researchers | SCAFFOLD |
| `xrpl/`, `xaman/` | — | Settlement researchers | SCAFFOLD |
| `player-gateway/` | ~32 | Portal researchers | SCAFFOLD |
| `creator-marketplace/` | ~29 | Marketplace researchers | SCAFFOLD |
| `commercial-revenue/` | ~32 | Economics researchers | SCAFFOLD |
| `frontend/` | ~39 | UI contributors | Prototype |
| `public-surface-export/` | ~609 | Frontend migration | Export bundle |
| `evernode/`, `deployment/`, `deploy/` | — | Operators | EXPERIMENTAL |
| `hotpocket/`, `hotpocket-arena-wrapper/` | — | Integration researchers | SCAFFOLD |
| `proofs/`, `proof-targets/` | — | Formal proof work | Reference |
| `vendor/` | ~9831 | Build (generated) | INCOMPLETE |
| `target/` | ~1744 | Build output | Generated |

---

## Cargo workspace members (`Cargo.toml`)

```
everarcade-abi
crates/transport-core
crates/canonicalizer-kernel
crates/rustrigs/{identity,movement,world,resources,crafting,structures,factions,quests,continuity,operations,inventory,combat,market,governance}
execution-core
contract-api
control-plane
provider-evernode
rustrigs
contracts/{set-contract,increment-contract,counter-world,counter-contract,arena-proof-contract}
everarcade-host
sdk/{everarcade-sdk,everarcade-world-sdk,everarcade-entity-sdk,everarcade-simulation-sdk,everarcade-economy-sdk,everarcade-governance-sdk,client-bridge}
src-bin-everarcade
runtime/{client,renderer-client,content-registry,everarcade-runtime}
tools
studio-gui
```

---

## Documentation reading order (`docs/README.md`)

1. `01-executive-overview.md`
2. `02-platform-vision.md`
3. `03-system-architecture.md`
4. `04-runtime-architecture.md`
5. `11-production-readiness.md`
6. `12-gap-analysis.md`
7. `runtime-capabilities.md`
8. `13-runtime-operations-manual.md`
9. `14-v0.1-architecture-freeze.md`
10. `repository-navigation.md`
11. `documentation-governance.md`

Older docs under `docs/architecture/`, `docs/runtime/`, etc. are **evidence**, not source of truth when they conflict.

---

## Concern → file quick lookup

| Concern | Primary paths |
|---------|---------------|
| Package load/validate | `runtime/everarcade-runtime/src/runtime/package_loader.rs` |
| Tick / gameplay | `runtime/everarcade-runtime/src/runtime/runtime_loop.rs` |
| Guest WASM | `runtime/everarcade-runtime/src/runtime/guest_wasm.rs` |
| Journal chain | `runtime/everarcade-runtime/src/runtime/journal.rs` |
| Replay | `runtime/everarcade-runtime/src/runtime/replay.rs` |
| Operator commands | `runtime/everarcade-runtime/src/runtime/operator.rs` |
| Creator packaging | `creator-sdk/cli/everarcade.mjs` |
| Canonical roots | `crates/canonicalizer-kernel/src/lib.rs` |
| WASM in execution-core | `execution-core/src/wasm/runtime.rs` |
| Host one-shot run | `everarcade-host/src/runner.rs` |
| Federation model | `federation/multi_lease_model.sh` |
| Federation tests | `execution-core/tests/federation_simulation_tests.rs` |
| world.evr spec | `specs/world-evr-package/` |
| Reference world | `examples/reference-certified-world-v1/` |
| Release gates | `docs/10-release-certification.md` |
| Offline build | `docs/build/offline-build-policy.md` |
| Artifact policy | `docs/repository/artifact-policy.md` |

---

## Node.js workspaces (`package.json`)

```
frontend/creator-dashboard
frontend/player-portal
frontend/operator-console
frontend/shared-ui
frontend/shared-api
frontend/shared-types
frontend/shared-wallet
frontend-gateway
```

---

## What NOT to explore first (time sinks)

- `vendor/` — third-party mirror, incomplete
- `reports/` bulk — mostly historical PASS artifacts
- `archive/` — historical milestones
- `scripts/` bulk — 560 scripts; use targeted list in `05-VALIDATION_COMMANDS.md`
- `execution-core/src/` breadth — 100+ modules, most not on proven path
- `everarcade-host/src/federation_network/` — scaffold unless task requires it