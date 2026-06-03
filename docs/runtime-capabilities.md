# Runtime Capability Matrix

This document is the definitive answer to: **What can EverArcade actually do today?** Status values are **Implemented**, **Partial**, **Scaffold**, and **Planned**.

## Deterministic WASM Execution

Status: **Implemented**

The runtime has an implemented deterministic execution foundation. Production certification remains gated by release policy.

## Package Loading

Status: **Implemented**

Runtime package loading exists for configured package paths and world metadata.

## State Mutation and Roots

Status: **Implemented**

State mutation and root calculation foundations exist.

## Receipt Generation

Status: **Implemented**

Receipt and journal evidence foundations exist.

## Journal Persistence

Status: **Implemented**

Runtime journal persistence and verification exist.

## Checkpoint Creation

Status: **Implemented**

Operator checkpoint creation exists.

## Replay Verification

Status: **Implemented**

Replay verification and replay root/report commands exist.

## Runtime Recovery

Status: **Implemented**

Runtime recovery manager foundations exist and are operator-accessible.

## Backup

Status: **Partial**

Checkpoint-backed backup manifests exist; production backup policy and drills remain gated.

## Restore

Status: **Partial**

Restore is not fully production-certified.

## Runtime Upgrade

Status: **Partial**

Upgrade model exists, but production automation and validation gates remain incomplete.

## Runtime Health and Metrics

Status: **Partial**

Health and metrics primitives exist; production observability is incomplete.

## World Runtime

Status: **Partial**

Persistent world runtime foundations exist across runtime and SDK domains.

## Federation Recovery

Status: **Partial**

Federation recovery modules and tests exist; production multi-host recovery is not certified.

## Distributed Receipt Propagation

Status: **Partial**

Distributed receipt capabilities exist but lack production federation gates.

## Checkpoint Synchronization

Status: **Partial**

Checkpoint sync foundations exist; production peer sync is not certified.

## Multi-Host Federation

Status: **Scaffold**

Multi-host authority remains scaffold-level until adversarial and operator gates pass.

## Renderer Streaming

Status: **Scaffold**

Renderer streaming is projection-only and not production ready.

## Historical Replay / Observer Runtime

Status: **Scaffold**

Historical observer paths are scaffold-level projection domains.

## Evernode Deployment Automation

Status: **Partial**

Provider and deployment artifacts exist; production commercial deployment automation is incomplete.

## XRPL Integration

Status: **Scaffold**

XRPL settlement boundaries and hooks are scaffold/partial and not production certified.

## ZK Integration

Status: **Planned**

ZK proof integration is planned and not implemented as a production runtime capability.

## Creator Marketplace

Status: **Scaffold**

Marketplace flows are scaffold-level and depend on package certification and commercial hosting gates.
