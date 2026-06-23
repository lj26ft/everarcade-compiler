# Maturity and Reality — EverArcade Compiler

## Classification definitions (`MATURITY.md`)

| Label | Meaning |
|-------|---------|
| PRODUCTION | Public production use with operational guarantees |
| ALPHA | Usable for local/limited workflows with known gaps |
| EXPERIMENTAL | Prototypes demonstrating direction; may change |
| SCAFFOLD | Directory/interface/docs exist; not a working product |
| PLANNED | Intended capability without meaningful implementation |

**No subsystem is PRODUCTION** in the v0.1 open-source milestone.

---

## Official subsystem table

| Subsystem | Status | Reality check |
|-----------|--------|---------------|
| Execution Core | ALPHA | Deep Rust; many modules prototype-only; federation tests real |
| Runtime | ALPHA | `everarcade-runtime` is the proven appliance; partial replay simplifications |
| Creator SDK | ALPHA | CLI works locally; bridge to runtime package incomplete in some paths |
| World Packages | ALPHA | `world.evr` spec frozen; dual shapes coexist |
| RustRigs | ALPHA | Reusable gameplay crates; not production stdlib |
| World Contracts | EXPERIMENTAL | ABI boundaries documented; API stability not final |
| Evernode Deployment | EXPERIMENTAL | Lease proof material; no public operator guarantees |
| Federation | SCAFFOLD | Shell model + records; not live BFT network |
| Renderer | SCAFFOLD | Projection shell model; no graphics pipeline |
| XRPL Settlement | SCAFFOLD | Deterministic intent records; no live ledger |
| Xahau Hooks | SCAFFOLD | 10 identical 9-line C stubs |
| GPU Marketplace | SCAFFOLD | Deterministic job model; `live_hardware_inspection=false` |
| Developer Portal | SCAFFOLD | Records + portal model |
| Player Gateway | SCAFFOLD | Hash-validated `.records`; not a live API |
| Commercial Revenue | SCAFFOLD | Intent transcripts only |
| Public Testnet | PLANNED | **Directory missing at repo root** |
| Security/Validation Reports | REFERENCE | Historical; tie claims to active scripts |

---

## Repo reality audit summary

`reports/repo_reality_audit_report.txt` (2026-06-05): **NOT PRODUCTION READY**

Classification key used in audit:

| Label | Meaning in audit |
|-------|------------------|
| Functional Prototype | Runnable code or deterministic local behavior; lacks production ops |
| Certification Scaffold | Shell models, fixtures, PASS reports — model not live service |
| Documentation Only | READMEs/maps without runnable implementation |
| Obsolete / Duplicate | Overlapping surfaces |

Notable audit classifications:

- `execution-core`, `everarcade-runtime`, `creator-sdk` → Functional Prototype
- `federation`, `renderer`, `gpu`, `player-gateway`, `commercial-revenue`, `hotpocket`, `xrpl`, `xaman` → Certification Scaffold
- `scripts` → Functional Prototype / Obsolete Duplicate (496 scripts, duplication risk)
- `docs`, `reports` → Documentation / Certification Scaffold mix

---

## Certification scaffold pattern (CRITICAL for reasoning)

Many "platform" areas share this shape:

1. `*_model.sh` — fixed inputs → SHA-256 roots
2. Pre-generated `.records` / `*_ROOT` marker files
3. `scripts/validate_*.sh` / `scripts/certify_*.sh`
4. `reports/*_validation_report.txt` ending in **PASS**

**What PASS means:** deterministic transcript/root consistency under local scripted conditions.

**What PASS does NOT mean:** live network, billing, GPU hardware, player traffic, production settlement.

---

## Misleading artifacts (do not over-interpret)

| Artifact | Sounds like | Actually is |
|----------|-------------|-------------|
| `release/manifest/components.tsv` PASS rows | Shipped subsystems | Doc-linked model certification |
| `reports/release-certification/release-report.txt` | Full platform release | v0.1 RC gates for core runtime |
| `reports/*_certification_*` per area | Operational validation | Model/root checks |
| `*-runtime-v0.1` directory names | Production runtime | Deterministic shell models |
| `federation/README.md` "Multi-Lease Federation Runtime" | Live federation | Evidence-exchange model |
| `hooks/*-hook` names | Deployed Xahau hooks | Identical C placeholders |
| `public-testnet` certification PASS | Live testnet | Fixtures in `public-surface-export/` only |

**Prefer:** `MATURITY.md` + `reports/repo_reality_audit_report.txt` + active validation scripts.

---

## Per-area depth (scaffold domains)

### federation/
- `multi_lease_model.sh` (~255 lines) — real deterministic model
- Static records in `members/`, `identity/`, `topology/`, etc.
- Orphan `.rs` stubs (14-line structs, not in any `Cargo.toml`)
- Real simulation: `execution-core/tests/federation_simulation_tests.rs`

### renderer/
- `projection/projection_model.sh` only
- No WebGL/WebGPU, no frame loop, no asset pipeline

### hooks/
- 10 packages, each `hook.c`: `int64_t hook(...) { return 0; }`

### gpu/
- `jobs/gpu_model.sh`, `marketplace/marketplace_model.sh`
- `runtime/gpu-runtime/src/lib.rs` — type mirror of shell contract

### creator-marketplace/ / commercial-revenue/
- Shell models + `.records` files
- No HTTP API, billing, payouts

### player-gateway/
- 32 files of `.records` validated by SHA-256 vs `*_ROOT`
- Not an authenticated gateway service

### frontend/
- `arena-live-client/app.js` — real HTTP to local runtime (`/state`, `/input`)
- `player-portal` — WebSocket to Arena Vanguard feed
- `creator-dashboard`, `operator-console` — static UI shells
- Not listed in `MATURITY.md` (gap); treat dashboards as SCAFFOLD, arena-live-client as ALPHA prototype

### public-testnet/
- **MISSING** at `/public-testnet/`
- `scripts/validate_public_testnet.sh` sources `$ROOT_DIR/public-testnet/testnet_model.sh` → **broken on clean checkout**
- Fixtures exist: `public-surface-export/registry-fixtures/public-testnet/`

---

## Security posture (v0.1)

`reports/open-source-readiness/security-audit.txt` (2026-06-18): **PASS**

- No committed production secrets, API keys, or PEM private keys in first-party materials
- Field names like `secret_root`, `resume_token` are model terms, not credentials
- `SECURITY.md` lacks dedicated security inbox email
- Creator SDK generates local Ed25519 attester keys in project dir (`everarcade.mjs`)

---

## Runtime capabilities matrix (`docs/runtime-capabilities.md`)

**Implemented:** WASM execution foundation, package loading, state/roots, receipts, journal, checkpoints, replay verification, recovery foundations.

**Partial:** backup, restore, upgrade, health/metrics, world runtime, federation recovery, distributed receipts, checkpoint sync, Evernode deployment automation.

**Scaffold:** multi-host federation, renderer streaming, historical replay/observer, XRPL integration.

**Planned:** ZK integration, creator marketplace (as production).