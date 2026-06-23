# Gotchas and Dual Stacks — Agent Reasoning Guide

## Trap 1: Two execution stacks

| | Stack 1 (PRIMARY) | Stack 2 (SECONDARY) |
|---|-------------------|---------------------|
| Binary | `everarcade-runtime` | `everarcade-host` |
| Package | `dist/runtime-package/` (JSON manifest + WASM) | Civilization bincode |
| Entry | `runtime/everarcade-runtime/src/main.rs` | `everarcade-host/src/runner.rs` |
| Execution | Template/guest WASM/placeholder tick | `execute_vm_boundary` hash chain |
| Maturity | ALPHA — proven local path | Mixed — anchor queue + federation scaffold |

**Agent rule:** Default all integration advice to Stack 1 unless task explicitly mentions civilization packages or anchoring.

---

## Trap 2: Two package formats

| Format | Path | Consumer |
|--------|------|----------|
| Runtime package | `dist/runtime-package/manifest.json` | `everarcade-runtime` |
| world.evr V1 | `out/world.evr/`, `dist/world.evr` | Certification, operators, specs |

Creator SDK produces both in different commands. They are **not interchangeable** without explicit bridging.

`examples/reference-certified-world-v1/` demonstrates world.evr certification, not necessarily the same load path as runtime appliance.

---

## Trap 3: Canonicalizer vs runtime hashing

- `crates/canonicalizer-kernel` — certification-grade canonical JSON + SHA256
- `runtime_loop.rs` — separate `ArenaState` hashing via `serde_json`

Certification roots and runtime roots **may disagree**. Do not assume one root validates the other without explicit proof script.

---

## Trap 4: PASS reports ≠ production

Running `scripts/certify_*.sh` or reading `reports/*_certification_*` produces PASS for **deterministic shell models**.

`release/manifest/components.tsv` lists federation, renderer, gpu, public-testnet as PASS — this is **certification linkage**, not shipping status.

Always cross-check `MATURITY.md`.

---

## Trap 5: execution-core breadth illusion

~1,900 files and 100+ modules suggest a complete platform. Most modules are:

- Design scaffolding
- Certification models
- Tests for future integration
- Not wired to `everarcade-runtime` main path

`execute_vm` applies empty state changes in some paths. `execute_vm_boundary` is hash-only.

---

## Trap 6: Vendor / offline Cargo

`.cargo/config.toml` sets `offline = true`. `vendor/` is restored from `dist/vendor.tar.gz` via `scripts/ensure_vendor_offline.sh`.

**CI/play-local trap (fixed Phase 0):** Creator SDK previously copied runtime to `/tmp/everarcade-runtime-launch-workspace` without vendor — caused `no matching package named 'anyhow'`. Runtime commands now use repo-root `cargo run --offline --locked -p everarcade-runtime`.

Maintainer regen: `bash scripts/vendor_deps.sh` (network once).

---

## Trap 7: public-testnet path broken

- `REPOSITORY_MAP.md` lists `public-testnet/`
- `scripts/validate_public_testnet.sh` sources `$ROOT_DIR/public-testnet/testnet_model.sh`
- Directory **does not exist** at repo root
- Fixtures: `public-surface-export/registry-fixtures/public-testnet/`

Any agent running public-testnet validation will fail until path is fixed.

---

## Trap 8: src-bin-everarcade stub commands

`src-bin-everarcade/src/commands/mod.rs` — many handlers return deterministic JSON stubs, not live runtime RPC.

Do not cite `runtime-*-status` CLI output as live system state.

---

## Trap 9: Creator SDK non-authority

`creator-sdk/README.md`:
- SDK does not own runtime authority
- Output is "local proof records"
- Does not yet fully bridge to runtime appliance in all paths
- Generates local Ed25519 attester keys in project directory

`world deploy` / `world publish` may write scaffold metadata only.

---

## Trap 10: Frontend split

- Public sites: external `everarcade-frontend`
- In-repo `frontend/`: prototypes
- `arena-live-client` is real (HTTP to local runtime)
- Dashboards are static shells

Do not point contributors to in-repo frontend for production portal work.

---

## Trap 11: compiler/ is not a compiler

`compiler/agent/agent_v8.py` scaffolds repo structure. It does not compile game source to WASM.

---

## Trap 12: federation/ Rust stubs are orphan

`federation/*.rs` — 14-line structs, `dead_code`, not in `Cargo.toml`.

Real federation logic: `execution-core/tests/federation_simulation_tests.rs`.

---

## Trap 13: hooks/ are identical stubs

All 10 `hook.c` files are the same placeholder. No Xahau build pipeline in repo.

---

## Trap 14: Git hygiene

`.gitignore` excludes: `vendor/`, `target/`, `.everarcade/`, generated receipts/checkpoints, secret filename patterns.

Do not commit `node_modules/`, local keys, or runtime state roots.

---

## Trap 15: Validation script dependency on rg

`validate_open_source_readiness.sh` uses `rg`. Without ripgrep installed, README checks false-FAIL.

---

## Decision tree for agents

```text
Task mentions world create/run/verify?
  YES → creator-sdk + everarcade-runtime + dist/runtime-package
  NO ↓
Task mentions world.evr / certification / attestation?
  YES → specs/world-evr-package + examples/reference-certified-world-v1 + canonicalizer-kernel
  NO ↓
Task mentions federation / multiplayer / peers?
  YES → Check MATURITY.md (SCAFFOLD) → execution-core tests + federation/multi_lease_model.sh
        Do NOT promise live network
  NO ↓
Task mentions XRPL / Xaman / settlement?
  YES → SCAFFOLD → xrpl/ xaman/ hooks/ — records only
  NO ↓
Task mentions GPU / marketplace / revenue?
  YES → SCAFFOLD → gpu/ creator-marketplace/ commercial-revenue/ — shell models
  NO ↓
Task mentions public testnet?
  YES → PLANNED → path broken → warn user
  NO ↓
Task mentions production deployment?
  YES → Decline or require new validation evidence — repo is NOT PRODUCTION READY
```

---

## Safe claims agents CAN make

- Local deterministic world creation and verification works (ALPHA)
- Reference certified world v1 demonstrates packaging and certification
- Runtime emits journals, receipts, checkpoints, replay material
- Security audit found no committed production secrets (2026-06-18)
- v0.1 architecture freeze limits scope to single-world runtime path

## Unsafe claims agents MUST NOT make

- Production-ready platform
- Live public testnet
- Working GPU marketplace or commercial revenue
- Deployed Xahau hooks
- Production federation / multiplayer
- Live XRPL settlement
- Offline reproducible builds (until vendor complete)
- PASS report = shipped feature