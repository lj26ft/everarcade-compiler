# Public Launch Readiness Audit for Vertical Slice v0.1

Date: 2026-05-30

This audit answers four launch questions:

1. Which repo surfaces are usable versus stub/scaffold?
2. Which documentation is placeholder-grade and needs real operator/developer content?
3. What reorganization would improve long-term project health?
4. What must still be built for the full EverArcade product beyond the user-facing runtime?

## Executive readiness

| area | launch posture | why it matters | v0.1 action |
|---|---|---|---|
| Source install | usable with vendor preflight | Cargo is configured for `vendor/`; missing vendor blocks builds before compile. | Publish one install page and release tarball path; include vendor/bootstrap failure explanation. |
| Product CLI | usable prototype | `new-game`, `start-game`, replay smoke, diagnostics, and packaging are wired. | Keep command set small in public docs; hide or label roadmap/status commands. |
| Host/operator CLI | usable prototype with broad scaffold surface | Core init/run/verify exists, but `everarcade-host` exposes many advanced commands before UX consolidation. | Document the first four commands only for v0.1; move advanced commands to operator reference. |
| Runtime client | usable local demo | Demonstrates deterministic ticks, roots, and replay count. | Keep as developer smoke/demo binary. |
| Renderer/history/federation | scaffold-level | User instruction and code output identify these as non-authoritative scaffolds. | Gate public claims behind projection/replay-only wording and integration tests. |
| SDK/templates | partial | Starter template exists; SDK docs are terse. | Expand creator path around one supported template and one package format. |
| Security posture | pre-audit | Determinism checks exist, but sandboxing, live network, supply-chain, path isolation, and auth boundaries need hardening. | Treat v0.1 public launch as developer preview; add threat model and issue tracker gates. |

## Stub vs usable matrix

| component | status | usable today? | evidence to verify | launch blocker | next build step |
|---|---|---|---|---|---|
| `everarcade` product CLI | prototype | yes, for local template flow | `cargo run -p everarcade-cli -- new-game <id>` then `start-game <id>` | Command list mixes finished flows and roadmap/status surfaces. | Split public command groups into `game`, `dev`, `runtime`, `scaffold` or hide scaffold commands. |
| `new-game` template scaffolding | prototype | yes | Creates `runtime/games/<id>/game.toml` from `templates/topdown-arena`. | Generated game is not yet a full playable packaged product. | Add `everarcade new --template topdown-arena --run` happy path. |
| `start-game`/`run-game` | prototype | yes | Seeds `runtime/world/status.txt` and `runtime/replay/latest/frame-0001.json`. | Starts a smoke runtime, not a real interactive hosted session. | Connect template code, runtime-client input, and web-reference client into one session loop. |
| Asset commands | prototype | partial | `asset-register`, `asset-build`, `asset-verify` write deterministic marker files. | No real asset import, transform, or content hash policy. | Define asset manifest schema, validation, hashing, and import errors. |
| CLI package hash | prototype | partial | `package-game` hashes `game.toml`. | Package hash excludes source/assets/config and is not full canonical package format. | Implement canonical package archive builder/validator. |
| Host init/run/verify | prototype | yes | `everarcade-host init`, `generate-fixture`, `run`, `verify`. | Operator docs too broad; security model not public-audit ready. | Promote four-command host flow and quarantine advanced commands. |
| Execution core | production-useful prototype | yes | Existing extensive tests and runtime API status commands. | Public API ownership needs stabilization. | Freeze v0.1 public API surface and generate API compatibility report. |
| WASM/contract execution | prototype | partial | Contract crates and host execute/list/inspect commands exist. | ABI docs and isolation guarantees are not complete enough for untrusted public games. | Complete sandbox/resource threat model and ABI conformance tests. |
| Replay diagnostics | prototype | partial | `replay-world`, `inspect-simulation`, replay status commands. | Replay docs contain placeholders and duplicate concepts. | Publish one replay verification guide with corruption/recovery examples. |
| Runtime appliance/release scripts | production-useful prototype | yes | Clean bootstrap scripts and runtime appliance docs exist. | Release process depends on vendored deps and path assumptions. | Add CI preflight that fails on missing vendor, absolute paths, or generated artifacts. |
| Renderer client | scaffold | partial | `renderer-client` projection/status commands. | Non-authoritative only; no public renderer product. | Integrate web-reference/renderer projection with real replay windows. |
| History/runtime archive | scaffold | partial | Historical query/status commands are markers. | Many docs are one-line placeholders. | Build index/archive import/export tests and one operator runbook. |
| Federation/topology | scaffold | partial | Status/recovery commands exist; user explicitly classifies as scaffold-level domain. | No real public multiplayer trust/abuse model. | Build two-node deterministic session test with peer auth, partition, and recovery. |
| XRPL/IPFS/Evernode | scaffold/prototype | partial | Optional docs and gated scripts exist. | Live submission uses stub path; external trust, key management, and retry semantics are not ready. | Keep disabled by default; document dry-run only until audited. |
| Studio/creator tooling | scaffold | no for public product | `studio`, `studio-gui`, creator docs are terse. | No cohesive creator app workflow. | Choose CLI-first creator MVP before GUI. |
| Tests in `everarcade-host/tests/*_tests.rs` placeholders | stub | no | Multiple tests assert placeholder invariants. | False confidence for runtime domains. | Replace with real scenario tests or mark ignored/roadmap. |

