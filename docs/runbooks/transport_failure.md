# Transport Failure Runbook

## Purpose

Respond to TCP disconnects, latency, packet loss, and partitions.

## Procedure

1. Detect heartbeat timeout or failed transfer.
2. Stop accepting new authority changes until continuity is known.
3. Retry with a resume request that includes the last replay cursor.
4. If the peer resumes, transfer missing replay and checkpoint records.
5. If both machines advanced independently, declare a partition event.
6. Preserve both lineages and require manual reconciliation before returning to normal operations.

## Limitations

The current certification harness validates interruption detection, resume, and convergence. It does not automatically merge independently advanced authoritative histories after a partition.
