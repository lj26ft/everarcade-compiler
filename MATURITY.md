# EverArcade Maturity Classification

This file answers: **What actually works today?**

## Classification definitions

- **PRODUCTION** — supported for public production use with operational guarantees.
- **ALPHA** — usable for local or limited workflows with known gaps.
- **EXPERIMENTAL** — prototypes that demonstrate direction but may change substantially.
- **SCAFFOLD** — directory, interface, documentation, or placeholder shape exists; do not treat as working product.
- **PLANNED** — intended capability without meaningful implementation in this repository.

## Subsystem status

| Subsystem | Status | Rationale |
| --- | --- | --- |
| Execution Core | ALPHA | Local deterministic execution and replay-oriented proof paths exist, but production guarantees and broad adversarial hardening are not complete. |
| Runtime | ALPHA | Local package execution, receipts, journals, checkpoints, and replay verification are the primary proven path. |
| Creator SDK | ALPHA | Local project creation, build, manifest validation, packaging, and play-local flows are supported for onboarding. |
| World Packages | ALPHA | Canonical package shape is documented and used by local flows; distribution and upgrade policy remain evolving. |
| World Contracts | EXPERIMENTAL | ABI and contract boundaries are documented and partially proven; public API stability is not final. |
| RustRigs | ALPHA | Reusable gameplay domain libraries exist for authoring and examples; they are not a production standard library yet. |
| Federation | SCAFFOLD | Treat federation/history domains as scaffold-level runtime areas; design notes and prototypes exist but not real multiplayer production federation. |
| Renderer | SCAFFOLD | Renderer and projection areas are not the canonical proof path and should not be presented as a production client runtime. |
| XRPL Settlement | SCAFFOLD | Settlement boundaries and proof documents exist, but live settlement is not a production system. |
| Xahau Hooks | SCAFFOLD | Hook directories document intended boundaries and proof surfaces; not production-ready. |
| Evernode Deployment | EXPERIMENTAL | Deployment and lease proof material exists, but public operator guarantees are not established. |
| GPU Marketplace | SCAFFOLD | Marketplace and worker directories describe future hosting economics and verification; not a working production marketplace. |
| Developer Portal | SCAFFOLD | Portal directories and docs exist; contributor onboarding should rely on CLI and docs first. |
| Player Gateway | SCAFFOLD | Player-facing concepts and guide material exist; production player experience is not complete. |
| Commercial Revenue | SCAFFOLD | Revenue and marketplace plans are reference material, not active production systems. |
| Public Testnet | PLANNED | Public-testnet operation is not currently supported as production or alpha capability. |
| Security/Validation Reports | REFERENCE | Historical evidence is preserved in `archive/`; current claims must be tied to active validation scripts. |

No subsystem is classified as **PRODUCTION** in this open-source readiness milestone.

## Reproducibility and contributor experience (Phase 0)

| Area | Status | Rationale |
| --- | --- | --- |
| Offline vendor / build reproducibility | ALPHA | `dist/vendor.tar.gz` restores `vendor/` for offline `cargo metadata` and targeted `cargo check`; `scripts/check_prerequisites.sh` and CI enforce zero network Cargo fetches. Maintainer refresh via `scripts/vendor_deps.sh`. |
| Contributor onboarding gate | ALPHA | Canonical 3-command gate (`check_prerequisites`, `validate_developer_onboarding`, reference world verify) documented in `CONTRIBUTING.md` and `.github/workflows/onboarding.yml`. |
| CI enforcement | ALPHA | Ubuntu + macOS onboarding workflow with early `cargo check --offline`; uploads `reports/` evidence. Scaffold subsystem validators remain out of CI scope. |
| Creator SDK runtime launch | ALPHA | `play-local` uses repo-root offline cargo workspace (not `/tmp`); fails with actionable vendor hints when offline resolution breaks. |
