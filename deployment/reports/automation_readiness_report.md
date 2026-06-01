# Automation Readiness Report

## Summary

EverArcade is ready for repeatable dry-run automation and targeted validation. It is not yet ready for unattended live EverNode deployment without real provider integration, live telemetry, external settlement service wiring, and fresh-VM recovery rehearsal.

## Readiness matrix

| Area | Readiness | Evidence | Remaining work |
| --- | --- | --- | --- |
| Release automation | Medium/high | Release reports, package generation scripts, reproducibility manifests, and validation scripts exist. | Add release provenance upload, signed artifact promotion, and fresh-machine release rehearsal. |
| Package generation | High for dry run | EverNode package generation and package verification tests exist. | Enforce one canonical package/rustrig manifest path and archive retention policy. |
| EverNode deployment | Medium for modeled flow; low for unattended live flow | Control-plane orchestration models leases, deployment steps, health verification, registry publication, and rollback. | Implement real EverNode/HotPocket provider adapter and host process integration. |
| Fresh VM validation | Medium | Linux VM/operator docs and scripts exist. | Add one-command fresh VM validation that vendors dependencies, installs prerequisites, runs targeted tests, and verifies state layout. |
| Operator recovery | Medium/high for runbooks; medium for automation | Recovery runbooks and automated recovery plan model exist. | Execute cross-machine restore drills with real checkpoints and archived replay. |
| Cross-machine validation | Medium | Cross-machine reports and federation tests exist. | Automate against live hosts with network partitions, rejoin, checkpoint transfer, and root comparison. |
| XRPL anchor publication | Low/medium | Anchor records and external settlement-required payloads are modeled. | Build external settlement service for signing/submission/retry/settlement and vault lifecycle. |
| Cost reporting | Medium | Cost model reports and lease resources exist. | Connect to real lease prices, storage/bandwidth usage, and per-game attribution. |

## Automation gaps by launch phase

### Before first live deployment

- Real EverNode provider adapter.
- Idempotent host bootstrap and state directory creation.
- Package upload/install/verify against a live EverNode host.
- Runtime health polling from real processes.
- Replay/checkpoint restore drill on a fresh machine.
- External settlement service disabled or in observe-only mode unless fully tested.

### Before unattended operations

- Lease renewal/expiration automation.
- Alert delivery and escalation.
- Cost threshold alerts.
- Automated migration/drain for lease failure or expiration.
- Continuous replay archive verification.
- Settlement retry and reconciliation dashboard.

### Before creator self-serve deployment

- Five-command CLI facade or Studio guided workflow.
- Per-game deployment status with clear health and cost estimate.
- Automated rollback and recovery surfaced as simple states.
- Documentation that separates creator actions from operator custody/deployment duties.

## Final readiness verdict

The architecture is automation-friendly, but live automation is integration-bound. Proceed with bounded refactors and dry-run certification; do not claim fully automated live EverNode hosting until provider, observability, recovery, and settlement integrations have passed fresh-machine and cross-machine validation.
