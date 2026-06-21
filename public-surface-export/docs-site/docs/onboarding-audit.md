# Onboarding Audit (Linux VM, 2026-05-23)

## Can a new developer answer the basics today?

- How do I build it? **Partially** (scripts exist, but no root README).
- How do I run it? **Partially** (`scripts/start.sh`, host CLI paths exist, but entry points scattered).
- How do I validate it? **Mostly yes** (`scripts/validate.sh`, `scripts/validate_clean_vm_bootstrap.sh`).
- How do I package it? **Yes** (`scripts/build_runtime_release.sh`).
- How do I run deterministic appliance? **Yes-ish** (runtime release + bootstrap docs exist, but fragmented).
- How do I add a module/contract? **Not clearly** (templates and contract crates exist, lacking canonical path doc).
- How do I know production vs demo? **No** (no authoritative matrix at root).

## Main pain points
1. Missing root `README.md`.
2. Immediate cargo failure in fresh clone because `.cargo/config.toml` requires `vendor/`.
3. Multiple overlapping docs with uncertain canonical version.
4. Path naming mismatch (`src-bin-everarcade` package is `everarcade-cli`) raises confusion.
5. Very large script surface without a beginner route map.

## Proposed Happy Path (first-run)
```bash
git clone <repo>
cd everarcade-compiler
bash scripts/vendor_deps.sh
cargo test --workspace
bash scripts/build_runtime_release.sh
bash scripts/validate_clean_vm_bootstrap.sh
```

## Onboarding fixes before v0.1.x
- Add concise root README with “5-command quickstart”.
- Add `scripts/doctor_quick.sh` callout as first diagnostic command.
- Add explicit “production-useful vs prototype/stub” matrix link.
- Normalize docs into `docs/onboarding/` and `docs/runtime/` hierarchy (see reorg plan).
