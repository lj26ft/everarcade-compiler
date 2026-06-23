# Open Source Readiness — EverArcade Compiler

## Current classification

| Gate | Status | Source |
|------|--------|--------|
| Documentation present | PASS | `scripts/validate_open_source_readiness.sh` |
| Security audit | PASS | `reports/open-source-readiness/security-audit.txt` |
| License (MIT) | PASS | `LICENSE` |
| Reference world | PASS | `examples/reference-certified-world-v1/` |
| Offline vendor | WARNING | Incomplete; bincode missing |
| Overall | **READY** (Phase 0 vendor + CI) | `reports/open_source_readiness_report.txt` |

Release candidate audit (`reports/release-candidate-audit/release-readiness-report.md`): proceed with v0.1 RC review; **do not claim production/public-testnet/commercial**.

---

## Strengths (ready to show externally)

1. **Honest positioning** in `README.md` — explicitly not production/testnet/commercial
2. **`MATURITY.md`** — no subsystem rated PRODUCTION
3. **Documented local proof path** — Creator SDK + reference world
4. **Security scan clean** — no committed secrets (2026-06-18 audit)
5. **Canonical docs hierarchy** — `docs/` with numbered architecture set
6. **Contributor docs** — `CONTRIBUTING.md`, `SECURITY.md`, `CODE_OF_CONDUCT.md`
7. **65 targeted validation scripts** — local proof automation exists
8. **Active maintenance** — recent World Factory, Attestation, Operator Identity work

---

## Gaps (prioritized)

### P0 — Blockers / high confusion

| # | Gap | Impact | Fix direction |
|---|-----|--------|---------------|
| 1 | Incomplete `vendor/` + offline Cargo | Clean clone may fail full workspace build | Restore vendor via `scripts/vendor_deps.sh` OR document network bootstrap + relax offline for dev |
| 2 | PASS reports imply production | External users misread scaffold as shipped | Relabel `components.tsv`; add disclaimers to certification reports |
| 3 | `public-testnet/` missing at root | `validate_public_testnet.sh` broken on clean checkout | Restore dir or repoint script to `public-surface-export/registry-fixtures/public-testnet/` |
| 4 | No CI/CD | No automated gate for contributors | GitHub Actions for onboarding + reference world verify |
| 5 | Dual stack / dual package confusion | Wrong integration path chosen | Consolidate docs; bridge SDK → runtime-package everywhere |

### P1 — Should fix for credible v0.1 release

| # | Gap | Fix direction |
|---|-----|---------------|
| 6 | Maturity banners missing on some READMEs | Add SCAFFOLD links per `docs/DOCUMENTATION_POLICY.md` |
| 7 | No security contact email | Add to `SECURITY.md` or GitHub private advisories |
| 8 | 560 scripts — wrong script risk | Document canonical 3-command gate in `CONTRIBUTING.md` |
| 9 | `validate_open_source_readiness.sh` requires `rg` | Add prerequisites check; fallback to `grep` |
| 10 | `frontend/` not in `MATURITY.md` | Add entry (SCAFFOLD dashboards, ALPHA arena-live-client) |
| 11 | Documentation duplication | Continue dedup per `OPEN_SOURCE_READINESS.md` |

### P2 — Acceptable for v0.1 if scoped

| # | Gap | Notes |
|---|-----|-------|
| 12 | Restore command partial | Documented in `docs/12-gap-analysis.md` |
| 13 | Placeholder WASM for templates | Expected for non-guest templates |
| 14 | Orphan `federation/*.rs` stubs | Remove or relocate |
| 15 | Centralized `tests/` sparse | Tests live in crates — document pattern |
| 16 | Abuse/acceptable-use policy | `reports/abuse_analysis_report.txt` notes gap |

### Out of scope for v0.1 OSS (explicitly)

- Live federation, public testnet, commercial marketplace
- XRPL/Xahau production settlement
- GPU provider marketplace
- Production renderer
- External security audit, SLOs, incident response

---

## Recommended next task: Contributor Trust Lane

**Goal:** Make "clone → verify in 30 minutes" true and CI-enforced.

### Deliverables

1. `scripts/check_prerequisites.sh` — bash, cargo, node, rg, openssl
2. Fix `public-testnet/` path integrity
3. Canonical gate (3 commands):
   ```bash
   bash scripts/check_prerequisites.sh
   CARGO_BUILD_JOBS=1 bash scripts/validate_developer_onboarding.sh
   bash examples/reference-certified-world-v1/operator/verify.sh examples/reference-certified-world-v1
   ```
4. `.github/workflows/onboarding.yml` — run gate on PR/main
5. Issue + PR templates with maturity-impact checkbox
6. Vendor decision documented (restore OR network-required bootstrap)

### Definition of done

- Fresh clone passes 3-command gate on Ubuntu/macOS
- CI green on same gate
- `validate_open_source_readiness.sh` → READY (vendor resolved or waived)
- `CONTRIBUTING.md` points to canonical gate only

---

## Public release framing (use this verbatim)

> **EverArcade v0.1** is an experimental deterministic world-runtime toolkit. Create, run, package, and locally verify worlds with replay evidence. Federation, settlement, GPU, marketplace, and portals are scaffold or planned — not production software.

---

## Audit file index

| File | Purpose |
|------|---------|
| `OPEN_SOURCE_READINESS.md` | Maintainer gap audit |
| `reports/open_source_readiness_report.txt` | Script output |
| `reports/open-source-readiness/security-audit.txt` | Secret scan + quick start |
| `reports/repo_reality_audit_report.txt` | Area-by-area reality |
| `reports/release-candidate-audit/release-readiness-report.md` | RC checklist |
| `docs/open-source/v0.1-public-release-readiness.md` | Public consumption checklist |
| `docs/12-gap-analysis.md` | v0.1 / beta / production gaps |