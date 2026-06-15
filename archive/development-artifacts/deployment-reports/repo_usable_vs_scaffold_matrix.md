# Repository Usable-vs-Scaffold Matrix

Audit date: 2026-05-30

Status legend: `ProductionCandidate`, `UsableInternal`, `ValidatedScaffold`, `Placeholder`, `Broken`, `Deprecated`.

## execution-core

- **Status:** `UsableInternal`
- **Evidence:** Workspace member with a large deterministic-runtime test surface, including `creator_pipeline_runtime_tests`, security runtime tests, simulation/runtime tests, replay tests, federation tests, and XRPL/EverNode readiness tests. Targeted offline tests are used by launch/protocol/security scripts.
- **Validation coverage:** Targeted package integration tests; creator pipeline runtime tests; runtime surface audit; security validation; protocol readiness validation.
- **Known gaps:** Federation, live networking, history, and deployment surfaces are heavily modeled and test-driven but should not be treated as a proven live production backend. Full workspace sensitivity remains high because the test matrix is very broad.
- **Recommended next action:** Keep execution semantics stable; continue using targeted gates for fresh-VM validation; separate production-critical deterministic runtime tests from scaffold/runtime-roadmap tests in future organization work.

## everarcade-sdk

- **Status:** `ValidatedScaffold`
- **Evidence:** SDK crates are workspace members under `sdk/`; `scripts/run_sdk_validation.sh` checks for core SDK files and scans for obvious non-deterministic primitives.
- **Validation coverage:** SDK validation script and SDK/runtime integration tests under `sdk/tests` and `execution-core/tests/sdk_tests.rs`.
- **Known gaps:** API maturity and compatibility guarantees are not yet established as production SDK contracts; multiple specialized SDK crates exist but are not organized under a long-term `crates/sdk` layout.
- **Recommended next action:** Freeze a narrow public SDK surface, document versioning guarantees, and move SDK crates only in a later reorganization milestone.

## studio-gui

- **Status:** `UsableInternal`
- **Evidence:** Workspace member with local `eframe`/`egui` path dependencies and targeted creator/publish/template tests used by launch readiness scripts.
- **Validation coverage:** `cargo test -p studio-gui --offline --locked`, launch readiness creator workflow filters, and studio GUI validation script.
- **Known gaps:** GUI implementation is validated as deterministic workflow logic, not as a polished end-user desktop app. No screenshot/manual UI certification was performed in this pass.
- **Recommended next action:** Preserve targeted tests as launch gates; add manual UX acceptance criteria separately from deterministic workflow validation.

## tools

- **Status:** `UsableInternal`
- **Evidence:** Workspace member with `creator_toolchain_tests.rs`; vertical-slice tests now compile and pass offline with the vendored dependency set.
- **Validation coverage:** Vertical slice filter, full `tools` package tests, creator pipeline validation, protocol readiness validation.
- **Known gaps:** Tooling mixes creator workflow, certification, protocol readiness, and report-generation concerns in one crate. This is acceptable for internal use but should be split only in a later reorganization.
- **Recommended next action:** Keep targeted creator tests small and launch-critical; avoid adding unrelated runtime features to this crate.

## runtime/renderer-client

- **Status:** `ValidatedScaffold`
- **Evidence:** Workspace member under `runtime/renderer-client`; runtime surface audit classifies renderer history, federation, and transport runtime as scaffold-level surfaces.
- **Validation coverage:** Cargo metadata and offline vendoring; runtime surface audit report.
- **Known gaps:** Renderer authority, history, and federation are not production runtime authority. Treat this as a deterministic client/scaffold boundary, not as a validated live renderer platform.
- **Recommended next action:** Document renderer authority boundaries and keep renderer/history/federation separate from deterministic execution authority.

## everarcade-cli

- **Status:** `ValidatedScaffold`
- **Evidence:** CLI crate is represented by workspace member `src-bin-everarcade`; root-level tests include CLI/bootstrap/onboarding checks.
- **Validation coverage:** Root integration tests and scripts reference CLI/runtime aliases, doctor output, bootstrap paths, and release/runtime packaging.
- **Known gaps:** CLI is not included in the minimal fresh-VM targeted gates in this pass; full CLI behavior is likely full-workspace sensitive.
- **Recommended next action:** Add a small offline locked CLI smoke gate after the current fresh-VM blockers remain green.

## scripts

- **Status:** `UsableInternal`
- **Evidence:** `scripts/` contains hundreds of validation and operation scripts. Fresh-VM critical scripts exist for vendoring, creator pipeline, protocol readiness, launch readiness, runtime surface audit, and security validation.
- **Validation coverage:** Required scripts are executed directly in this pass. Script audit documents scripts that ignore flags, omit `CARGO_BUILD_JOBS=1`, or are placeholders.
- **Known gaps:** Many scripts are duplicated by domain and many cargo scripts do not set `CARGO_BUILD_JOBS=1`; some scripts intentionally act as report placeholders.
- **Recommended next action:** Create a shared validation harness later; do not bulk-edit all scripts in this audit pass.

## deployment/reports

- **Status:** `UsableInternal`
- **Evidence:** Existing report directory contains many milestone reports and now includes repository status, completion, test surface, script surface, and reorganization audit reports.
- **Validation coverage:** Reports are file-backed and generated/updated by scripts such as vendor validation and runtime surface audit.
- **Known gaps:** Reports are numerous and not consistently indexed; several represent scaffold readiness rather than live deployment evidence.
- **Recommended next action:** Add an index and distinguish generated validation reports from planning/audit reports.

## templates

- **Status:** `ValidatedScaffold`
- **Evidence:** Template crates and `game.toml` surfaces exist for topdown arena, persistent world, turn-based, simulation world, and cooperative session flows.
- **Validation coverage:** Template validation script checks required template files; launch readiness covers template generation equivalence.
- **Known gaps:** Templates are not certified as production games; they should be treated as reproducible starter surfaces.
- **Recommended next action:** Keep templates minimal, deterministic, and documented as examples until a template release checklist exists.

## examples

- **Status:** `Placeholder`
- **Evidence:** Multiple example directories and several example `Cargo.toml` files exist, but examples are not included in the root workspace and were not part of the required fresh-VM gates.
- **Validation coverage:** Not covered by the required targeted commands in this pass.
- **Known gaps:** Example freshness is uncertain; some examples may lag current SDK/runtime APIs.
- **Recommended next action:** Add an optional examples validation gate after fresh-VM core validation is stable.

## vendor/bootstrap

- **Status:** `ProductionCandidate`
- **Evidence:** `scripts/vendor_deps.sh` regenerates `vendor/`, writes `.cargo/config.toml` for vendored sources, validates `cargo metadata --offline --locked`, and now includes `zerocopy` and `zerocopy-derive` 0.8.49 required by `Cargo.lock`.
- **Validation coverage:** `bash scripts/vendor_deps.sh`, offline locked cargo metadata, and all targeted offline locked cargo tests.
- **Known gaps:** Vendored directory is large and should be refreshed deliberately when `Cargo.lock` changes.
- **Recommended next action:** Treat vendor refresh as a required lockfile-change step and keep `preflight_vendor.sh` strict.