## Documentation that needs more than placeholders

These docs are short enough or explicit enough to be misleading for public v0.1 if left as-is. They should either be expanded into real guides or moved into an internal roadmap folder.

### Highest priority public docs

| doc | current issue | required content before public launch |
|---|---|---|
| `docs/GAME_PACKAGE_FORMAT.md` | one-line package shape | Canonical manifest schema, required files, hashing rules, validation errors, example package, compatibility versioning. |
| `docs/GAME_DEVELOPER_START.md` | competes with quickstarts | Merge with `CLI_QUICKSTART.md`; provide one happy path and one troubleshooting section. |
| `docs/developer/quickstart.md` | only one sentence | Replace with source install, create/run/package loop, targeted validation, and next links. |
| `docs/sdk/getting-started.md` | too terse for creators | Template anatomy, deterministic input model, tick loop, replay test, packaging. |
| `docs/ASSET_PIPELINE.md` | lists commands only | Asset manifest schema, supported asset types, hashing, deterministic transforms, failure modes. |
| `docs/CLIENT_BRIDGE.md` | lists APIs only | Request/response examples, authority boundary, auth/session model, web-reference integration. |
| `docs/replay-verification.md` | explicit placeholder | Replay truth model, corruption examples, how to inspect/recover, CLI examples. |
| `docs/state-folder-layout.md` | explicit placeholder | `runtime/` tree, which files are protocol truth, which are derived/rebuildable, cleanup rules. |
| `docs/operator-node.md` | explicit placeholder | Minimum host setup, state directory, fixture run/verify, logs, recovery, upgrade path. |
| `docs/xrpl-anchoring.md` | explicit placeholder | Dry-run vs live mode, key handling, payload semantics, retry/idempotency, trust boundary. |

### Runtime placeholder/one-line docs

