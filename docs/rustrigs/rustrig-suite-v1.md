# RustRig Mutation Library Suite v1

RustRig Suite v1 packages certified deterministic gameplay mutation kernels for independent re-check. Each crate exposes `apply`, `replay`, `state_root`, `certified_status`, a named mutation wrapper, invariant documentation, property targets, and certification notes.

| Mutation | Crate | Proof target | Report | Status |
|---|---|---|---|---|
| `inventory.transfer` | `everarcade-rustrig-inventory` | `proof-targets/rustrigs/inventory-transfer/` | `reports/rustrig-suite/inventory-transfer-report.txt` | PASS |
| `combat.attack` | `everarcade-rustrig-combat` | `proof-targets/rustrigs/combat-attack/` | `reports/rustrig-suite/combat-attack-report.txt` | PASS |
| `market.trade` | `everarcade-rustrig-market` | `proof-targets/rustrigs/market-trade/` | `reports/rustrig-suite/market-trade-report.txt` | PASS |
| `governance.vote` | `everarcade-rustrig-governance` | `proof-targets/rustrigs/governance-vote/` | `reports/rustrig-suite/governance-vote-report.txt` | PASS |

Flow: `world.evr` → certified RustRig kernels → signed certificate → independent re-check → deploy.

Final output: **RUSTRIG MUTATION LIBRARY SUITE V1: PASS**
