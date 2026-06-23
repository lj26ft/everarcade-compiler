# Task Document: Single-World Deterministic Path v0.1 Hardening (Phase 1)

**Owner:** Grok Build  
**Priority:** Phase 1 — Highest current leverage  
**Scope:** Strictly limited to the single-world deterministic runtime path per `docs/14-v0.1-architecture-freeze.md`. No federation, renderer, settlement, marketplace, or public hosting work.  
**Goal:** Advance runtime + creator-sdk + targeted execution-core from ALPHA toward BETA with clear, verifiable, reproducible evidence. Eliminate dual-package confusion and strengthen replay/journal/checkpoint guarantees.

## Why this matters (ELI5)

We have one main way to create and run worlds. Right now the instruction booklets (packages) and the proof of what happened (replay evidence) sometimes don't line up perfectly. This task makes them consistent so anyone can trust the evidence without guessing which booklet was used.

## Sub-tasks (priority order)

### 1. Clear documentation of dual package shapes

Update `creator-sdk/README.md`, `docs/03-system-architecture.md`, `docs/04-runtime-architecture.md`, and `docs/world-package-spec-v1.md` with a comparison table (runtime-package vs world.evr V1, consumers, bridging steps, current gaps).

Add maturity banners per `docs/DOCUMENTATION_POLICY.md`.

### 2. Complete the bridge from SDK to runtime appliance

Ensure `creator-sdk/cli/everarcade.mjs world package` output reliably loads into `everarcade-runtime` (manifest handling, WASM, world.json).

Fix any conversion or compatibility gaps.

### 3. Align hashing and canonical roots

Make `crates/canonicalizer-kernel` roots consistent with `runtime/everarcade-runtime/src/runtime/runtime_loop.rs` (serde_json).

Add explicit compatibility verification in the canonical gate if full unification is not immediate.

### 4. Strengthen full replay evidence

Expand replay verification to cover placeholder, official-template, and wasm-guest classifications with complete journal/checkpoint/receipt output.

Add basic backup/restore foundations tied to the journal.

### 5. Update validation gate and CI

Extend `scripts/validate_developer_onboarding.sh` (or add a targeted replay script) to exercise the bridged path + full replay.

Keep the existing onboarding CI green and add replay evidence upload.

## ELI5 contributor guidance (add to docs)

> We now have one clear instruction manual and rock-solid proof that the world ran exactly as expected. Clone, run the gate, and you're building verifiable worlds.

## Acceptance criteria

- Dual-package confusion eliminated in all key docs.
- Default proof path produces matching roots and complete replay evidence.
- Canonical gate passes with full verification.
- `MATURITY.md` updated to reflect BETA progress on core subsystems.
- No scope creep into scaffold areas.

## Validation

Run the canonical gate + targeted replay checks:

```bash
bash scripts/ensure_vendor_offline.sh
bash scripts/check_prerequisites.sh
CARGO_BUILD_JOBS=1 bash scripts/validate_developer_onboarding.sh
bash examples/reference-certified-world-v1/operator/verify.sh examples/reference-certified-world-v1
bash scripts/ci/run-deterministic-world-factory.sh
```

Update `handoff/redteam-v0.1/` and `handoff/supergrok/` after completion.

## Reference: dual-stack gotchas

See `handoff/supergrok/06-GOTCHAS_AND_DUAL_STACKS.md` for:

- Stack 1 (runtime-package) vs Stack 2 (everarcade-host)
- Canonicalizer vs runtime hashing divergence
- PASS reports ≠ production claims