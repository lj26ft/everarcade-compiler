# Script Surface Audit

Audit date: 2026-05-30

## Summary

- `scripts/` contains 335 shell scripts.
- 236 scripts invoke Cargo directly.
- 162 Cargo-invoking scripts do not set `CARGO_BUILD_JOBS` themselves.
- 2 scripts contain explicit placeholder/stub language: `scripts/restore_runtime.sh` and `scripts/run_full_runtime_validation.sh`.
- Required fresh-VM scripts exist: `vendor_deps.sh`, `run_creator_pipeline_validation.sh`, `run_protocol_readiness_validation.sh`, `run_launch_readiness_validation.sh`, `run_runtime_surface_audit.sh`, and `run_security_validation.sh`.

## Fresh-VM critical scripts

| Script | Status | Flag behavior | CARGO_BUILD_JOBS | Fresh-VM recommendation |
| --- | --- | --- | --- | --- |
| `scripts/vendor_deps.sh` | Usable | Regenerates vendor when missing and validates `cargo metadata --offline --locked`. | Cargo metadata only. | Keep as first gate after lockfile changes. |
| `scripts/preflight_vendor.sh` | Usable | Validates vendored config/source state. | Not applicable. | Keep strict and fast. |
| `scripts/run_creator_pipeline_validation.sh` | Usable | Hard-codes `--offline --locked`. | Uses `CARGO_BUILD_JOBS=1`. | Keep launch-critical. |
| `scripts/run_protocol_readiness_validation.sh` | Usable | Hard-codes `--offline --locked`. | Uses `CARGO_BUILD_JOBS=1`. | Keep launch-critical. |
| `scripts/run_launch_readiness_validation.sh` | Usable | Hard-codes `--offline --locked`. | Uses `CARGO_BUILD_JOBS=1`. | Keep launch-critical. |
| `scripts/run_runtime_surface_audit.sh` | Validated scaffold | Accepts but ignores flags because it writes a report and does not invoke Cargo. | Not applicable. | Document as report-only. |
| `scripts/run_security_validation.sh` | Usable | Forwards caller flags to each Cargo invocation. | Now exports default `CARGO_BUILD_JOBS=1`. | Keep required security gate; do not broaden in this pass. |

## Missing scripts

No required script from the requested validation list is missing.

## Duplicate script risk

The script directory has many domain-specific validation wrappers with overlapping names and behavior, especially around runtime, replay, federation, recovery, deployment, and validation. This creates discoverability and maintenance risk but was not bulk-refactored in this pass.

## Placeholder scripts

- `scripts/restore_runtime.sh` — placeholder/stub-class operational surface.
- `scripts/run_full_runtime_validation.sh` — explicit placeholder report path; should not be treated as a full production runtime validation.

## Scripts that ignore flags

- `scripts/run_runtime_surface_audit.sh` accepts `--offline --locked` in the required command but does not parse or use flags because it only writes `deployment/reports/runtime_surface_audit_report.md`.
- Several simple report-generation scripts follow a similar pattern; this is acceptable when no Cargo/network command is invoked, but each report should state that it is a model/scaffold audit rather than a live validation.

## Scripts that do not use `CARGO_BUILD_JOBS=1`

Many Cargo scripts omit `CARGO_BUILD_JOBS`. This audit intentionally fixed only the required fresh-VM security gate by exporting `CARGO_BUILD_JOBS=${CARGO_BUILD_JOBS:-1}` in `scripts/run_security_validation.sh`. Bulk editing all scripts would violate the minimal-diff policy.

High-priority future candidates include:

- release/fresh-VM scripts: `release_validate.sh`, `release_validate_fresh_vm.sh`, `run_release_validation.sh`, `run_distribution_validation.sh`.
- runtime-heavy scripts: `run_runtime_stress_validation.sh`, `run_release_candidate_validation.sh`, `run_runtime_replay_validation.sh`, and scripts containing `network`, `federation`, `live`, `stress`, `soak`, or `workspace`.
- host/operator/deployment scripts that build or test large surfaces.

## Scripts that should be fresh-VM safe

- `vendor_deps.sh`
- `preflight_vendor.sh`
- `run_creator_pipeline_validation.sh`
- `run_protocol_readiness_validation.sh`
- `run_launch_readiness_validation.sh`
- `run_runtime_surface_audit.sh`
- `run_security_validation.sh`
- `run_studio_gui_validation.sh`
- `run_sdk_validation.sh`
- `run_template_validation.sh`

## Scripts that should require `--offline --locked --frozen`

Release, distribution, runtime package, security, and fresh-VM scripts should prefer strict flags. Current required security validation is invoked as `bash scripts/run_security_validation.sh --offline --locked --frozen`; release packaging scripts already use strict frozen/offline patterns in several places. Future cleanup should standardize these through a shared harness instead of copy-pasting flags into hundreds of scripts.

## Recommended next action

Create a shared `scripts/lib/cargo_validation.sh` helper later that sets `CARGO_BUILD_JOBS=${CARGO_BUILD_JOBS:-1}`, validates `vendor/`, and normalizes `--offline --locked --frozen` handling. Do not perform that migration in this audit pass.
