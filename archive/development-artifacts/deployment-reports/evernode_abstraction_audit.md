# EverNode Abstraction Audit

## Summary

EverNode is conceptually abstracted behind the control plane: leases, deployment orchestration, health gates, rollback, metrics, and registry publication are represented. However, much of the live EverNode layer is modeled rather than integrated with a real EverNode host process/API. The current posture is appropriate for final pre-live audit, not yet sufficient for unattended lease-per-game hosting.

## What operators still must do manually

- Provision VM/host prerequisites, system packages, filesystem permissions, users, firewall rules, and secrets.
- Install and configure EverNode/HotPocket runtime processes.
- Map generated deployment artifacts into the actual EverNode contract/runtime deployment flow.
- Configure networking, DNS, TLS, ports, and federation peer reachability.
- Validate disk capacity and retention policies for replay/checkpoint growth.
- Operate XRPL/Xahau wallets and signing infrastructure outside EverArcade.
- Execute incident response when real process state diverges from modeled control-plane state.

## What can be automated with current structure

- Package generation and reproducibility metadata.
- Package/rustrig hash validation and deployment manifest generation.
- Lease allocation decisions in the control-plane model.
- Runtime supervision state transitions in the model.
- Health gate evaluation based on modeled metrics.
- Rollback decision recording.
- Operator recovery plans and runbook-driven restore flows.
- Registry publication of package/runtime/deployment metadata.

## What is only modeled

- Real EverNode lease purchase/renewal/release.
- Real HotPocket contract installation and state directory mounting.
- Real process supervisor integration beyond the in-memory/runtime model.
- Real host resource telemetry and cost metering.
- Real cross-machine federation membership and peer authentication.
- Real anchor publication to XRPL/Xahau.
- Real lease-per-game billing, renewal, and eviction policy.

## Integration needed for live EverNode

- A provider adapter that translates control-plane deployment intents into real EverNode/HotPocket actions.
- Idempotent host bootstrap with versioned prerequisites and state directory creation.
- Process manager integration for start/stop/restart/status/log retrieval.
- Lease API integration for capacity discovery, allocation, renewal, failure handling, and release.
- Artifact upload/install/verify hooks with content-addressed package checks.
- Health polling from real runtime endpoints and host metrics.
- Recovery automation that can restore a checkpoint and replay window on a fresh host.
- Secure secret boundary for settlement service credentials outside HotPocket authority.

## Lease-per-game hosting requirements

- One lease record per game deployment, with package hash, rustrig set hash, runtime version, state root, replay cursor, and checkpoint cursor.
- Capacity planner that compares requested CPU/memory/storage/bandwidth against live EverNode inventory.
- Renewal and expiration policy tied to checkpoint handoff and package archival.
- Cost model per lease with replay/checkpoint storage growth and bandwidth estimates.
- Automated drain/migrate flow before lease expiration or node failure.
- Public deployment status that reports package verified, runtime healthy, replay advancing, checkpoint fresh, and anchor publication pending/settled.

## Launch blocker classification

The abstraction is adequate for dry-run and operator rehearsal. Live deployment remains blocked on real EverNode process/API integration, host bootstrap idempotency, real telemetry, and settlement-service separation.
