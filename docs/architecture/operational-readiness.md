# EverArcade Operational Readiness Assessment

Canonical assessment date: 2026-06-03

## Overall operational score

**Operational Readiness: 34 / 100**  
**Classification: Scaffold**

EverArcade has substantial operational vocabulary, documents, runbooks, scripts, and status modules. It is not yet operationally ready for unattended commercial service because real monitoring, alerting, backup automation, incident response, disaster recovery drills, and clean-host deployment evidence are incomplete.

## Installation

**Status: Experimental**

### Implemented

- Install/bootstrap scripts and Linux VM docs exist.
- Operator install/bootstrap/deploy docs exist.
- Systemd service template and operator config templates exist.

### Gaps

- No single production installer validated from signed release artifacts.
- Source-tree and vendored-dependency assumptions remain prominent.
- No `doctor` command that verifies environment, package signatures, storage paths, permissions, and runtime health end to end.

### v0.1 requirement

A clean host must install, verify, start, stop, and remove/rollback EverArcade through one documented path.

## Upgrade process

**Status: Scaffold**

### Implemented

- Operator upgrade/rollback documentation exists.
- Release packaging and validation scripts exist.

### Gaps

- No automated pre-upgrade backup, schema migration, replay/checkpoint compatibility check, or rollback drill.
- No versioned persistence migration framework identified as production path.

### v0.1 requirement

Upgrade must be blocked unless backup, migration dry-run, replay/checkpoint verification, and rollback plan pass.

## Backup process

**Status: Scaffold**

### Implemented

- Persistence artifacts are separated into receipts, journal, and checkpoints.
- Archive/replay/checkpoint documents and reports exist.

### Gaps

- No canonical backup command, schedule, retention policy, off-host target, encryption policy, or backup integrity report.
- No automated restore test from backup.

### v0.1 requirement

Add `everarcade backup` or operator script that snapshots world root, verifies receipts/journal/checkpoints, stores backup manifest, and supports restore rehearsal.

## Recovery process

**Status: Experimental**

### Implemented

- Checkpoint restore and recovery modules/tests exist.
- Operator recovery docs and machine recovery/rejoin runbooks exist.
- Scripts exist for restore, local cluster recovery, Linux VM recovery, failover, network partition, and resumable sync.

### Gaps

- Recovery is not yet a single machine-readable workflow with status, logs, and release gate integration.
- Recovery evidence is mainly local/test/synthetic rather than live lease/host failure proof.

### v0.1 requirement

Recovery drill must kill a running world, restore from checkpoint, replay to tip, verify root equivalence, and emit a pass/fail report.

## Monitoring

**Status: Scaffold**

### Implemented

- Runtime/node modules include health, metrics, dashboard, watchdog, supervisor, daemon, and lifecycle concepts.
- Observability reports define required metrics, logs, alerts, health states, recovery signals, replay growth, checkpoint age, lease capacity, and cost signals.

### Gaps

- No production metrics exporter, dashboard, alert destination, threshold policy, or service-level objective.
- Metrics are not consistently tied to running service lifecycle.

### v0.1 requirement

Expose minimum health/status: package hash, runtime version, replay cursor/root, checkpoint id/age/root, latest receipt, process state, disk usage, and recovery state.

## Observability

**Status: Scaffold**

### Implemented

- Structured operational concepts are documented.
- Validation scripts for observability exist and some run targeted tests.

### Gaps

- No durable log export, rotation, redaction, correlation ids, support bundle, or alert/runbook links.
- Consensus-safe metrics and host telemetry separation is documented but not wired end to end.

### v0.1 requirement

Add JSON logs outside consensus authority with world id, package hash, replay/checkpoint ids, receipt ids, operation id, and error code.

## Incident response

**Status: Scaffold**

### Implemented

- Runbooks exist for checkpoint restore, machine recovery, machine rejoin, machine startup, and transport failure.

### Gaps

- No severity model, escalation path, on-call checklist, incident timeline template, postmortem template, or runbook-linked alerts.
- No support bundle command for collecting safe diagnostics.

### v0.1 requirement

Create incident workflow for runtime down, checkpoint restore failed, replay divergence, disk full, package verification failed, and lease restart failed.

## Disaster recovery

**Status: Scaffold**

### Implemented

- Checkpoint/replay concepts exist.
- Recovery and restoration modules exist.

### Gaps

- No off-host backups, region/provider recovery plan, restore-time objective, restore-point objective, or disaster drill.
- Evernode lease disaster behavior is unknown.

### v0.1 requirement

Define RPO/RTO for developer-preview operation and prove restore from backup onto a clean host.

## Runbooks

**Status: Experimental**

### Implemented

- Runbook files exist for checkpoint restore, machine recovery, machine rejoin, startup, and transport failure.
- Operator docs exist for install, deploy, recover, rollback, and upgrade.

### Gaps

- Runbooks need concrete commands, expected outputs, decision trees, rollback steps, and links to alerts/status fields.
- Runbooks need regular drill evidence.

### v0.1 requirement

Every absolute release blocker must have a runbook and a drill report.

## Operator documentation

**Status: Experimental**

### Implemented

- Operator and deployment docs are broad and cover many desired workflows.

### Gaps

- Documentation breadth can obscure the supported v0.1 path.
- Scaffold commands and advanced domains need maturity labels.
- External operators need one short path, not many overlapping scripts/reports.

### v0.1 requirement

Publish a single operator quickstart for the v0.1 runtime and move advanced/scaffold material behind explicit labels.

## Minimum operations checklist for v0.1

| Capability | Required before v0.1? | Current posture |
| --- | --- | --- |
| Install from signed artifact | Yes | Partial/scaffold |
| Start/stop/restart service | Yes | Partial |
| Health/status command | Yes | Partial/scaffold |
| Append-only replay persisted | Yes | Partial |
| Periodic checkpoint persisted | Yes | Partial |
| Forced restart recovery | Yes | Partial |
| Restore rehearsal gate | Yes | Missing as canonical gate |
| Backup command and manifest | Yes | Missing |
| Rollback after failed upgrade | Yes | Scaffold |
| Metrics/log export | Yes, minimal | Scaffold |
| Alerts | No for developer preview, yes before paid customers | Scaffold |
| Disaster recovery drill | Before paid customers | Missing |
| Live Evernode lease recovery | Required for Evernode-hosted v0.1 | Missing/proven false by absence |

## Commercial operations blockers

1. Monitoring, logging, and alert routing are not production-wired.
2. Backups and restore drills are not automated.
3. Upgrade/rollback safety is not proven.
4. Evernode live lease lifecycle is not validated.
5. Support/incident workflows are not defined.
6. Security operations for packages, keys, secrets, and external settlement are incomplete.

## Recommended operational release stance

- **v0.1 developer preview:** acceptable if single-world deploy/recover gate passes and limitations are explicit.
- **Paid customer beta:** blocked until monitoring, backup, restore drills, upgrade/rollback, support workflows, and security hardening are complete.
- **Public commercial launch:** blocked until live Evernode/host operations, incident response, disaster recovery, and settlement/marketplace boundaries are production-proven.