| doc group | files | required consolidation |
|---|---|---|
| Interactive/live runtime docs | `docs/runtime/interactive_replay.md`, `docs/runtime/interactive_runtime.md`, `docs/runtime/live_gameplay.md`, `docs/runtime/terminal_gameplay.md` | Merge into one interactive runtime guide with terminal client commands, input vocabulary, replay output, and limitations. |
| Renderer/projection docs | `docs/runtime/frame_projection.md`, `docs/runtime/graphical_renderer.md`, `docs/runtime/graphical_replay_validation.md`, `docs/runtime/projection_playback.md`, `docs/runtime/projection_stream_service.md`, `docs/runtime/projection_validation.md`, `docs/runtime/render_projection_law.md`, `docs/runtime/render_validation.md`, `docs/runtime/renderer_bridge.md`, `docs/runtime/terminal_renderer.md`, `docs/runtime/visual_projection_runtime.md`, `docs/runtime/visible_runtime.md` | Consolidate into one renderer authority model plus one projection API reference. |
| Runtime operation docs | `docs/runtime/runtime_bootstrap_flow.md`, `docs/runtime/runtime_operations_architecture.md`, `docs/runtime/runtime_restoration_flow.md`, `docs/runtime/runtime_validation.md`, `docs/runtime/runtime_metrics.md`, `docs/runtime/release_pipeline_architecture.md`, `docs/runtime/evernode_runtime_model.md`, `docs/runtime/operational_ledger_model.md` | Create one operator runtime manual and one release pipeline guide. |
| Sovereign placeholders | `docs/runtime/civilization_operations_model.md`, `docs/runtime/deployment_continuity_model.md`, `docs/runtime/federation_coordination_model.md`, `docs/runtime/release_and_upgrade_model.md`, `docs/runtime/runtime_governance_model.md`, `docs/runtime/runtime_restoration_model.md`, `docs/runtime/sovereign_deployment_architecture.md` | Either write real architecture docs or move to `docs/roadmap/sovereign/`. |
| WASM model fragments | `docs/runtime/abi_and_memory_model.md`, `docs/runtime/checkpoint_and_restore_model.md`, `docs/runtime/deterministic_wasm_runtime.md`, `docs/runtime/execution_receipt_model.md`, `docs/runtime/fuel_metering_model.md` | Combine around ABI, memory, fuel, receipts, restore, and conformance tests. |

### Security placeholder docs

| doc | missing launch-critical content |
|---|---|
| `docs/security/adversarial_validation.md` | Concrete adversarial cases, command/test mapping, expected deterministic rejection outputs. |
| `docs/security/capability_isolation.md` | Capability inventory, deny-by-default policy, file/network/process boundaries. |
| `docs/security/resource_governance.md` | Exact quotas for CPU, memory, replay size, package size, asset size, event counts. |
| `docs/security/runtime_failure_taxonomy.md` | Public error taxonomy, severity, operator response, telemetry fields. |
| `docs/security/wasm_isolation_model.md` | Host imports allowed/denied, memory limits, fuel metering, panic/trap semantics. |
| `docs/security/external_audit_scope.md` | Scope, exclusions, invariants, reproducible build inputs, audit artifacts. |

## Reorganization needed for long-term health

| repo area | problem | recommended reorganization |
|---|---|---|
| Crate layout | Product CLI lives in `src-bin-everarcade`; host/runtime crates are spread across root, `runtime/`, and `tools/`. | Move to `crates/everarcade-cli`, `crates/everarcade-host`, `crates/runtime-client`, `crates/renderer-client`, with compatibility package names preserved. |
| Runtime domains | Renderer/history/federation commands are interleaved with runtime and product CLI surfaces. | Create explicit `runtime-authoritative/` and `runtime-projection/` ownership boundaries; keep scaffold domains behind feature flags or `experimental` commands. |
| Docs | Many one-line docs duplicate terms and make public maturity unclear. | Create `docs/getting-started/`, `docs/reference/`, `docs/operator/`, `docs/security/`, `docs/roadmap/`; move placeholders to roadmap. |
| Tests | Placeholder host tests can imply coverage where none exists. | Replace placeholders with scenario tests or rename to `roadmap_*` with `#[ignore]` and linked issue IDs. |
| Scripts | Release, validation, bootstrap, and profile scripts are numerous and overlapping. | Add `scripts/README.md` and a single `scripts/doctor_quick.sh`/`scripts/release_gate.sh` entrypoint with delegated subcommands. |
| Templates/examples | `templates/`, `examples/`, `creator-examples/`, and contracts overlap for new users. | Promote one canonical `templates/topdown-arena` v0.1 path; move exploratory examples to `examples/experimental`. |
| Generated/runtime artifacts | Repo root contains many runtime status-looking files and report folders. | Enforce generated artifact ignore rules and move committed fixtures under `fixtures/` with README ownership. |
| Public API | `execution-core` is broad and many surfaces are evolving. | Maintain `execution_core::api` as stable public facade and treat all other modules as internal unless documented. |

## Security risks before public v0.1

