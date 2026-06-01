# Observability Gap Analysis

## Summary

EverArcade has the core observability vocabulary: runtime metrics, health states, alerts, logs, lease capacity, recovery status, checkpoint age, replay growth, and cost reports. The gap is production wiring: real collection, durable export, alert delivery, dashboards, and runbook-linked remediation.

## Current coverage and gaps

| Area | Current posture | Gap | Launch recommendation |
| --- | --- | --- | --- |
| Metrics | Metrics structs cover runtime, federation, deployment, and player counters. | Need real collectors from runtime, host, HotPocket, EverNode, and settlement workers. | Add a metrics adapter/export endpoint; keep deterministic in-consensus metrics separate from host telemetry. |
| Logs | Structured log store supports deployment, runtime, recovery, operator, lease, and anchor events. | Need durable append/export, redaction, log rotation, and correlation ids across control plane and host processes. | Standardize JSON logs outside authority with replay/checkpoint/deployment ids. |
| Alerts | Alert triggers cover runtime stalled, checkpoint overdue, node lost, partition, deployment failure, recovery failure, and lease exhaustion. | Need thresholds, destinations, dedupe, escalation, and tests against real telemetry. | Wire alerts to operator notification channels and runbook links. |
| Health states | Health states include healthy, warning, critical, and failed. | Need a single health contract exposed by every runtime and deployment. | Publish `/health` or equivalent status with replay cursor, checkpoint age, package hash, and lease id. |
| Recovery signals | Recovery plans and runbooks exist. | Need machine-readable recovery state and automation checkpoints. | Emit recovery started/restored/rejoined/failed events with root comparisons. |
| Replay growth | Replay throughput/growth is modeled. | Need retention alarms, compaction policy, archive upload status, and replay window lag. | Alert on no replay growth while runtime alive and on storage forecast breaches. |
| Checkpoint age | Checkpoint age is modeled and can trigger critical health. | Need per-game checkpoint freshness dashboards and restore-test status. | Gate deploy/upgrade on checkpoint freshness and successful restore rehearsal. |
| Lease capacity | Lease capacity and exhaustion exist in the model. | Need live EverNode inventory and lease renewal/expiration telemetry. | Show capacity remaining, expiring leases, and migration required dates. |
| Cost model | Cost model reports exist. | Need cost attribution from actual leases, storage, bandwidth, and archive retention. | Produce per-game daily estimate and alert on budget thresholds. |

## Consensus versus operations observability

- Consensus-safe metrics: deterministic counters, roots, replay cursors, checkpoint cursors, package hashes, receipt ids.
- Operations-only telemetry: CPU, memory, disk, wall-clock latency, network RTT, GPU usage, process ids, host paths, and alert delivery state.
- The operations-only telemetry must not affect gameplay authority unless converted into an explicit accepted operator record.

## Minimum launch dashboard

- Runtime state: healthy/warning/critical/failed.
- Current package hash, rustrig set hash, runtime version, lease id.
- Replay cursor, replay growth, replay archive lag.
- Latest checkpoint id, checkpoint age, last restore-test result.
- Disk usage and replay/checkpoint storage forecast.
- Active alerts with runbook links.
- Anchor queue: intents emitted, submitted, settled, failed.
- Lease capacity: allocated, running, draining, failed, expiring.
- Estimated daily/monthly cost per game.

## Remaining gaps

The largest observability gaps are live telemetry adapters, durable export, production alert routing, and cost integration with real leases. These are bounded integration tasks and should be completed before unattended launch.
