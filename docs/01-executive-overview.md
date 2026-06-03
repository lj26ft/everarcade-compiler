# 01. Executive Overview

## What EverArcade Is

EverArcade is a deterministic runtime platform for persistent, replayable game worlds. It separates authoritative world execution from rendering, dashboards, and hosting convenience layers so that a world can be verified, recovered, migrated, and operated from durable execution evidence rather than trust in a single process or visual client.

EverArcade exists as a platform, not a single game. A creator packages deterministic world logic and assets; an operator runs that package inside the runtime; the runtime executes inputs, mutates state, emits receipts, creates checkpoints, and provides replay material; federation and deployment systems distribute or host that authority when the required release gates are satisfied.

## What Problem It Solves

Online game infrastructure commonly couples gameplay authority, presentation, server operations, and historical state into one mutable service. That coupling makes worlds difficult to audit, recover, federate, or preserve. EverArcade solves this by treating execution as an append-only, verifiable history:

- gameplay authority is deterministic;
- state changes are represented by canonical roots and receipts;
- recovery uses checkpoints and replay, not informal database repair;
- renderer and frontend systems project state but do not own it;
- operators can validate the lineage of a world before continuing it.

## Core Principles

1. **Authority is explicit.** Only runtime-owned execution paths can create authoritative world state.
2. **Determinism is mandatory.** Given the same package, prior state, input, and protocol version, replay must produce the same roots.
3. **Evidence outlives processes.** Receipts, journals, checkpoints, manifests, and validation artifacts are the durable record.
4. **Projection is not authority.** Renderers, clients, dashboards, and observers may display or summarize state, but they cannot define canonical state.
5. **Production readiness is gated.** A subsystem is not production ready until tests, release gates, operations, recovery, and ownership are explicit.
6. **Historical documents are evidence, not onboarding.** Canonical documentation lives in the numbered documents in this directory.

## Deterministic Execution

EverArcade executes packaged world logic in a deterministic runtime boundary. Determinism means that execution results can be recomputed and compared. The platform uses canonical serialization, state roots, receipt hashes, journal entries, checkpoint hashes, and replay verification to detect divergence.

A deterministic runtime makes several business and operational guarantees possible:

- an auditor can inspect the lineage of a world;
- an operator can restore from a checkpoint and validate replay;
- a federation peer can reject invalid or divergent state;
- creators can debug divergence against reproducible inputs;
- release certification can require evidence instead of manual confidence.

## Runtime Authority

The runtime owns package loading, execution, state mutation, journaling, receipt generation, checkpoint creation, replay verification, recovery, and upgrade continuity. It is the authoritative boundary for world history.

Non-authoritative systems include renderers, portals, dashboards, observers, analytics, creator tools, and convenience CLIs. These systems may submit inputs, request projections, or inspect artifacts, but they do not define canonical world state.

## Federation

Federation is the platform path from a single operated runtime to multi-host execution and recovery. Its purpose is to exchange receipts, checkpoints, roots, peer status, and recovery material while preserving deterministic authority.

Current federation work should be treated as partial unless a specific capability is listed as implemented in the readiness and capability matrices. Federation has tests and scaffolding, but multi-host production authority still requires stronger release gates, operator recovery procedures, adversarial validation, and commercial hosting policy.

## Persistent Worlds

EverArcade targets worlds that continue beyond a single play session. A persistent world needs durable state, replayable history, checkpoint recovery, upgrade continuity, and clear ownership over state domains such as entities, economy, inventory, simulation, governance, and scheduling.

The world runtime is implemented as a set of runtime and SDK capabilities, but not every world subsystem is production ready. The canonical status is documented in `11-production-readiness.md` and `runtime-capabilities.md`.

## Evernode Deployment

Evernode is the deployment target for hosted EverArcade runtime nodes. The platform contains provider and deployment code, reports, templates, and readiness notes. That does not mean commercial hosting is complete. Evernode deployment is currently partial: it can describe and exercise deployment flows, but production hosting requires stronger automation, artifact verification, rollback, observability, capacity policy, operator onboarding, and certification gates.

## Current Platform Status

EverArcade has an implemented deterministic execution foundation, runtime operator surface, persistence primitives, replay verification, checkpoint and recovery paths, SDK crates, example contracts, provider scaffolding, and a large body of validation evidence.

EverArcade is not yet production ready as a commercial, multi-host platform. The key incomplete areas are federation hardening, renderer streaming maturity, release gate automation, operator observability, commercial hosting policy, XRPL integration, ZK proof integration, creator marketplace flows, and complete production runbooks for every failure class.

Use these documents as the authoritative onboarding path:

1. `03-system-architecture.md` for platform architecture.
2. `04-runtime-architecture.md` for runtime behavior.
3. `11-production-readiness.md` for subsystem readiness.
4. `12-gap-analysis.md` for required work.
5. `13-runtime-operations-manual.md` for operator procedures.
6. `documentation-governance.md` for documentation ownership.
