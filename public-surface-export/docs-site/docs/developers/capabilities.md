# Capability Matrix

Status values follow `MATURITY.md`: **PRODUCTION**, **ALPHA**, **EXPERIMENTAL**, **SCAFFOLD**, and **PLANNED**. This repository currently marks no subsystem as **PRODUCTION**.

| Capability | Status | Audience | What it enables | Where to read more |
| --- | --- | --- | --- | --- |
| Deterministic WASM runtime | ALPHA | Developers, operators | Local deterministic execution and replay-oriented workflows. | [Technical architecture](../architecture/technical-architecture.md) |
| World packaging | ALPHA | Developers, operators | Canonical deployable world artifacts and package roots. | [Canonical package format](../canonical-package-format.md) |
| Replay verification | ALPHA | Operators, protocol engineers | Independent recomputation of receipts and state roots. | [Replay verification](../architecture/replay-verification.md) |
| Checkpoint restore | ALPHA | Operators | Recovery from compact verified state points. | [Checkpointing](../architecture/checkpointing.md) |
| Execution receipts | ALPHA | Operators, protocol engineers | Auditable execution evidence per window. | [Technical architecture](../architecture/technical-architecture.md) |
| Continuity roots | ALPHA | Protocol engineers | Lineage evidence that chains state and receipt history. | [Continuity engine](../concepts/continuity-engine.md) |
| Federation | SCAFFOLD | Operators, protocol engineers | Design path for multi-operator convergence. | [Federation](../architecture/federation.md) |
| World Contracts | EXPERIMENTAL | Developers, protocol engineers | Deterministic mutation boundaries. | [World Contract](../concepts/world-contract.md) |
| RustRigs | ALPHA | Developers | Reusable gameplay domain libraries and examples. | [RustRigs](../concepts/rustrigs.md) |
| Creator SDK | ALPHA | Developers | Local project creation, build, package, and play-local flows. | [Creator SDK quick start](../creator-sdk/quick-start.md) |
| Operator proof bundles | ALPHA | Operators | Replayable evidence for execution windows. | [Operator technical operations](../operators/technical-operations.md) |
| XRPL/Xahau anchoring | SCAFFOLD | Protocol engineers | Boundary for ledger references to runtime evidence. | [XRPL anchoring](../architecture/xrpl-anchoring.md) |
| Evernode deployment | EXPERIMENTAL | Operators | Experimental hosting and lease deployment boundary. | [Evernode integration](../architecture/evernode-integration.md) |
| Marketplace systems | SCAFFOLD | Developers, operators | Future publishing, economics, and distribution flows. | [Capabilities](./capabilities.md) |
| Governance systems | SCAFFOLD | Protocol engineers | Future policy and upgrade coordination surfaces. | [Technical architecture](../architecture/technical-architecture.md) |
| GPU projection/runtime | SCAFFOLD | Operators, developers | Future projection and worker domains outside the proof path. | [Runtime capabilities](../runtime-capabilities.md) |
