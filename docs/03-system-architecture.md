# 03. System Architecture

This document is the single architectural reference for EverArcade. Subsystem documents may explain implementation details, but platform authority, data flow, and ownership are defined here.

## Platform Diagram

```text
Developer
    ↓
Package
    ↓
Runtime
    ↓
Execution Core
    ↓
State Engine
    ↓
Receipts
    ↓
Checkpoint System
    ↓
Replay Verification
    ↓
Federation
    ↓
Deployment
```

## Architectural Summary

A developer creates a package. The runtime loads and validates it. The execution core runs deterministic logic. The state engine applies mutations and computes roots. The receipt system records execution evidence. The checkpoint system persists recovery points. Replay verification recomputes history. Federation exchanges verifiable artifacts. Deployment installs and operates the runtime on target hosts.

## Authority Boundaries

| Boundary | Authoritative | Owns | Does Not Own |
|---|---:|---|---|
| Package format | Yes | declared code, metadata, assets, compatibility | runtime state after execution |
| Runtime platform | Yes | lifecycle, package loading, journal, checkpoints, recovery, operator commands | visual output, marketplace policy |
| Execution core | Yes | deterministic execution result | frontend events outside accepted inputs |
| State engine | Yes | canonical state mutation and roots | display caches |
| Receipt system | Yes | execution evidence and lineage | UX summaries |
| Replay verifier | Yes | reproducibility decision | subjective gameplay quality |
| Federation | Partial | peer exchange, root comparison, recovery material | unilateral state invention |
| Renderer/projection | No | visual projection and client experience | canonical state |
| Dashboards/observers | No | inspection and operations visibility | execution authority |
| Deployment provider | Partial | install, package transfer, host integration | runtime correctness |

## Data Flow

1. **Authoring:** creator uses SDKs and examples to build a package.
2. **Packaging:** package metadata, code, and assets are validated and signed where required.
3. **Loading:** runtime resolves package metadata and prepares execution.
4. **Input admission:** runtime accepts only inputs that match schema, capability, and ordering rules.
5. **Execution:** deterministic core executes the package against prior state.
6. **Mutation:** state engine applies the resulting state transition.
7. **Evidence:** receipt and journal entries bind input, output, state roots, package identity, and sequence.
8. **Checkpoint:** runtime periodically creates durable restore points.
9. **Replay:** verifier recomputes history and compares roots.
10. **Federation:** peers exchange roots, receipts, checkpoints, and missing ranges.
11. **Deployment:** provider tooling installs, upgrades, rolls back, and monitors runtime nodes.

## Ownership Model

- **Runtime owns authority.** Runtime crates decide what happened.
- **SDK owns creator ergonomics.** SDK crates make deterministic authoring possible but cannot bypass runtime validation.
- **Provider owns host integration.** Evernode and deployment code move artifacts and configure hosts.
- **Renderer owns projection.** Rendering code presents world state but must remain non-authoritative.
- **Reports own evidence.** Historical reports support claims but are not canonical architecture.
- **Governance owns documentation scope.** New architecture claims must update this document set.

## Runtime Boundaries

The runtime boundary includes lifecycle, configuration, package loading, input queues, execution loop, journal persistence, checkpoints, backup, replay, recovery, health, metrics, supervisor behavior, and upgrade continuity.

The runtime boundary excludes player UI, renderer interpolation, creator dashboards, commercial billing, marketplace ranking, and external ledger settlement. Those systems integrate through explicit APIs and artifacts.

## Non-Authoritative Systems

The following systems must never be treated as the source of canonical world state:

- browser clients and player portals;
- renderer clients and projection services;
- dashboards and operator consoles;
- analytics, metrics, and reports;
- SDK dev runtimes outside certification;
- historical milestone documents;
- deployment reports without release gate evidence.

## Security Boundaries

- **Package boundary:** untrusted packages must be validated before execution.
- **WASM boundary:** contract execution must be deterministic and resource-governed.
- **State boundary:** only runtime-owned mutation paths can alter canonical state.
- **Receipt boundary:** receipts must bind execution evidence to sequence and roots.
- **Checkpoint boundary:** restore points must verify hashes and ancestry.
- **Peer boundary:** federation peers must be authenticated and verified before trust.
- **Operator boundary:** operator commands must not bypass validation.
- **Deployment boundary:** host installation must verify artifacts before activation.

## Future Architecture

Future architecture work must harden, not blur, boundaries. Multi-host federation, XRPL settlement, ZK proofs, marketplace publishing, and commercial hosting must integrate as external or adjacent domains that consume runtime evidence. They must not replace the runtime as the authority source.
