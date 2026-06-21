# Runtime Platform Proof Chain

This is the authoritative v0.1 runtime maturity sequence. Each stage narrows the claim to local, reproducible evidence; none of these stages imply production readiness.

```text
Package Proven
↓
Runtime Boot Proven
↓
Deterministic Execution Proven
↓
Template Gameplay Proven
↓
WASM Guest Execution Proven
↓
Playable Local Game Proven
↓
Multiplayer Local Session Proven
↓
Network Transport Proven
↓
Federated Runtime Synchronization Proven
↓
Multi-Lease Civilization Runtime Proven
```

| Stage | Purpose | Validation script | Certification script | Evidence produced |
| --- | --- | --- | --- | --- |
| Package Proven | Proves a Creator SDK project can produce a runtime package with manifest, world metadata, and world payload. | `node creator-sdk/cli/everarcade.mjs package --project <project>` and package checks inside `scripts/validate_developer_onboarding.sh` | Covered by `scripts/certify_developer_experience.sh` for onboarding; historical package flows use `scripts/run_package_certification_validation.sh` | `<project>/dist/runtime-package/manifest.json`, `world.json`, `world.wasm` |
| Runtime Boot Proven | Proves the runtime can start against a packaged world and write boot/session evidence. | `scripts/validate_runtime_platform.sh`, `scripts/validate_runtime_appliance.sh`, or the runtime start path exercised by Creator SDK commands | `scripts/certify_runtime_bootstrap.sh` | Runtime bootstrap report, health marker, session/runtime-root files |
| Deterministic Execution Proven | Proves deterministic execution can produce replay-verifiable state evidence. | `scripts/validate_deterministic_execution.sh` | `scripts/certify_deterministic_execution.sh` | `reports/deterministic_execution_validation_report.txt`, deterministic execution certification report |
| Template Gameplay Proven | Proves the Arena template gameplay path mutates state and verifies replay. | `scripts/validate_template_gameplay_execution.sh` | `scripts/certify_template_gameplay_execution.sh` | `reports/template_gameplay_validation_report.txt`, template gameplay certification report |
| WASM Guest Execution Proven | Proves a WASM guest can be built, packaged, loaded, executed, and replay-verified. | `scripts/validate_wasm_guest_execution.sh` | `scripts/certify_wasm_guest_execution.sh` | `reports/wasm_guest_execution_validation_report.txt`, guest receipts/journals/replay proof |
| Playable Local Game Proven | Proves a local Arena session can join, move, attack, update score, advance ticks, emit receipts/journals/transcripts, and verify replay. | `scripts/validate_playable_local_game.sh` and `scripts/validate_developer_onboarding.sh` | `scripts/certify_playable_local_game.sh` and `scripts/certify_developer_experience.sh` | `reports/playable_local_game_validation_report.txt`, `reports/developer_onboarding_validation_report.txt`, runtime-root session/gameplay/receipt/journal/transcript/replay files |
| Multiplayer Local Session Proven | Proves two local players can share one authoritative runtime state with deterministic receipts, journals, transcripts, and replay. | `scripts/validate_multiplayer_local_session.sh` | `scripts/certify_multiplayer_local_session.sh` | `reports/multiplayer_local_session_validation_report.txt`, runtime-root multiplayer gameplay evidence |
| Network Transport Proven | Proves local client identities can synchronize through deterministic transport messages while preserving authoritative runtime replay equivalence. | `scripts/validate_network_transport_session.sh` | `scripts/certify_network_transport_session.sh` | network transport state observations, transport log, receipt delivery, journal, and replay evidence |
| Federated Runtime Synchronization Proven | Proves two independent local runtime authorities can synchronize deterministic state through checkpoint, receipt, and journal exchange with divergence detection, recovery, and federation replay verification. | `scripts/validate_federated_runtime_sync.sh` | `scripts/certify_federated_runtime_sync.sh` | `reports/federated_runtime_validation_report.txt`, `reports/federated_runtime_certification_report.txt`, local federation evidence under `federation/` |
| Multi-Lease Civilization Runtime Proven | Proves one local civilization can survive deterministic lease transition and simulated lease failure across two independent lease-style runtime authorities while preserving economy, inventory, recovery, and replay continuity. | `scripts/validate_multi_lease_civilization.sh` | `scripts/certify_multi_lease_civilization.sh` | `reports/multi_lease_civilization_validation_report.txt`, `reports/multi_lease_civilization_certification_report.txt`, local civilization evidence under `civilization/` |

## Interpretation rules

- PASS means the named local proof passed under the script's documented assumptions.
- PASS does not mean production ready, public-testnet ready, commercially ready, or adversarially secure.
- Scaffold domains must not be promoted by association with the local runtime proof chain.