| risk | severity | current signal | mitigation before launch |
|---|---|---|---|
| Untrusted package execution boundary unclear | high | Package format and WASM isolation docs are incomplete. | Publish supported package format, deny arbitrary host IO, document allowed imports, add conformance tests. |
| Path traversal / unsafe copy semantics in CLI install flow | medium-high | `install-game` recursively copies a user path into `runtime/games/<folder-name>` without documented symlink/path policy. | Canonicalize source, reject symlinks or external traversal for packaged imports, size-limit copies. |
| Supply-chain/bootstrap confusion | high | Build fails when `vendor/` is missing; release reproducibility depends on vendored deps. | Publicly document vendor requirement, publish signed checksums, CI verify `--offline --locked --frozen`. |
| Live XRPL/IPFS trust boundary | high | XRPL live mode is gated, but submission remains stub-like; IPFS publication is derived. | Keep live mode disabled in public CLI, require explicit feature/env flag, add key/secret handling docs. |
| Scaffold commands imply production readiness | medium-high | Runtime status commands print healthy/scaffold markers for domains that are not complete. | Label scaffold commands in help/docs; remove from default public quickstart. |
| Resource exhaustion | medium-high | Security docs mention quotas but public limits are not enumerated. | Enforce and document max package size, replay window size, asset size, memory/fuel/tick quotas. |
| False test confidence | medium | Placeholder tests exist in host runtime domains. | Replace with actual tests for federation/recovery/governance or mark them ignored roadmap tests. |
| Secrets and state handling | medium | Operator docs do not define key storage, permissions, or redaction. | Add state directory permissions, secret redaction, no-secrets-in-logs policy. |
| Network/multiplayer abuse model missing | medium | Federation/topology are scaffold-level. | Do not launch public federation; build peer auth, rate limits, replay corruption rejection, partition recovery tests. |
| Renderer authority confusion | medium | Renderer commands exist but are non-authoritative. | Enforce client command-only boundary in docs and code; add tests proving renderer cannot mutate state. |

## Full EverArcade product build-out backlog

### Must-have for vertical slice v0.1 public developer preview

- One installable CLI binary release with checksums and a `doctor` command.
- One canonical starter game template that can be created, run, replay-verified, and packaged.
- One public package format with a validator and deterministic package hash over all required files.
- One local gameplay loop that accepts player input and persists replay/checkpoint artifacts.
- One web or terminal client path documented end-to-end.
- One replay inspector with a corruption/recovery example.
- Security baseline: package size limits, deterministic rejection errors, no live external settlement by default.
- CI release gate for formatting, targeted crate checks/tests, clean bootstrap, generated artifact scan, and docs link check.

### Needed for complete EverArcade product

| product area | build-out needed |
|---|---|
| Creator SDK | Stable Rust SDK, template generator, deterministic test harness, asset import pipeline, publishing validator. |
| Game runtime | Real session lifecycle, input queue, tick scheduler, persistence, checkpoint restore, replay verification, crash recovery. |
| Client experience | Download page, launcher, web-reference client, terminal fallback, readable errors, update flow. |
| Renderer/projection | Non-authoritative renderer protocol, frame stream, historical replay, asset loading, visual validation. |
| Package registry | Local registry, signatures, compatibility rules, content-addressed artifacts, rollback and quarantine. |
| Multiplayer/federation | Peer identity, auth, deterministic input ordering, topology, partition recovery, observer mode, abuse/rate limits. |
| Operator/node | Service install, systemd/container profile, logs/metrics, backup/restore, upgrade/migration, health checks. |
| Economy/settlement | Dry-run ledger first, XRPL live adapter later, key management, idempotent anchors, settlement proofs, dispute flow. |
| Studio | Project browser, template creation, asset validation, replay debugger, package publish UI, diagnostics. |
| Observability | Structured logs, trace IDs, replay roots in telemetry, metrics dashboard, support bundle. |
| Security/compliance | Threat model, external audit, fuzzing, sandboxing, dependency review, disclosure policy. |
| Governance/community | Version support policy, RFC process, contribution guide, issue templates, roadmap maturity labels. |

## Recommended v0.1 launch promise

Launch as a **developer preview for deterministic local game creation and replay verification**, not as a complete multiplayer/federated/settlement product. The public promise should be:

> Install the CLI, create a starter deterministic game, run it locally, inspect replay output, and package it reproducibly.

Everything else should be clearly marked experimental, scaffold, or roadmap until it has end-to-end tests and operator documentation.
