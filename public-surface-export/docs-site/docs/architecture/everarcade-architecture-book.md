# EverArcade Architecture Book

## Purpose

This book is the authoritative architecture reference for EverArcade. It consolidates repository documentation, runtime modules, validation reports, test suites, deployment reports, architecture notes, and milestone artifacts into one onboarding system for senior Rust engineers, infrastructure engineers, protocol developers, and future contributors.

EverArcade is a deterministic execution platform for games and persistent worlds. Its protocol center is not a renderer, game loop, dashboard, or hosting provider. Its center is a verifiable state transition: a prior canonical state plus a validated input and execution plan produces a new canonical state, roots, receipts, checkpoints, replay material, and optionally non-authoritative projection frames.

## Responsibilities

- Explain what EverArcade is and why it exists.
- Define authoritative execution, WASM boundaries, receipts, replay, lineage, checkpoints, federation, world simulation, renderer projection, deployment, and SDK ownership.
- Provide a repository inventory that lets a new engineer find code, tests, reports, and runbooks.
- Normalize older milestones into current architecture language.
- Classify implemented, partially implemented, scaffold, and planned systems.

## Non-Responsibilities

- This book does not replace Rust API docs or source review for exact type signatures.
- This book does not mark every module production-ready. Renderer, history, and federation client domains must be treated as scaffold-level unless the status matrix says otherwise.
- This book does not make visual output authoritative. Display is derived from runtime facts.

## Internal Components

- Part I: platform vision and deterministic protocol principles.
- Part II: runtime architecture: execution core, WASM, state, receipts, replay, lineage, checkpoints.
- Part III: federation and recovery.
- Part IV: persistent world, simulation, economy, governance, inventory, and partitioning.
- Part V: renderer projection and historical replay.
- Part VI: deployment, release certification, Evernode operation, and recovery.
- Part VII: SDK, game development, contract development, repository inventory, and roadmap.

## Data Flow

User input or peer artifact enters a boundary, is validated, is executed deterministically, mutates canonical state only through owned runtime modules, emits receipts and roots, persists checkpoints or archives, and is verified by replay, signature checks, continuity checks, or reconciliation.

```text
Deterministic Execution Pipeline

User Input
↓
Host Runtime
↓
WASM Guest / Runtime Executor
↓
State Diff
↓
Canonical State
↓
State Root
↓
Receipt
↓
Journal / Store
↓
Replay Verification
```

## Determinism Guarantees

EverArcade determinism is built from canonical serialization, stable hashing, explicit protocol epochs, receipt generation, replay verification, checkpoint restoration, and divergence detection. Determinism must be preserved across native host execution, guest WASM execution, federation synchronization, world scheduling, and renderer projection boundaries.

## Failure Modes

The major failure classes are artifact corruption, execution divergence, missing continuity, malformed or malicious peer input, checkpoint gaps, replay gaps, storage loss, and operator mistakes. Recovery uses quarantine, replay reconstruction, checkpoint restore, receipt range transfer, reconciliation plans, rollback recovery, and operator runbooks.

## Future Evolution

The project should evolve by turning reports into executable release gates, hardening persistence and federation, stabilizing package and SDK APIs, and keeping scaffold-level renderer/history/federation domains explicit until validated.


# What EverArcade Is

## Purpose

EverArcade exists to make game and world execution reproducible, inspectable, and portable. The platform treats the execution transcript as a durable asset. A game is not merely a binary and a renderer; it is a package that can be executed, checked, replayed, synchronized, restored, and audited.

## Responsibilities

- Preserve deterministic authority and explicit ownership.
- Keep inputs, state changes, receipts, roots, checkpoints, replay material, and synchronization artifacts verifiable.
- Maintain contributor clarity by distinguishing implementation evidence from roadmap intent.

## Non-Responsibilities

- This subsystem narrative does not claim every named module is production-grade.
- It does not move authority into renderer, dashboard, observer, or convenience tooling paths.
- It does not replace targeted source review and tests before changing Rust behavior.

## Internal Components

- Source modules named in the subsystem chapters.
- Tests, fixtures, validation reports, and runbooks that support the architecture claim.
- Roadmap rows that identify implemented, partial, scaffold, and planned status.

### Canonical Data Flow

Input enters through a host, SDK, contract, peer, checkpoint, replay archive, or package boundary. Processing validates schema, version, signature, hash ancestry, and capability scope. State mutation occurs only through deterministic modules that own the relevant state. Receipt generation records roots, input hashes, output hashes, execution roots, state changes, snapshots, and protocol epoch metadata. Verification reruns execution or compares signed roots, receipt chains, checkpoint ancestry, and continuity records.

### Determinism and Validation

Hashing is the common language between modules. Canonical state roots prove the state that resulted from execution. Execution roots summarize the planned work. Receipt hashes summarize evidence. Checkpoint roots and snapshot hashes bridge recovery. Continuity roots and lineage records explain why later history descends from earlier history. Canonical serialization is mandatory before hashing. A module that serializes the same logical value differently on two machines cannot be authoritative.

Replay is the practical test of determinism. If a verifier receives the same prior state, input, protocol epoch, and contract package, it must recompute the same roots and receipts. If it cannot, EverArcade treats that as divergence, not as a renderer bug or UX inconsistency. Divergence is resolved by comparing roots, choosing valid lineage, restoring checkpoints, exchanging missing receipt ranges, and quarantining invalid peers or artifacts.

### Failure and Recovery Pattern

Corruption is detected by mismatched hashes, invalid signatures, malformed manifests, invalid proof material, or checkpoint ancestry gaps. Divergence is detected by root comparison and replay. Recovery chooses the smallest safe repair: import missing receipts, hydrate an archive, restore a checkpoint, roll back a window, resynchronize from a peer, or rebuild an index. Reconciliation must never invent authoritative state; it must select from verifiable material or recompute from canonical inputs.

## Future Evolution

Future work should turn partial and scaffold areas into validated runtime paths only when they have deterministic tests, operational runbooks, release gates, and clear owner modules.


# Vision and Principles

## Purpose

The key principle is that authority follows deterministic evidence. Inputs, packages, state, roots, receipts, checkpoints, and replay records are the authoritative chain. Renderers are clients of that chain. Federation peers are witnesses and executors of that chain. Deployment systems distribute and operate that chain.

## Responsibilities

- Preserve deterministic authority and explicit ownership.
- Keep inputs, state changes, receipts, roots, checkpoints, replay material, and synchronization artifacts verifiable.
- Maintain contributor clarity by distinguishing implementation evidence from roadmap intent.

## Non-Responsibilities

- This subsystem narrative does not claim every named module is production-grade.
- It does not move authority into renderer, dashboard, observer, or convenience tooling paths.
- It does not replace targeted source review and tests before changing Rust behavior.

## Internal Components

- Source modules named in the subsystem chapters.
- Tests, fixtures, validation reports, and runbooks that support the architecture claim.
- Roadmap rows that identify implemented, partial, scaffold, and planned status.

### Canonical Data Flow

Input enters through a host, SDK, contract, peer, checkpoint, replay archive, or package boundary. Processing validates schema, version, signature, hash ancestry, and capability scope. State mutation occurs only through deterministic modules that own the relevant state. Receipt generation records roots, input hashes, output hashes, execution roots, state changes, snapshots, and protocol epoch metadata. Verification reruns execution or compares signed roots, receipt chains, checkpoint ancestry, and continuity records.

### Determinism and Validation

Hashing is the common language between modules. Canonical state roots prove the state that resulted from execution. Execution roots summarize the planned work. Receipt hashes summarize evidence. Checkpoint roots and snapshot hashes bridge recovery. Continuity roots and lineage records explain why later history descends from earlier history. Canonical serialization is mandatory before hashing. A module that serializes the same logical value differently on two machines cannot be authoritative.

Replay is the practical test of determinism. If a verifier receives the same prior state, input, protocol epoch, and contract package, it must recompute the same roots and receipts. If it cannot, EverArcade treats that as divergence, not as a renderer bug or UX inconsistency. Divergence is resolved by comparing roots, choosing valid lineage, restoring checkpoints, exchanging missing receipt ranges, and quarantining invalid peers or artifacts.

### Failure and Recovery Pattern

Corruption is detected by mismatched hashes, invalid signatures, malformed manifests, invalid proof material, or checkpoint ancestry gaps. Divergence is detected by root comparison and replay. Recovery chooses the smallest safe repair: import missing receipts, hydrate an archive, restore a checkpoint, roll back a window, resynchronize from a peer, or rebuild an index. Reconciliation must never invent authoritative state; it must select from verifiable material or recompute from canonical inputs.

## Future Evolution

Future work should turn partial and scaffold areas into validated runtime paths only when they have deterministic tests, operational runbooks, release gates, and clear owner modules.


# System Overview

## Purpose

The workspace is organized around an execution core, a host runtime, runtime clients, renderer clients, contract APIs, SDK crates, example contracts, provider deployment code, tools, reports, runbooks, templates, and test vectors. Historical reports remain evidence; current architecture documents and source modules are the canonical map.

## Responsibilities

- Preserve deterministic authority and explicit ownership.
- Keep inputs, state changes, receipts, roots, checkpoints, replay material, and synchronization artifacts verifiable.
- Maintain contributor clarity by distinguishing implementation evidence from roadmap intent.

## Non-Responsibilities

- This subsystem narrative does not claim every named module is production-grade.
- It does not move authority into renderer, dashboard, observer, or convenience tooling paths.
- It does not replace targeted source review and tests before changing Rust behavior.

## Internal Components

- Source modules named in the subsystem chapters.
- Tests, fixtures, validation reports, and runbooks that support the architecture claim.
- Roadmap rows that identify implemented, partial, scaffold, and planned status.

### Canonical Data Flow

Input enters through a host, SDK, contract, peer, checkpoint, replay archive, or package boundary. Processing validates schema, version, signature, hash ancestry, and capability scope. State mutation occurs only through deterministic modules that own the relevant state. Receipt generation records roots, input hashes, output hashes, execution roots, state changes, snapshots, and protocol epoch metadata. Verification reruns execution or compares signed roots, receipt chains, checkpoint ancestry, and continuity records.

### Determinism and Validation

Hashing is the common language between modules. Canonical state roots prove the state that resulted from execution. Execution roots summarize the planned work. Receipt hashes summarize evidence. Checkpoint roots and snapshot hashes bridge recovery. Continuity roots and lineage records explain why later history descends from earlier history. Canonical serialization is mandatory before hashing. A module that serializes the same logical value differently on two machines cannot be authoritative.

Replay is the practical test of determinism. If a verifier receives the same prior state, input, protocol epoch, and contract package, it must recompute the same roots and receipts. If it cannot, EverArcade treats that as divergence, not as a renderer bug or UX inconsistency. Divergence is resolved by comparing roots, choosing valid lineage, restoring checkpoints, exchanging missing receipt ranges, and quarantining invalid peers or artifacts.

### Failure and Recovery Pattern

Corruption is detected by mismatched hashes, invalid signatures, malformed manifests, invalid proof material, or checkpoint ancestry gaps. Divergence is detected by root comparison and replay. Recovery chooses the smallest safe repair: import missing receipts, hydrate an archive, restore a checkpoint, roll back a window, resynchronize from a peer, or rebuild an index. Reconciliation must never invent authoritative state; it must select from verifiable material or recompute from canonical inputs.

## Future Evolution

Future work should turn partial and scaffold areas into validated runtime paths only when they have deterministic tests, operational runbooks, release gates, and clear owner modules.


# Execution Core

## Purpose

execution-core is the protocol foundation. It exposes modules for canonical serialization, hashing, Merkle/state roots, state engine, receipts, replay, checkpoints, lineage, federation runtime, world runtime, scheduler, economy, governance, inventory, package handling, security, settlement, and WASM ABI exports. The execute path constructs state stores, computes previous roots, execution roots, output hashes, snapshot hashes, and receipt hashes.

## Responsibilities

- Preserve deterministic authority and explicit ownership.
- Keep inputs, state changes, receipts, roots, checkpoints, replay material, and synchronization artifacts verifiable.
- Maintain contributor clarity by distinguishing implementation evidence from roadmap intent.

## Non-Responsibilities

- This subsystem narrative does not claim every named module is production-grade.
- It does not move authority into renderer, dashboard, observer, or convenience tooling paths.
- It does not replace targeted source review and tests before changing Rust behavior.

## Internal Components

- Source modules named in the subsystem chapters.
- Tests, fixtures, validation reports, and runbooks that support the architecture claim.
- Roadmap rows that identify implemented, partial, scaffold, and planned status.

### Canonical Data Flow

Input enters through a host, SDK, contract, peer, checkpoint, replay archive, or package boundary. Processing validates schema, version, signature, hash ancestry, and capability scope. State mutation occurs only through deterministic modules that own the relevant state. Receipt generation records roots, input hashes, output hashes, execution roots, state changes, snapshots, and protocol epoch metadata. Verification reruns execution or compares signed roots, receipt chains, checkpoint ancestry, and continuity records.

### Determinism and Validation

Hashing is the common language between modules. Canonical state roots prove the state that resulted from execution. Execution roots summarize the planned work. Receipt hashes summarize evidence. Checkpoint roots and snapshot hashes bridge recovery. Continuity roots and lineage records explain why later history descends from earlier history. Canonical serialization is mandatory before hashing. A module that serializes the same logical value differently on two machines cannot be authoritative.

Replay is the practical test of determinism. If a verifier receives the same prior state, input, protocol epoch, and contract package, it must recompute the same roots and receipts. If it cannot, EverArcade treats that as divergence, not as a renderer bug or UX inconsistency. Divergence is resolved by comparing roots, choosing valid lineage, restoring checkpoints, exchanging missing receipt ranges, and quarantining invalid peers or artifacts.

### Failure and Recovery Pattern

Corruption is detected by mismatched hashes, invalid signatures, malformed manifests, invalid proof material, or checkpoint ancestry gaps. Divergence is detected by root comparison and replay. Recovery chooses the smallest safe repair: import missing receipts, hydrate an archive, restore a checkpoint, roll back a window, resynchronize from a peer, or rebuild an index. Reconciliation must never invent authoritative state; it must select from verifiable material or recompute from canonical inputs.

## Future Evolution

Future work should turn partial and scaffold areas into validated runtime paths only when they have deterministic tests, operational runbooks, release gates, and clear owner modules.


# WASM Runtime

## Purpose

The WASM boundary allows guest logic to run under a deterministic ABI. The guest accepts serialized VmInput, returns serialized VmOutput, and exposes allocation/output helpers. The design goal is to make a package portable while keeping the host responsible for validation, fuel/memory policy, package loading, and evidence persistence.

## Responsibilities

- Preserve deterministic authority and explicit ownership.
- Keep inputs, state changes, receipts, roots, checkpoints, replay material, and synchronization artifacts verifiable.
- Maintain contributor clarity by distinguishing implementation evidence from roadmap intent.

## Non-Responsibilities

- This subsystem narrative does not claim every named module is production-grade.
- It does not move authority into renderer, dashboard, observer, or convenience tooling paths.
- It does not replace targeted source review and tests before changing Rust behavior.

## Internal Components

- Source modules named in the subsystem chapters.
- Tests, fixtures, validation reports, and runbooks that support the architecture claim.
- Roadmap rows that identify implemented, partial, scaffold, and planned status.

### Canonical Data Flow

Input enters through a host, SDK, contract, peer, checkpoint, replay archive, or package boundary. Processing validates schema, version, signature, hash ancestry, and capability scope. State mutation occurs only through deterministic modules that own the relevant state. Receipt generation records roots, input hashes, output hashes, execution roots, state changes, snapshots, and protocol epoch metadata. Verification reruns execution or compares signed roots, receipt chains, checkpoint ancestry, and continuity records.

### Determinism and Validation

Hashing is the common language between modules. Canonical state roots prove the state that resulted from execution. Execution roots summarize the planned work. Receipt hashes summarize evidence. Checkpoint roots and snapshot hashes bridge recovery. Continuity roots and lineage records explain why later history descends from earlier history. Canonical serialization is mandatory before hashing. A module that serializes the same logical value differently on two machines cannot be authoritative.

Replay is the practical test of determinism. If a verifier receives the same prior state, input, protocol epoch, and contract package, it must recompute the same roots and receipts. If it cannot, EverArcade treats that as divergence, not as a renderer bug or UX inconsistency. Divergence is resolved by comparing roots, choosing valid lineage, restoring checkpoints, exchanging missing receipt ranges, and quarantining invalid peers or artifacts.

### Failure and Recovery Pattern

Corruption is detected by mismatched hashes, invalid signatures, malformed manifests, invalid proof material, or checkpoint ancestry gaps. Divergence is detected by root comparison and replay. Recovery chooses the smallest safe repair: import missing receipts, hydrate an archive, restore a checkpoint, roll back a window, resynchronize from a peer, or rebuild an index. Reconciliation must never invent authoritative state; it must select from verifiable material or recompute from canonical inputs.

## Future Evolution

Future work should turn partial and scaffold areas into validated runtime paths only when they have deterministic tests, operational runbooks, release gates, and clear owner modules.


# State Engine

## Purpose

The state engine owns canonical state mutation. It is intentionally narrow: apply ordered changes, maintain a store, compute Merkle roots, create snapshots, and support proofs/history. All higher-level domains must eventually reduce their authoritative mutations to canonical state or subsystem-specific roots that can be checked by replay.

## Responsibilities

- Preserve deterministic authority and explicit ownership.
- Keep inputs, state changes, receipts, roots, checkpoints, replay material, and synchronization artifacts verifiable.
- Maintain contributor clarity by distinguishing implementation evidence from roadmap intent.

## Non-Responsibilities

- This subsystem narrative does not claim every named module is production-grade.
- It does not move authority into renderer, dashboard, observer, or convenience tooling paths.
- It does not replace targeted source review and tests before changing Rust behavior.

## Internal Components

- Source modules named in the subsystem chapters.
- Tests, fixtures, validation reports, and runbooks that support the architecture claim.
- Roadmap rows that identify implemented, partial, scaffold, and planned status.

### Canonical Data Flow

Input enters through a host, SDK, contract, peer, checkpoint, replay archive, or package boundary. Processing validates schema, version, signature, hash ancestry, and capability scope. State mutation occurs only through deterministic modules that own the relevant state. Receipt generation records roots, input hashes, output hashes, execution roots, state changes, snapshots, and protocol epoch metadata. Verification reruns execution or compares signed roots, receipt chains, checkpoint ancestry, and continuity records.

### Determinism and Validation

Hashing is the common language between modules. Canonical state roots prove the state that resulted from execution. Execution roots summarize the planned work. Receipt hashes summarize evidence. Checkpoint roots and snapshot hashes bridge recovery. Continuity roots and lineage records explain why later history descends from earlier history. Canonical serialization is mandatory before hashing. A module that serializes the same logical value differently on two machines cannot be authoritative.

Replay is the practical test of determinism. If a verifier receives the same prior state, input, protocol epoch, and contract package, it must recompute the same roots and receipts. If it cannot, EverArcade treats that as divergence, not as a renderer bug or UX inconsistency. Divergence is resolved by comparing roots, choosing valid lineage, restoring checkpoints, exchanging missing receipt ranges, and quarantining invalid peers or artifacts.

### Failure and Recovery Pattern

Corruption is detected by mismatched hashes, invalid signatures, malformed manifests, invalid proof material, or checkpoint ancestry gaps. Divergence is detected by root comparison and replay. Recovery chooses the smallest safe repair: import missing receipts, hydrate an archive, restore a checkpoint, roll back a window, resynchronize from a peer, or rebuild an index. Reconciliation must never invent authoritative state; it must select from verifiable material or recompute from canonical inputs.

## Future Evolution

Future work should turn partial and scaffold areas into validated runtime paths only when they have deterministic tests, operational runbooks, release gates, and clear owner modules.


# Receipt System

## Purpose

Receipts are EverArcade evidence records. They bind protocol epoch, ABI version, input hash, previous root, new root, execution root, resource usage, state changes, output hash, snapshot hash, and receipt hash. Signed and distributed receipts enable peers and operators to synchronize without trusting unaudited logs.

## Responsibilities

- Preserve deterministic authority and explicit ownership.
- Keep inputs, state changes, receipts, roots, checkpoints, replay material, and synchronization artifacts verifiable.
- Maintain contributor clarity by distinguishing implementation evidence from roadmap intent.

## Non-Responsibilities

- This subsystem narrative does not claim every named module is production-grade.
- It does not move authority into renderer, dashboard, observer, or convenience tooling paths.
- It does not replace targeted source review and tests before changing Rust behavior.

## Internal Components

- Source modules named in the subsystem chapters.
- Tests, fixtures, validation reports, and runbooks that support the architecture claim.
- Roadmap rows that identify implemented, partial, scaffold, and planned status.

### Canonical Data Flow

Input enters through a host, SDK, contract, peer, checkpoint, replay archive, or package boundary. Processing validates schema, version, signature, hash ancestry, and capability scope. State mutation occurs only through deterministic modules that own the relevant state. Receipt generation records roots, input hashes, output hashes, execution roots, state changes, snapshots, and protocol epoch metadata. Verification reruns execution or compares signed roots, receipt chains, checkpoint ancestry, and continuity records.

### Determinism and Validation

Hashing is the common language between modules. Canonical state roots prove the state that resulted from execution. Execution roots summarize the planned work. Receipt hashes summarize evidence. Checkpoint roots and snapshot hashes bridge recovery. Continuity roots and lineage records explain why later history descends from earlier history. Canonical serialization is mandatory before hashing. A module that serializes the same logical value differently on two machines cannot be authoritative.

Replay is the practical test of determinism. If a verifier receives the same prior state, input, protocol epoch, and contract package, it must recompute the same roots and receipts. If it cannot, EverArcade treats that as divergence, not as a renderer bug or UX inconsistency. Divergence is resolved by comparing roots, choosing valid lineage, restoring checkpoints, exchanging missing receipt ranges, and quarantining invalid peers or artifacts.

### Failure and Recovery Pattern

Corruption is detected by mismatched hashes, invalid signatures, malformed manifests, invalid proof material, or checkpoint ancestry gaps. Divergence is detected by root comparison and replay. Recovery chooses the smallest safe repair: import missing receipts, hydrate an archive, restore a checkpoint, roll back a window, resynchronize from a peer, or rebuild an index. Reconciliation must never invent authoritative state; it must select from verifiable material or recompute from canonical inputs.

## Future Evolution

Future work should turn partial and scaffold areas into validated runtime paths only when they have deterministic tests, operational runbooks, release gates, and clear owner modules.


# Replay Engine

## Purpose

Replay verifies that execution is reproducible. The engine compares recomputed roots and receipt material across local runs, peer bundles, historical windows, and test vectors. Replay is also a debugging and recovery mechanism because it identifies the first boundary where roots diverge.

## Responsibilities

- Preserve deterministic authority and explicit ownership.
- Keep inputs, state changes, receipts, roots, checkpoints, replay material, and synchronization artifacts verifiable.
- Maintain contributor clarity by distinguishing implementation evidence from roadmap intent.

## Non-Responsibilities

- This subsystem narrative does not claim every named module is production-grade.
- It does not move authority into renderer, dashboard, observer, or convenience tooling paths.
- It does not replace targeted source review and tests before changing Rust behavior.

## Internal Components

- Source modules named in the subsystem chapters.
- Tests, fixtures, validation reports, and runbooks that support the architecture claim.
- Roadmap rows that identify implemented, partial, scaffold, and planned status.

### Canonical Data Flow

Input enters through a host, SDK, contract, peer, checkpoint, replay archive, or package boundary. Processing validates schema, version, signature, hash ancestry, and capability scope. State mutation occurs only through deterministic modules that own the relevant state. Receipt generation records roots, input hashes, output hashes, execution roots, state changes, snapshots, and protocol epoch metadata. Verification reruns execution or compares signed roots, receipt chains, checkpoint ancestry, and continuity records.

### Determinism and Validation

Hashing is the common language between modules. Canonical state roots prove the state that resulted from execution. Execution roots summarize the planned work. Receipt hashes summarize evidence. Checkpoint roots and snapshot hashes bridge recovery. Continuity roots and lineage records explain why later history descends from earlier history. Canonical serialization is mandatory before hashing. A module that serializes the same logical value differently on two machines cannot be authoritative.

Replay is the practical test of determinism. If a verifier receives the same prior state, input, protocol epoch, and contract package, it must recompute the same roots and receipts. If it cannot, EverArcade treats that as divergence, not as a renderer bug or UX inconsistency. Divergence is resolved by comparing roots, choosing valid lineage, restoring checkpoints, exchanging missing receipt ranges, and quarantining invalid peers or artifacts.

### Failure and Recovery Pattern

Corruption is detected by mismatched hashes, invalid signatures, malformed manifests, invalid proof material, or checkpoint ancestry gaps. Divergence is detected by root comparison and replay. Recovery chooses the smallest safe repair: import missing receipts, hydrate an archive, restore a checkpoint, roll back a window, resynchronize from a peer, or rebuild an index. Reconciliation must never invent authoritative state; it must select from verifiable material or recompute from canonical inputs.

## Future Evolution

Future work should turn partial and scaffold areas into validated runtime paths only when they have deterministic tests, operational runbooks, release gates, and clear owner modules.


# Lineage and Continuity

## Purpose

Lineage explains ancestry. A state root by itself proves a state, but lineage explains how that state descends from prior execution, checkpoint, archive, civilization, or partition history. Continuity records protect long-lived worlds from accidental forks and unverifiable migration.

## Responsibilities

- Preserve deterministic authority and explicit ownership.
- Keep inputs, state changes, receipts, roots, checkpoints, replay material, and synchronization artifacts verifiable.
- Maintain contributor clarity by distinguishing implementation evidence from roadmap intent.

## Non-Responsibilities

- This subsystem narrative does not claim every named module is production-grade.
- It does not move authority into renderer, dashboard, observer, or convenience tooling paths.
- It does not replace targeted source review and tests before changing Rust behavior.

## Internal Components

- Source modules named in the subsystem chapters.
- Tests, fixtures, validation reports, and runbooks that support the architecture claim.
- Roadmap rows that identify implemented, partial, scaffold, and planned status.

### Canonical Data Flow

Input enters through a host, SDK, contract, peer, checkpoint, replay archive, or package boundary. Processing validates schema, version, signature, hash ancestry, and capability scope. State mutation occurs only through deterministic modules that own the relevant state. Receipt generation records roots, input hashes, output hashes, execution roots, state changes, snapshots, and protocol epoch metadata. Verification reruns execution or compares signed roots, receipt chains, checkpoint ancestry, and continuity records.

### Determinism and Validation

Hashing is the common language between modules. Canonical state roots prove the state that resulted from execution. Execution roots summarize the planned work. Receipt hashes summarize evidence. Checkpoint roots and snapshot hashes bridge recovery. Continuity roots and lineage records explain why later history descends from earlier history. Canonical serialization is mandatory before hashing. A module that serializes the same logical value differently on two machines cannot be authoritative.

Replay is the practical test of determinism. If a verifier receives the same prior state, input, protocol epoch, and contract package, it must recompute the same roots and receipts. If it cannot, EverArcade treats that as divergence, not as a renderer bug or UX inconsistency. Divergence is resolved by comparing roots, choosing valid lineage, restoring checkpoints, exchanging missing receipt ranges, and quarantining invalid peers or artifacts.

### Failure and Recovery Pattern

Corruption is detected by mismatched hashes, invalid signatures, malformed manifests, invalid proof material, or checkpoint ancestry gaps. Divergence is detected by root comparison and replay. Recovery chooses the smallest safe repair: import missing receipts, hydrate an archive, restore a checkpoint, roll back a window, resynchronize from a peer, or rebuild an index. Reconciliation must never invent authoritative state; it must select from verifiable material or recompute from canonical inputs.

## Future Evolution

Future work should turn partial and scaffold areas into validated runtime paths only when they have deterministic tests, operational runbooks, release gates, and clear owner modules.


# Checkpoint System

## Purpose

Checkpoints provide bounded recovery. Rather than replaying from genesis every time, the runtime can restore a checkpoint, validate its ancestry, import missing receipt ranges, and resume execution. Checkpoint transfer and fast sync are crucial for federation and operator recovery.

## Responsibilities

- Preserve deterministic authority and explicit ownership.
- Keep inputs, state changes, receipts, roots, checkpoints, replay material, and synchronization artifacts verifiable.
- Maintain contributor clarity by distinguishing implementation evidence from roadmap intent.

## Non-Responsibilities

- This subsystem narrative does not claim every named module is production-grade.
- It does not move authority into renderer, dashboard, observer, or convenience tooling paths.
- It does not replace targeted source review and tests before changing Rust behavior.

## Internal Components

- Source modules named in the subsystem chapters.
- Tests, fixtures, validation reports, and runbooks that support the architecture claim.
- Roadmap rows that identify implemented, partial, scaffold, and planned status.

### Canonical Data Flow

Input enters through a host, SDK, contract, peer, checkpoint, replay archive, or package boundary. Processing validates schema, version, signature, hash ancestry, and capability scope. State mutation occurs only through deterministic modules that own the relevant state. Receipt generation records roots, input hashes, output hashes, execution roots, state changes, snapshots, and protocol epoch metadata. Verification reruns execution or compares signed roots, receipt chains, checkpoint ancestry, and continuity records.

### Determinism and Validation

Hashing is the common language between modules. Canonical state roots prove the state that resulted from execution. Execution roots summarize the planned work. Receipt hashes summarize evidence. Checkpoint roots and snapshot hashes bridge recovery. Continuity roots and lineage records explain why later history descends from earlier history. Canonical serialization is mandatory before hashing. A module that serializes the same logical value differently on two machines cannot be authoritative.

Replay is the practical test of determinism. If a verifier receives the same prior state, input, protocol epoch, and contract package, it must recompute the same roots and receipts. If it cannot, EverArcade treats that as divergence, not as a renderer bug or UX inconsistency. Divergence is resolved by comparing roots, choosing valid lineage, restoring checkpoints, exchanging missing receipt ranges, and quarantining invalid peers or artifacts.

### Failure and Recovery Pattern

Corruption is detected by mismatched hashes, invalid signatures, malformed manifests, invalid proof material, or checkpoint ancestry gaps. Divergence is detected by root comparison and replay. Recovery chooses the smallest safe repair: import missing receipts, hydrate an archive, restore a checkpoint, roll back a window, resynchronize from a peer, or rebuild an index. Reconciliation must never invent authoritative state; it must select from verifiable material or recompute from canonical inputs.

## Future Evolution

Future work should turn partial and scaffold areas into validated runtime paths only when they have deterministic tests, operational runbooks, release gates, and clear owner modules.


# Federation Runtime

## Purpose

Federation allows multiple peers to execute, verify, exchange, and recover runtime history. It is not a license to accept peer state blindly. Peers exchange bundles, receipts, checkpoints, topology updates, assignments, and replay windows. Verification remains rooted in hashes, signatures, and continuity.

## Responsibilities

- Preserve deterministic authority and explicit ownership.
- Keep inputs, state changes, receipts, roots, checkpoints, replay material, and synchronization artifacts verifiable.
- Maintain contributor clarity by distinguishing implementation evidence from roadmap intent.

## Non-Responsibilities

- This subsystem narrative does not claim every named module is production-grade.
- It does not move authority into renderer, dashboard, observer, or convenience tooling paths.
- It does not replace targeted source review and tests before changing Rust behavior.

## Internal Components

- Source modules named in the subsystem chapters.
- Tests, fixtures, validation reports, and runbooks that support the architecture claim.
- Roadmap rows that identify implemented, partial, scaffold, and planned status.

### Canonical Data Flow

Input enters through a host, SDK, contract, peer, checkpoint, replay archive, or package boundary. Processing validates schema, version, signature, hash ancestry, and capability scope. State mutation occurs only through deterministic modules that own the relevant state. Receipt generation records roots, input hashes, output hashes, execution roots, state changes, snapshots, and protocol epoch metadata. Verification reruns execution or compares signed roots, receipt chains, checkpoint ancestry, and continuity records.

### Determinism and Validation

Hashing is the common language between modules. Canonical state roots prove the state that resulted from execution. Execution roots summarize the planned work. Receipt hashes summarize evidence. Checkpoint roots and snapshot hashes bridge recovery. Continuity roots and lineage records explain why later history descends from earlier history. Canonical serialization is mandatory before hashing. A module that serializes the same logical value differently on two machines cannot be authoritative.

Replay is the practical test of determinism. If a verifier receives the same prior state, input, protocol epoch, and contract package, it must recompute the same roots and receipts. If it cannot, EverArcade treats that as divergence, not as a renderer bug or UX inconsistency. Divergence is resolved by comparing roots, choosing valid lineage, restoring checkpoints, exchanging missing receipt ranges, and quarantining invalid peers or artifacts.

### Failure and Recovery Pattern

Corruption is detected by mismatched hashes, invalid signatures, malformed manifests, invalid proof material, or checkpoint ancestry gaps. Divergence is detected by root comparison and replay. Recovery chooses the smallest safe repair: import missing receipts, hydrate an archive, restore a checkpoint, roll back a window, resynchronize from a peer, or rebuild an index. Reconciliation must never invent authoritative state; it must select from verifiable material or recompute from canonical inputs.

## Future Evolution

Future work should turn partial and scaffold areas into validated runtime paths only when they have deterministic tests, operational runbooks, release gates, and clear owner modules.


# Synchronization

## Purpose

Synchronization moves artifacts between peers. A peer may need a receipt range, checkpoint delta, replay window, archive package, topology update, or execution assignment. Sync succeeds only when the imported material validates against expected roots, signatures, windows, and continuity.

## Responsibilities

- Preserve deterministic authority and explicit ownership.
- Keep inputs, state changes, receipts, roots, checkpoints, replay material, and synchronization artifacts verifiable.
- Maintain contributor clarity by distinguishing implementation evidence from roadmap intent.

## Non-Responsibilities

- This subsystem narrative does not claim every named module is production-grade.
- It does not move authority into renderer, dashboard, observer, or convenience tooling paths.
- It does not replace targeted source review and tests before changing Rust behavior.

## Internal Components

- Source modules named in the subsystem chapters.
- Tests, fixtures, validation reports, and runbooks that support the architecture claim.
- Roadmap rows that identify implemented, partial, scaffold, and planned status.

### Canonical Data Flow

Input enters through a host, SDK, contract, peer, checkpoint, replay archive, or package boundary. Processing validates schema, version, signature, hash ancestry, and capability scope. State mutation occurs only through deterministic modules that own the relevant state. Receipt generation records roots, input hashes, output hashes, execution roots, state changes, snapshots, and protocol epoch metadata. Verification reruns execution or compares signed roots, receipt chains, checkpoint ancestry, and continuity records.

### Determinism and Validation

Hashing is the common language between modules. Canonical state roots prove the state that resulted from execution. Execution roots summarize the planned work. Receipt hashes summarize evidence. Checkpoint roots and snapshot hashes bridge recovery. Continuity roots and lineage records explain why later history descends from earlier history. Canonical serialization is mandatory before hashing. A module that serializes the same logical value differently on two machines cannot be authoritative.

Replay is the practical test of determinism. If a verifier receives the same prior state, input, protocol epoch, and contract package, it must recompute the same roots and receipts. If it cannot, EverArcade treats that as divergence, not as a renderer bug or UX inconsistency. Divergence is resolved by comparing roots, choosing valid lineage, restoring checkpoints, exchanging missing receipt ranges, and quarantining invalid peers or artifacts.

### Failure and Recovery Pattern

Corruption is detected by mismatched hashes, invalid signatures, malformed manifests, invalid proof material, or checkpoint ancestry gaps. Divergence is detected by root comparison and replay. Recovery chooses the smallest safe repair: import missing receipts, hydrate an archive, restore a checkpoint, roll back a window, resynchronize from a peer, or rebuild an index. Reconciliation must never invent authoritative state; it must select from verifiable material or recompute from canonical inputs.

## Future Evolution

Future work should turn partial and scaffold areas into validated runtime paths only when they have deterministic tests, operational runbooks, release gates, and clear owner modules.


# Recovery and Reconciliation

## Purpose

Recovery handles gaps, corruption, divergence, stale peers, partitions, and operator mistakes. Reconciliation compares roots and peer material, builds a repair plan, imports or restores verifiable artifacts, and resumes advancement. The system should prefer minimal safe repair over destructive reset.

## Responsibilities

- Preserve deterministic authority and explicit ownership.
- Keep inputs, state changes, receipts, roots, checkpoints, replay material, and synchronization artifacts verifiable.
- Maintain contributor clarity by distinguishing implementation evidence from roadmap intent.

## Non-Responsibilities

- This subsystem narrative does not claim every named module is production-grade.
- It does not move authority into renderer, dashboard, observer, or convenience tooling paths.
- It does not replace targeted source review and tests before changing Rust behavior.

## Internal Components

- Source modules named in the subsystem chapters.
- Tests, fixtures, validation reports, and runbooks that support the architecture claim.
- Roadmap rows that identify implemented, partial, scaffold, and planned status.

### Canonical Data Flow

Input enters through a host, SDK, contract, peer, checkpoint, replay archive, or package boundary. Processing validates schema, version, signature, hash ancestry, and capability scope. State mutation occurs only through deterministic modules that own the relevant state. Receipt generation records roots, input hashes, output hashes, execution roots, state changes, snapshots, and protocol epoch metadata. Verification reruns execution or compares signed roots, receipt chains, checkpoint ancestry, and continuity records.

### Determinism and Validation

Hashing is the common language between modules. Canonical state roots prove the state that resulted from execution. Execution roots summarize the planned work. Receipt hashes summarize evidence. Checkpoint roots and snapshot hashes bridge recovery. Continuity roots and lineage records explain why later history descends from earlier history. Canonical serialization is mandatory before hashing. A module that serializes the same logical value differently on two machines cannot be authoritative.

Replay is the practical test of determinism. If a verifier receives the same prior state, input, protocol epoch, and contract package, it must recompute the same roots and receipts. If it cannot, EverArcade treats that as divergence, not as a renderer bug or UX inconsistency. Divergence is resolved by comparing roots, choosing valid lineage, restoring checkpoints, exchanging missing receipt ranges, and quarantining invalid peers or artifacts.

### Failure and Recovery Pattern

Corruption is detected by mismatched hashes, invalid signatures, malformed manifests, invalid proof material, or checkpoint ancestry gaps. Divergence is detected by root comparison and replay. Recovery chooses the smallest safe repair: import missing receipts, hydrate an archive, restore a checkpoint, roll back a window, resynchronize from a peer, or rebuild an index. Reconciliation must never invent authoritative state; it must select from verifiable material or recompute from canonical inputs.

## Future Evolution

Future work should turn partial and scaffold areas into validated runtime paths only when they have deterministic tests, operational runbooks, release gates, and clear owner modules.


# Topology

## Purpose

Topology models who participates, what capabilities they advertise, which trust scope applies, what assignments they own, and how membership epochs evolve. Topology is security-sensitive because an invalid peer or assignment can waste resources or attempt to poison synchronization.

## Responsibilities

- Preserve deterministic authority and explicit ownership.
- Keep inputs, state changes, receipts, roots, checkpoints, replay material, and synchronization artifacts verifiable.
- Maintain contributor clarity by distinguishing implementation evidence from roadmap intent.

## Non-Responsibilities

- This subsystem narrative does not claim every named module is production-grade.
- It does not move authority into renderer, dashboard, observer, or convenience tooling paths.
- It does not replace targeted source review and tests before changing Rust behavior.

## Internal Components

- Source modules named in the subsystem chapters.
- Tests, fixtures, validation reports, and runbooks that support the architecture claim.
- Roadmap rows that identify implemented, partial, scaffold, and planned status.

### Canonical Data Flow

Input enters through a host, SDK, contract, peer, checkpoint, replay archive, or package boundary. Processing validates schema, version, signature, hash ancestry, and capability scope. State mutation occurs only through deterministic modules that own the relevant state. Receipt generation records roots, input hashes, output hashes, execution roots, state changes, snapshots, and protocol epoch metadata. Verification reruns execution or compares signed roots, receipt chains, checkpoint ancestry, and continuity records.

### Determinism and Validation

Hashing is the common language between modules. Canonical state roots prove the state that resulted from execution. Execution roots summarize the planned work. Receipt hashes summarize evidence. Checkpoint roots and snapshot hashes bridge recovery. Continuity roots and lineage records explain why later history descends from earlier history. Canonical serialization is mandatory before hashing. A module that serializes the same logical value differently on two machines cannot be authoritative.

Replay is the practical test of determinism. If a verifier receives the same prior state, input, protocol epoch, and contract package, it must recompute the same roots and receipts. If it cannot, EverArcade treats that as divergence, not as a renderer bug or UX inconsistency. Divergence is resolved by comparing roots, choosing valid lineage, restoring checkpoints, exchanging missing receipt ranges, and quarantining invalid peers or artifacts.

### Failure and Recovery Pattern

Corruption is detected by mismatched hashes, invalid signatures, malformed manifests, invalid proof material, or checkpoint ancestry gaps. Divergence is detected by root comparison and replay. Recovery chooses the smallest safe repair: import missing receipts, hydrate an archive, restore a checkpoint, roll back a window, resynchronize from a peer, or rebuild an index. Reconciliation must never invent authoritative state; it must select from verifiable material or recompute from canonical inputs.

## Future Evolution

Future work should turn partial and scaffold areas into validated runtime paths only when they have deterministic tests, operational runbooks, release gates, and clear owner modules.


# World Runtime

## Purpose

The world runtime layers persistent game/world semantics on top of deterministic execution. It owns ticks, world state, continuity, validation, restoration, and persistent runtime loops. It must never bypass execution evidence; world state is valuable only when reproducible.

## Responsibilities

- Preserve deterministic authority and explicit ownership.
- Keep inputs, state changes, receipts, roots, checkpoints, replay material, and synchronization artifacts verifiable.
- Maintain contributor clarity by distinguishing implementation evidence from roadmap intent.

## Non-Responsibilities

- This subsystem narrative does not claim every named module is production-grade.
- It does not move authority into renderer, dashboard, observer, or convenience tooling paths.
- It does not replace targeted source review and tests before changing Rust behavior.

## Internal Components

- Source modules named in the subsystem chapters.
- Tests, fixtures, validation reports, and runbooks that support the architecture claim.
- Roadmap rows that identify implemented, partial, scaffold, and planned status.

### Canonical Data Flow

Input enters through a host, SDK, contract, peer, checkpoint, replay archive, or package boundary. Processing validates schema, version, signature, hash ancestry, and capability scope. State mutation occurs only through deterministic modules that own the relevant state. Receipt generation records roots, input hashes, output hashes, execution roots, state changes, snapshots, and protocol epoch metadata. Verification reruns execution or compares signed roots, receipt chains, checkpoint ancestry, and continuity records.

### Determinism and Validation

Hashing is the common language between modules. Canonical state roots prove the state that resulted from execution. Execution roots summarize the planned work. Receipt hashes summarize evidence. Checkpoint roots and snapshot hashes bridge recovery. Continuity roots and lineage records explain why later history descends from earlier history. Canonical serialization is mandatory before hashing. A module that serializes the same logical value differently on two machines cannot be authoritative.

Replay is the practical test of determinism. If a verifier receives the same prior state, input, protocol epoch, and contract package, it must recompute the same roots and receipts. If it cannot, EverArcade treats that as divergence, not as a renderer bug or UX inconsistency. Divergence is resolved by comparing roots, choosing valid lineage, restoring checkpoints, exchanging missing receipt ranges, and quarantining invalid peers or artifacts.

### Failure and Recovery Pattern

Corruption is detected by mismatched hashes, invalid signatures, malformed manifests, invalid proof material, or checkpoint ancestry gaps. Divergence is detected by root comparison and replay. Recovery chooses the smallest safe repair: import missing receipts, hydrate an archive, restore a checkpoint, roll back a window, resynchronize from a peer, or rebuild an index. Reconciliation must never invent authoritative state; it must select from verifiable material or recompute from canonical inputs.

## Future Evolution

Future work should turn partial and scaffold areas into validated runtime paths only when they have deterministic tests, operational runbooks, release gates, and clear owner modules.


# Simulation

## Purpose

Simulation modules cover deterministic gameplay, AI memory, ecology, society, factions, civilization behavior, and procedural world behavior. These are higher-level domains. They are useful only when scheduled deterministically and committed through roots/receipts/checkpoints.

## Responsibilities

- Preserve deterministic authority and explicit ownership.
- Keep inputs, state changes, receipts, roots, checkpoints, replay material, and synchronization artifacts verifiable.
- Maintain contributor clarity by distinguishing implementation evidence from roadmap intent.

## Non-Responsibilities

- This subsystem narrative does not claim every named module is production-grade.
- It does not move authority into renderer, dashboard, observer, or convenience tooling paths.
- It does not replace targeted source review and tests before changing Rust behavior.

## Internal Components

- Source modules named in the subsystem chapters.
- Tests, fixtures, validation reports, and runbooks that support the architecture claim.
- Roadmap rows that identify implemented, partial, scaffold, and planned status.

### Canonical Data Flow

Input enters through a host, SDK, contract, peer, checkpoint, replay archive, or package boundary. Processing validates schema, version, signature, hash ancestry, and capability scope. State mutation occurs only through deterministic modules that own the relevant state. Receipt generation records roots, input hashes, output hashes, execution roots, state changes, snapshots, and protocol epoch metadata. Verification reruns execution or compares signed roots, receipt chains, checkpoint ancestry, and continuity records.

### Determinism and Validation

Hashing is the common language between modules. Canonical state roots prove the state that resulted from execution. Execution roots summarize the planned work. Receipt hashes summarize evidence. Checkpoint roots and snapshot hashes bridge recovery. Continuity roots and lineage records explain why later history descends from earlier history. Canonical serialization is mandatory before hashing. A module that serializes the same logical value differently on two machines cannot be authoritative.

Replay is the practical test of determinism. If a verifier receives the same prior state, input, protocol epoch, and contract package, it must recompute the same roots and receipts. If it cannot, EverArcade treats that as divergence, not as a renderer bug or UX inconsistency. Divergence is resolved by comparing roots, choosing valid lineage, restoring checkpoints, exchanging missing receipt ranges, and quarantining invalid peers or artifacts.

### Failure and Recovery Pattern

Corruption is detected by mismatched hashes, invalid signatures, malformed manifests, invalid proof material, or checkpoint ancestry gaps. Divergence is detected by root comparison and replay. Recovery chooses the smallest safe repair: import missing receipts, hydrate an archive, restore a checkpoint, roll back a window, resynchronize from a peer, or rebuild an index. Reconciliation must never invent authoritative state; it must select from verifiable material or recompute from canonical inputs.

## Future Evolution

Future work should turn partial and scaffold areas into validated runtime paths only when they have deterministic tests, operational runbooks, release gates, and clear owner modules.


# Scheduler

## Purpose

Schedulers decide ordering. In deterministic systems, scheduling is protocol behavior, not incidental implementation. Entity order, lane order, migration order, tick order, governance windows, and partition execution order must be stable and replayable.

## Responsibilities

- Preserve deterministic authority and explicit ownership.
- Keep inputs, state changes, receipts, roots, checkpoints, replay material, and synchronization artifacts verifiable.
- Maintain contributor clarity by distinguishing implementation evidence from roadmap intent.

## Non-Responsibilities

- This subsystem narrative does not claim every named module is production-grade.
- It does not move authority into renderer, dashboard, observer, or convenience tooling paths.
- It does not replace targeted source review and tests before changing Rust behavior.

## Internal Components

- Source modules named in the subsystem chapters.
- Tests, fixtures, validation reports, and runbooks that support the architecture claim.
- Roadmap rows that identify implemented, partial, scaffold, and planned status.

### Canonical Data Flow

Input enters through a host, SDK, contract, peer, checkpoint, replay archive, or package boundary. Processing validates schema, version, signature, hash ancestry, and capability scope. State mutation occurs only through deterministic modules that own the relevant state. Receipt generation records roots, input hashes, output hashes, execution roots, state changes, snapshots, and protocol epoch metadata. Verification reruns execution or compares signed roots, receipt chains, checkpoint ancestry, and continuity records.

### Determinism and Validation

Hashing is the common language between modules. Canonical state roots prove the state that resulted from execution. Execution roots summarize the planned work. Receipt hashes summarize evidence. Checkpoint roots and snapshot hashes bridge recovery. Continuity roots and lineage records explain why later history descends from earlier history. Canonical serialization is mandatory before hashing. A module that serializes the same logical value differently on two machines cannot be authoritative.

Replay is the practical test of determinism. If a verifier receives the same prior state, input, protocol epoch, and contract package, it must recompute the same roots and receipts. If it cannot, EverArcade treats that as divergence, not as a renderer bug or UX inconsistency. Divergence is resolved by comparing roots, choosing valid lineage, restoring checkpoints, exchanging missing receipt ranges, and quarantining invalid peers or artifacts.

### Failure and Recovery Pattern

Corruption is detected by mismatched hashes, invalid signatures, malformed manifests, invalid proof material, or checkpoint ancestry gaps. Divergence is detected by root comparison and replay. Recovery chooses the smallest safe repair: import missing receipts, hydrate an archive, restore a checkpoint, roll back a window, resynchronize from a peer, or rebuild an index. Reconciliation must never invent authoritative state; it must select from verifiable material or recompute from canonical inputs.

## Future Evolution

Future work should turn partial and scaffold areas into validated runtime paths only when they have deterministic tests, operational runbooks, release gates, and clear owner modules.


# Economy

## Purpose

The economy runtime handles ledgers, settlement windows, treasury, budgets, fiscal execution, and economic validation. Economic state requires especially strong replay and audit trails because it is likely to affect assets, rewards, fees, and settlement gateways.

## Responsibilities

- Preserve deterministic authority and explicit ownership.
- Keep inputs, state changes, receipts, roots, checkpoints, replay material, and synchronization artifacts verifiable.
- Maintain contributor clarity by distinguishing implementation evidence from roadmap intent.

## Non-Responsibilities

- This subsystem narrative does not claim every named module is production-grade.
- It does not move authority into renderer, dashboard, observer, or convenience tooling paths.
- It does not replace targeted source review and tests before changing Rust behavior.

## Internal Components

- Source modules named in the subsystem chapters.
- Tests, fixtures, validation reports, and runbooks that support the architecture claim.
- Roadmap rows that identify implemented, partial, scaffold, and planned status.

### Canonical Data Flow

Input enters through a host, SDK, contract, peer, checkpoint, replay archive, or package boundary. Processing validates schema, version, signature, hash ancestry, and capability scope. State mutation occurs only through deterministic modules that own the relevant state. Receipt generation records roots, input hashes, output hashes, execution roots, state changes, snapshots, and protocol epoch metadata. Verification reruns execution or compares signed roots, receipt chains, checkpoint ancestry, and continuity records.

### Determinism and Validation

Hashing is the common language between modules. Canonical state roots prove the state that resulted from execution. Execution roots summarize the planned work. Receipt hashes summarize evidence. Checkpoint roots and snapshot hashes bridge recovery. Continuity roots and lineage records explain why later history descends from earlier history. Canonical serialization is mandatory before hashing. A module that serializes the same logical value differently on two machines cannot be authoritative.

Replay is the practical test of determinism. If a verifier receives the same prior state, input, protocol epoch, and contract package, it must recompute the same roots and receipts. If it cannot, EverArcade treats that as divergence, not as a renderer bug or UX inconsistency. Divergence is resolved by comparing roots, choosing valid lineage, restoring checkpoints, exchanging missing receipt ranges, and quarantining invalid peers or artifacts.

### Failure and Recovery Pattern

Corruption is detected by mismatched hashes, invalid signatures, malformed manifests, invalid proof material, or checkpoint ancestry gaps. Divergence is detected by root comparison and replay. Recovery chooses the smallest safe repair: import missing receipts, hydrate an archive, restore a checkpoint, roll back a window, resynchronize from a peer, or rebuild an index. Reconciliation must never invent authoritative state; it must select from verifiable material or recompute from canonical inputs.

## Future Evolution

Future work should turn partial and scaffold areas into validated runtime paths only when they have deterministic tests, operational runbooks, release gates, and clear owner modules.


# Governance

## Purpose

Governance models proposals, votes, quorums, treaties, laws, diplomacy, conflict, and continuity. Governance cannot be a loose side channel; it must execute in windows and produce roots/commits that can be replayed and synchronized.

## Responsibilities

- Preserve deterministic authority and explicit ownership.
- Keep inputs, state changes, receipts, roots, checkpoints, replay material, and synchronization artifacts verifiable.
- Maintain contributor clarity by distinguishing implementation evidence from roadmap intent.

## Non-Responsibilities

- This subsystem narrative does not claim every named module is production-grade.
- It does not move authority into renderer, dashboard, observer, or convenience tooling paths.
- It does not replace targeted source review and tests before changing Rust behavior.

## Internal Components

- Source modules named in the subsystem chapters.
- Tests, fixtures, validation reports, and runbooks that support the architecture claim.
- Roadmap rows that identify implemented, partial, scaffold, and planned status.

### Canonical Data Flow

Input enters through a host, SDK, contract, peer, checkpoint, replay archive, or package boundary. Processing validates schema, version, signature, hash ancestry, and capability scope. State mutation occurs only through deterministic modules that own the relevant state. Receipt generation records roots, input hashes, output hashes, execution roots, state changes, snapshots, and protocol epoch metadata. Verification reruns execution or compares signed roots, receipt chains, checkpoint ancestry, and continuity records.

### Determinism and Validation

Hashing is the common language between modules. Canonical state roots prove the state that resulted from execution. Execution roots summarize the planned work. Receipt hashes summarize evidence. Checkpoint roots and snapshot hashes bridge recovery. Continuity roots and lineage records explain why later history descends from earlier history. Canonical serialization is mandatory before hashing. A module that serializes the same logical value differently on two machines cannot be authoritative.

Replay is the practical test of determinism. If a verifier receives the same prior state, input, protocol epoch, and contract package, it must recompute the same roots and receipts. If it cannot, EverArcade treats that as divergence, not as a renderer bug or UX inconsistency. Divergence is resolved by comparing roots, choosing valid lineage, restoring checkpoints, exchanging missing receipt ranges, and quarantining invalid peers or artifacts.

### Failure and Recovery Pattern

Corruption is detected by mismatched hashes, invalid signatures, malformed manifests, invalid proof material, or checkpoint ancestry gaps. Divergence is detected by root comparison and replay. Recovery chooses the smallest safe repair: import missing receipts, hydrate an archive, restore a checkpoint, roll back a window, resynchronize from a peer, or rebuild an index. Reconciliation must never invent authoritative state; it must select from verifiable material or recompute from canonical inputs.

## Future Evolution

Future work should turn partial and scaffold areas into validated runtime paths only when they have deterministic tests, operational runbooks, release gates, and clear owner modules.


# Inventory

## Purpose

Inventory models items, ownership, transfer, and validation. The current architecture treats inventory as a partial/scaffold domain: useful structures exist, but production game-facing APIs and policy are not complete.

## Responsibilities

- Preserve deterministic authority and explicit ownership.
- Keep inputs, state changes, receipts, roots, checkpoints, replay material, and synchronization artifacts verifiable.
- Maintain contributor clarity by distinguishing implementation evidence from roadmap intent.

## Non-Responsibilities

- This subsystem narrative does not claim every named module is production-grade.
- It does not move authority into renderer, dashboard, observer, or convenience tooling paths.
- It does not replace targeted source review and tests before changing Rust behavior.

## Internal Components

- Source modules named in the subsystem chapters.
- Tests, fixtures, validation reports, and runbooks that support the architecture claim.
- Roadmap rows that identify implemented, partial, scaffold, and planned status.

### Canonical Data Flow

Input enters through a host, SDK, contract, peer, checkpoint, replay archive, or package boundary. Processing validates schema, version, signature, hash ancestry, and capability scope. State mutation occurs only through deterministic modules that own the relevant state. Receipt generation records roots, input hashes, output hashes, execution roots, state changes, snapshots, and protocol epoch metadata. Verification reruns execution or compares signed roots, receipt chains, checkpoint ancestry, and continuity records.

### Determinism and Validation

Hashing is the common language between modules. Canonical state roots prove the state that resulted from execution. Execution roots summarize the planned work. Receipt hashes summarize evidence. Checkpoint roots and snapshot hashes bridge recovery. Continuity roots and lineage records explain why later history descends from earlier history. Canonical serialization is mandatory before hashing. A module that serializes the same logical value differently on two machines cannot be authoritative.

Replay is the practical test of determinism. If a verifier receives the same prior state, input, protocol epoch, and contract package, it must recompute the same roots and receipts. If it cannot, EverArcade treats that as divergence, not as a renderer bug or UX inconsistency. Divergence is resolved by comparing roots, choosing valid lineage, restoring checkpoints, exchanging missing receipt ranges, and quarantining invalid peers or artifacts.

### Failure and Recovery Pattern

Corruption is detected by mismatched hashes, invalid signatures, malformed manifests, invalid proof material, or checkpoint ancestry gaps. Divergence is detected by root comparison and replay. Recovery chooses the smallest safe repair: import missing receipts, hydrate an archive, restore a checkpoint, roll back a window, resynchronize from a peer, or rebuild an index. Reconciliation must never invent authoritative state; it must select from verifiable material or recompute from canonical inputs.

## Future Evolution

Future work should turn partial and scaffold areas into validated runtime paths only when they have deterministic tests, operational runbooks, release gates, and clear owner modules.


# Partitioning

## Purpose

Partitioning splits work across regions, owners, assignments, migrations, and partition roots. Partitioning is necessary for scale but dangerous if it weakens determinism. Assignment lineage and partition continuity protect migration and failover.

## Responsibilities

- Preserve deterministic authority and explicit ownership.
- Keep inputs, state changes, receipts, roots, checkpoints, replay material, and synchronization artifacts verifiable.
- Maintain contributor clarity by distinguishing implementation evidence from roadmap intent.

## Non-Responsibilities

- This subsystem narrative does not claim every named module is production-grade.
- It does not move authority into renderer, dashboard, observer, or convenience tooling paths.
- It does not replace targeted source review and tests before changing Rust behavior.

## Internal Components

- Source modules named in the subsystem chapters.
- Tests, fixtures, validation reports, and runbooks that support the architecture claim.
- Roadmap rows that identify implemented, partial, scaffold, and planned status.

### Canonical Data Flow

Input enters through a host, SDK, contract, peer, checkpoint, replay archive, or package boundary. Processing validates schema, version, signature, hash ancestry, and capability scope. State mutation occurs only through deterministic modules that own the relevant state. Receipt generation records roots, input hashes, output hashes, execution roots, state changes, snapshots, and protocol epoch metadata. Verification reruns execution or compares signed roots, receipt chains, checkpoint ancestry, and continuity records.

### Determinism and Validation

Hashing is the common language between modules. Canonical state roots prove the state that resulted from execution. Execution roots summarize the planned work. Receipt hashes summarize evidence. Checkpoint roots and snapshot hashes bridge recovery. Continuity roots and lineage records explain why later history descends from earlier history. Canonical serialization is mandatory before hashing. A module that serializes the same logical value differently on two machines cannot be authoritative.

Replay is the practical test of determinism. If a verifier receives the same prior state, input, protocol epoch, and contract package, it must recompute the same roots and receipts. If it cannot, EverArcade treats that as divergence, not as a renderer bug or UX inconsistency. Divergence is resolved by comparing roots, choosing valid lineage, restoring checkpoints, exchanging missing receipt ranges, and quarantining invalid peers or artifacts.

### Failure and Recovery Pattern

Corruption is detected by mismatched hashes, invalid signatures, malformed manifests, invalid proof material, or checkpoint ancestry gaps. Divergence is detected by root comparison and replay. Recovery chooses the smallest safe repair: import missing receipts, hydrate an archive, restore a checkpoint, roll back a window, resynchronize from a peer, or rebuild an index. Reconciliation must never invent authoritative state; it must select from verifiable material or recompute from canonical inputs.

## Future Evolution

Future work should turn partial and scaffold areas into validated runtime paths only when they have deterministic tests, operational runbooks, release gates, and clear owner modules.


# Renderer Projection

## Purpose

Renderer projection converts authoritative facts into non-authoritative frames. The renderer may display state, HUDs, events, inventory, historical windows, or debugger views, but it does not create authoritative state. Projection roots and validation reports help prove display equivalence where needed.

## Responsibilities

- Preserve deterministic authority and explicit ownership.
- Keep inputs, state changes, receipts, roots, checkpoints, replay material, and synchronization artifacts verifiable.
- Maintain contributor clarity by distinguishing implementation evidence from roadmap intent.

## Non-Responsibilities

- This subsystem narrative does not claim every named module is production-grade.
- It does not move authority into renderer, dashboard, observer, or convenience tooling paths.
- It does not replace targeted source review and tests before changing Rust behavior.

## Internal Components

- Source modules named in the subsystem chapters.
- Tests, fixtures, validation reports, and runbooks that support the architecture claim.
- Roadmap rows that identify implemented, partial, scaffold, and planned status.

### Canonical Data Flow

Input enters through a host, SDK, contract, peer, checkpoint, replay archive, or package boundary. Processing validates schema, version, signature, hash ancestry, and capability scope. State mutation occurs only through deterministic modules that own the relevant state. Receipt generation records roots, input hashes, output hashes, execution roots, state changes, snapshots, and protocol epoch metadata. Verification reruns execution or compares signed roots, receipt chains, checkpoint ancestry, and continuity records.

### Determinism and Validation

Hashing is the common language between modules. Canonical state roots prove the state that resulted from execution. Execution roots summarize the planned work. Receipt hashes summarize evidence. Checkpoint roots and snapshot hashes bridge recovery. Continuity roots and lineage records explain why later history descends from earlier history. Canonical serialization is mandatory before hashing. A module that serializes the same logical value differently on two machines cannot be authoritative.

Replay is the practical test of determinism. If a verifier receives the same prior state, input, protocol epoch, and contract package, it must recompute the same roots and receipts. If it cannot, EverArcade treats that as divergence, not as a renderer bug or UX inconsistency. Divergence is resolved by comparing roots, choosing valid lineage, restoring checkpoints, exchanging missing receipt ranges, and quarantining invalid peers or artifacts.

### Failure and Recovery Pattern

Corruption is detected by mismatched hashes, invalid signatures, malformed manifests, invalid proof material, or checkpoint ancestry gaps. Divergence is detected by root comparison and replay. Recovery chooses the smallest safe repair: import missing receipts, hydrate an archive, restore a checkpoint, roll back a window, resynchronize from a peer, or rebuild an index. Reconciliation must never invent authoritative state; it must select from verifiable material or recompute from canonical inputs.

## Future Evolution

Future work should turn partial and scaffold areas into validated runtime paths only when they have deterministic tests, operational runbooks, release gates, and clear owner modules.


# Replay Transport and Historical Replay

## Purpose

Replay transport delivers history to observers and clients. Historical replay supports branches, archives, eras, hydration, materialization, cache, observer sessions, stream windows, and proof verification. This domain is scaffold-to-partial and must not be confused with execution authority.

## Responsibilities

- Preserve deterministic authority and explicit ownership.
- Keep inputs, state changes, receipts, roots, checkpoints, replay material, and synchronization artifacts verifiable.
- Maintain contributor clarity by distinguishing implementation evidence from roadmap intent.

## Non-Responsibilities

- This subsystem narrative does not claim every named module is production-grade.
- It does not move authority into renderer, dashboard, observer, or convenience tooling paths.
- It does not replace targeted source review and tests before changing Rust behavior.

## Internal Components

- Source modules named in the subsystem chapters.
- Tests, fixtures, validation reports, and runbooks that support the architecture claim.
- Roadmap rows that identify implemented, partial, scaffold, and planned status.

### Canonical Data Flow

Input enters through a host, SDK, contract, peer, checkpoint, replay archive, or package boundary. Processing validates schema, version, signature, hash ancestry, and capability scope. State mutation occurs only through deterministic modules that own the relevant state. Receipt generation records roots, input hashes, output hashes, execution roots, state changes, snapshots, and protocol epoch metadata. Verification reruns execution or compares signed roots, receipt chains, checkpoint ancestry, and continuity records.

### Determinism and Validation

Hashing is the common language between modules. Canonical state roots prove the state that resulted from execution. Execution roots summarize the planned work. Receipt hashes summarize evidence. Checkpoint roots and snapshot hashes bridge recovery. Continuity roots and lineage records explain why later history descends from earlier history. Canonical serialization is mandatory before hashing. A module that serializes the same logical value differently on two machines cannot be authoritative.

Replay is the practical test of determinism. If a verifier receives the same prior state, input, protocol epoch, and contract package, it must recompute the same roots and receipts. If it cannot, EverArcade treats that as divergence, not as a renderer bug or UX inconsistency. Divergence is resolved by comparing roots, choosing valid lineage, restoring checkpoints, exchanging missing receipt ranges, and quarantining invalid peers or artifacts.

### Failure and Recovery Pattern

Corruption is detected by mismatched hashes, invalid signatures, malformed manifests, invalid proof material, or checkpoint ancestry gaps. Divergence is detected by root comparison and replay. Recovery chooses the smallest safe repair: import missing receipts, hydrate an archive, restore a checkpoint, roll back a window, resynchronize from a peer, or rebuild an index. Reconciliation must never invent authoritative state; it must select from verifiable material or recompute from canonical inputs.

## Future Evolution

Future work should turn partial and scaffold areas into validated runtime paths only when they have deterministic tests, operational runbooks, release gates, and clear owner modules.


# Deployment and Distribution

## Purpose

Deployment packages runtimes, contracts, assets, manifests, state folders, release artifacts, and provider metadata. Reproducible builds and signed artifacts are necessary because a deterministic protocol still fails if operators cannot know which binary/package produced a receipt.

## Responsibilities

- Preserve deterministic authority and explicit ownership.
- Keep inputs, state changes, receipts, roots, checkpoints, replay material, and synchronization artifacts verifiable.
- Maintain contributor clarity by distinguishing implementation evidence from roadmap intent.

## Non-Responsibilities

- This subsystem narrative does not claim every named module is production-grade.
- It does not move authority into renderer, dashboard, observer, or convenience tooling paths.
- It does not replace targeted source review and tests before changing Rust behavior.

## Internal Components

- Source modules named in the subsystem chapters.
- Tests, fixtures, validation reports, and runbooks that support the architecture claim.
- Roadmap rows that identify implemented, partial, scaffold, and planned status.

### Canonical Data Flow

Input enters through a host, SDK, contract, peer, checkpoint, replay archive, or package boundary. Processing validates schema, version, signature, hash ancestry, and capability scope. State mutation occurs only through deterministic modules that own the relevant state. Receipt generation records roots, input hashes, output hashes, execution roots, state changes, snapshots, and protocol epoch metadata. Verification reruns execution or compares signed roots, receipt chains, checkpoint ancestry, and continuity records.

### Determinism and Validation

Hashing is the common language between modules. Canonical state roots prove the state that resulted from execution. Execution roots summarize the planned work. Receipt hashes summarize evidence. Checkpoint roots and snapshot hashes bridge recovery. Continuity roots and lineage records explain why later history descends from earlier history. Canonical serialization is mandatory before hashing. A module that serializes the same logical value differently on two machines cannot be authoritative.

Replay is the practical test of determinism. If a verifier receives the same prior state, input, protocol epoch, and contract package, it must recompute the same roots and receipts. If it cannot, EverArcade treats that as divergence, not as a renderer bug or UX inconsistency. Divergence is resolved by comparing roots, choosing valid lineage, restoring checkpoints, exchanging missing receipt ranges, and quarantining invalid peers or artifacts.

### Failure and Recovery Pattern

Corruption is detected by mismatched hashes, invalid signatures, malformed manifests, invalid proof material, or checkpoint ancestry gaps. Divergence is detected by root comparison and replay. Recovery chooses the smallest safe repair: import missing receipts, hydrate an archive, restore a checkpoint, roll back a window, resynchronize from a peer, or rebuild an index. Reconciliation must never invent authoritative state; it must select from verifiable material or recompute from canonical inputs.

## Future Evolution

Future work should turn partial and scaffold areas into validated runtime paths only when they have deterministic tests, operational runbooks, release gates, and clear owner modules.


# Release Certification

## Purpose

Release certification converts validation reports into gates. The repository already contains many reports for CI, replay, runtime, release integrity, signatures, lineage, certification, performance, and readiness. The next maturity step is enforcing these as automated release criteria.

## Responsibilities

- Preserve deterministic authority and explicit ownership.
- Keep inputs, state changes, receipts, roots, checkpoints, replay material, and synchronization artifacts verifiable.
- Maintain contributor clarity by distinguishing implementation evidence from roadmap intent.

## Non-Responsibilities

- This subsystem narrative does not claim every named module is production-grade.
- It does not move authority into renderer, dashboard, observer, or convenience tooling paths.
- It does not replace targeted source review and tests before changing Rust behavior.

## Internal Components

- Source modules named in the subsystem chapters.
- Tests, fixtures, validation reports, and runbooks that support the architecture claim.
- Roadmap rows that identify implemented, partial, scaffold, and planned status.

### Canonical Data Flow

Input enters through a host, SDK, contract, peer, checkpoint, replay archive, or package boundary. Processing validates schema, version, signature, hash ancestry, and capability scope. State mutation occurs only through deterministic modules that own the relevant state. Receipt generation records roots, input hashes, output hashes, execution roots, state changes, snapshots, and protocol epoch metadata. Verification reruns execution or compares signed roots, receipt chains, checkpoint ancestry, and continuity records.

### Determinism and Validation

Hashing is the common language between modules. Canonical state roots prove the state that resulted from execution. Execution roots summarize the planned work. Receipt hashes summarize evidence. Checkpoint roots and snapshot hashes bridge recovery. Continuity roots and lineage records explain why later history descends from earlier history. Canonical serialization is mandatory before hashing. A module that serializes the same logical value differently on two machines cannot be authoritative.

Replay is the practical test of determinism. If a verifier receives the same prior state, input, protocol epoch, and contract package, it must recompute the same roots and receipts. If it cannot, EverArcade treats that as divergence, not as a renderer bug or UX inconsistency. Divergence is resolved by comparing roots, choosing valid lineage, restoring checkpoints, exchanging missing receipt ranges, and quarantining invalid peers or artifacts.

### Failure and Recovery Pattern

Corruption is detected by mismatched hashes, invalid signatures, malformed manifests, invalid proof material, or checkpoint ancestry gaps. Divergence is detected by root comparison and replay. Recovery chooses the smallest safe repair: import missing receipts, hydrate an archive, restore a checkpoint, roll back a window, resynchronize from a peer, or rebuild an index. Reconciliation must never invent authoritative state; it must select from verifiable material or recompute from canonical inputs.

## Future Evolution

Future work should turn partial and scaffold areas into validated runtime paths only when they have deterministic tests, operational runbooks, release gates, and clear owner modules.


# Evernode Deployment

## Purpose

Evernode deployment describes provider-hosted runtime instances, peer manifests, capacity advertisements, anchor intents, state manifests, recovery discovery, and operator availability. The design points toward provider-grade operation, but the status remains partial until automation and observability are hardened.

## Responsibilities

- Preserve deterministic authority and explicit ownership.
- Keep inputs, state changes, receipts, roots, checkpoints, replay material, and synchronization artifacts verifiable.
- Maintain contributor clarity by distinguishing implementation evidence from roadmap intent.

## Non-Responsibilities

- This subsystem narrative does not claim every named module is production-grade.
- It does not move authority into renderer, dashboard, observer, or convenience tooling paths.
- It does not replace targeted source review and tests before changing Rust behavior.

## Internal Components

- Source modules named in the subsystem chapters.
- Tests, fixtures, validation reports, and runbooks that support the architecture claim.
- Roadmap rows that identify implemented, partial, scaffold, and planned status.

### Canonical Data Flow

Input enters through a host, SDK, contract, peer, checkpoint, replay archive, or package boundary. Processing validates schema, version, signature, hash ancestry, and capability scope. State mutation occurs only through deterministic modules that own the relevant state. Receipt generation records roots, input hashes, output hashes, execution roots, state changes, snapshots, and protocol epoch metadata. Verification reruns execution or compares signed roots, receipt chains, checkpoint ancestry, and continuity records.

### Determinism and Validation

Hashing is the common language between modules. Canonical state roots prove the state that resulted from execution. Execution roots summarize the planned work. Receipt hashes summarize evidence. Checkpoint roots and snapshot hashes bridge recovery. Continuity roots and lineage records explain why later history descends from earlier history. Canonical serialization is mandatory before hashing. A module that serializes the same logical value differently on two machines cannot be authoritative.

Replay is the practical test of determinism. If a verifier receives the same prior state, input, protocol epoch, and contract package, it must recompute the same roots and receipts. If it cannot, EverArcade treats that as divergence, not as a renderer bug or UX inconsistency. Divergence is resolved by comparing roots, choosing valid lineage, restoring checkpoints, exchanging missing receipt ranges, and quarantining invalid peers or artifacts.

### Failure and Recovery Pattern

Corruption is detected by mismatched hashes, invalid signatures, malformed manifests, invalid proof material, or checkpoint ancestry gaps. Divergence is detected by root comparison and replay. Recovery chooses the smallest safe repair: import missing receipts, hydrate an archive, restore a checkpoint, roll back a window, resynchronize from a peer, or rebuild an index. Reconciliation must never invent authoritative state; it must select from verifiable material or recompute from canonical inputs.

## Future Evolution

Future work should turn partial and scaffold areas into validated runtime paths only when they have deterministic tests, operational runbooks, release gates, and clear owner modules.


# Operational Recovery

## Purpose

Operators need runbooks for startup, machine recovery, machine rejoin, checkpoint restore, transport failure, rollback, resync, and divergence. Good recovery procedures are protocol features because a bad operator action can create unverifiable history.

## Responsibilities

- Preserve deterministic authority and explicit ownership.
- Keep inputs, state changes, receipts, roots, checkpoints, replay material, and synchronization artifacts verifiable.
- Maintain contributor clarity by distinguishing implementation evidence from roadmap intent.

## Non-Responsibilities

- This subsystem narrative does not claim every named module is production-grade.
- It does not move authority into renderer, dashboard, observer, or convenience tooling paths.
- It does not replace targeted source review and tests before changing Rust behavior.

## Internal Components

- Source modules named in the subsystem chapters.
- Tests, fixtures, validation reports, and runbooks that support the architecture claim.
- Roadmap rows that identify implemented, partial, scaffold, and planned status.

### Canonical Data Flow

Input enters through a host, SDK, contract, peer, checkpoint, replay archive, or package boundary. Processing validates schema, version, signature, hash ancestry, and capability scope. State mutation occurs only through deterministic modules that own the relevant state. Receipt generation records roots, input hashes, output hashes, execution roots, state changes, snapshots, and protocol epoch metadata. Verification reruns execution or compares signed roots, receipt chains, checkpoint ancestry, and continuity records.

### Determinism and Validation

Hashing is the common language between modules. Canonical state roots prove the state that resulted from execution. Execution roots summarize the planned work. Receipt hashes summarize evidence. Checkpoint roots and snapshot hashes bridge recovery. Continuity roots and lineage records explain why later history descends from earlier history. Canonical serialization is mandatory before hashing. A module that serializes the same logical value differently on two machines cannot be authoritative.

Replay is the practical test of determinism. If a verifier receives the same prior state, input, protocol epoch, and contract package, it must recompute the same roots and receipts. If it cannot, EverArcade treats that as divergence, not as a renderer bug or UX inconsistency. Divergence is resolved by comparing roots, choosing valid lineage, restoring checkpoints, exchanging missing receipt ranges, and quarantining invalid peers or artifacts.

### Failure and Recovery Pattern

Corruption is detected by mismatched hashes, invalid signatures, malformed manifests, invalid proof material, or checkpoint ancestry gaps. Divergence is detected by root comparison and replay. Recovery chooses the smallest safe repair: import missing receipts, hydrate an archive, restore a checkpoint, roll back a window, resynchronize from a peer, or rebuild an index. Reconciliation must never invent authoritative state; it must select from verifiable material or recompute from canonical inputs.

## Future Evolution

Future work should turn partial and scaffold areas into validated runtime paths only when they have deterministic tests, operational runbooks, release gates, and clear owner modules.


# SDK and Game Development

## Purpose

SDK crates help developers build games, worlds, simulations, economies, entities, governance logic, and client bridges. Templates and examples provide starting points. The SDK should hide incidental complexity without hiding determinism rules.

## Responsibilities

- Preserve deterministic authority and explicit ownership.
- Keep inputs, state changes, receipts, roots, checkpoints, replay material, and synchronization artifacts verifiable.
- Maintain contributor clarity by distinguishing implementation evidence from roadmap intent.

## Non-Responsibilities

- This subsystem narrative does not claim every named module is production-grade.
- It does not move authority into renderer, dashboard, observer, or convenience tooling paths.
- It does not replace targeted source review and tests before changing Rust behavior.

## Internal Components

- Source modules named in the subsystem chapters.
- Tests, fixtures, validation reports, and runbooks that support the architecture claim.
- Roadmap rows that identify implemented, partial, scaffold, and planned status.

### Canonical Data Flow

Input enters through a host, SDK, contract, peer, checkpoint, replay archive, or package boundary. Processing validates schema, version, signature, hash ancestry, and capability scope. State mutation occurs only through deterministic modules that own the relevant state. Receipt generation records roots, input hashes, output hashes, execution roots, state changes, snapshots, and protocol epoch metadata. Verification reruns execution or compares signed roots, receipt chains, checkpoint ancestry, and continuity records.

### Determinism and Validation

Hashing is the common language between modules. Canonical state roots prove the state that resulted from execution. Execution roots summarize the planned work. Receipt hashes summarize evidence. Checkpoint roots and snapshot hashes bridge recovery. Continuity roots and lineage records explain why later history descends from earlier history. Canonical serialization is mandatory before hashing. A module that serializes the same logical value differently on two machines cannot be authoritative.

Replay is the practical test of determinism. If a verifier receives the same prior state, input, protocol epoch, and contract package, it must recompute the same roots and receipts. If it cannot, EverArcade treats that as divergence, not as a renderer bug or UX inconsistency. Divergence is resolved by comparing roots, choosing valid lineage, restoring checkpoints, exchanging missing receipt ranges, and quarantining invalid peers or artifacts.

### Failure and Recovery Pattern

Corruption is detected by mismatched hashes, invalid signatures, malformed manifests, invalid proof material, or checkpoint ancestry gaps. Divergence is detected by root comparison and replay. Recovery chooses the smallest safe repair: import missing receipts, hydrate an archive, restore a checkpoint, roll back a window, resynchronize from a peer, or rebuild an index. Reconciliation must never invent authoritative state; it must select from verifiable material or recompute from canonical inputs.

## Future Evolution

Future work should turn partial and scaffold areas into validated runtime paths only when they have deterministic tests, operational runbooks, release gates, and clear owner modules.


# Contract Development

## Purpose

Contracts use ABI and package boundaries. A contract must treat input/output serialization, memory allocation, fuel/memory limits, state changes, and receipt effects as protocol behavior. Example contracts are onboarding material and compatibility tests.

## Responsibilities

- Preserve deterministic authority and explicit ownership.
- Keep inputs, state changes, receipts, roots, checkpoints, replay material, and synchronization artifacts verifiable.
- Maintain contributor clarity by distinguishing implementation evidence from roadmap intent.

## Non-Responsibilities

- This subsystem narrative does not claim every named module is production-grade.
- It does not move authority into renderer, dashboard, observer, or convenience tooling paths.
- It does not replace targeted source review and tests before changing Rust behavior.

## Internal Components

- Source modules named in the subsystem chapters.
- Tests, fixtures, validation reports, and runbooks that support the architecture claim.
- Roadmap rows that identify implemented, partial, scaffold, and planned status.

### Canonical Data Flow

Input enters through a host, SDK, contract, peer, checkpoint, replay archive, or package boundary. Processing validates schema, version, signature, hash ancestry, and capability scope. State mutation occurs only through deterministic modules that own the relevant state. Receipt generation records roots, input hashes, output hashes, execution roots, state changes, snapshots, and protocol epoch metadata. Verification reruns execution or compares signed roots, receipt chains, checkpoint ancestry, and continuity records.

### Determinism and Validation

Hashing is the common language between modules. Canonical state roots prove the state that resulted from execution. Execution roots summarize the planned work. Receipt hashes summarize evidence. Checkpoint roots and snapshot hashes bridge recovery. Continuity roots and lineage records explain why later history descends from earlier history. Canonical serialization is mandatory before hashing. A module that serializes the same logical value differently on two machines cannot be authoritative.

Replay is the practical test of determinism. If a verifier receives the same prior state, input, protocol epoch, and contract package, it must recompute the same roots and receipts. If it cannot, EverArcade treats that as divergence, not as a renderer bug or UX inconsistency. Divergence is resolved by comparing roots, choosing valid lineage, restoring checkpoints, exchanging missing receipt ranges, and quarantining invalid peers or artifacts.

### Failure and Recovery Pattern

Corruption is detected by mismatched hashes, invalid signatures, malformed manifests, invalid proof material, or checkpoint ancestry gaps. Divergence is detected by root comparison and replay. Recovery chooses the smallest safe repair: import missing receipts, hydrate an archive, restore a checkpoint, roll back a window, resynchronize from a peer, or rebuild an index. Reconciliation must never invent authoritative state; it must select from verifiable material or recompute from canonical inputs.

## Future Evolution

Future work should turn partial and scaffold areas into validated runtime paths only when they have deterministic tests, operational runbooks, release gates, and clear owner modules.


# Repository Inventory

## Purpose

The complete workspace crate inventory is maintained in `03-repository-map.md` and summarized here. The repository contains workspace crates for ABI, execution core, contract API, control plane, provider Evernode, rustrigs, contracts, host, SDKs, CLI, runtime clients, tools, content registry, and studio GUI. Non-crate trees include deployment reports, world reports, wasm reports, security reports, docs, runbooks, templates, registry entries, hooks, benchmarks, test vectors, and examples.

## Responsibilities

- Preserve deterministic authority and explicit ownership.
- Keep inputs, state changes, receipts, roots, checkpoints, replay material, and synchronization artifacts verifiable.
- Maintain contributor clarity by distinguishing implementation evidence from roadmap intent.

## Non-Responsibilities

- This subsystem narrative does not claim every named module is production-grade.
- It does not move authority into renderer, dashboard, observer, or convenience tooling paths.
- It does not replace targeted source review and tests before changing Rust behavior.

## Internal Components

- Source modules named in the subsystem chapters.
- Tests, fixtures, validation reports, and runbooks that support the architecture claim.
- Roadmap rows that identify implemented, partial, scaffold, and planned status.

### Canonical Data Flow

Input enters through a host, SDK, contract, peer, checkpoint, replay archive, or package boundary. Processing validates schema, version, signature, hash ancestry, and capability scope. State mutation occurs only through deterministic modules that own the relevant state. Receipt generation records roots, input hashes, output hashes, execution roots, state changes, snapshots, and protocol epoch metadata. Verification reruns execution or compares signed roots, receipt chains, checkpoint ancestry, and continuity records.

### Determinism and Validation

Hashing is the common language between modules. Canonical state roots prove the state that resulted from execution. Execution roots summarize the planned work. Receipt hashes summarize evidence. Checkpoint roots and snapshot hashes bridge recovery. Continuity roots and lineage records explain why later history descends from earlier history. Canonical serialization is mandatory before hashing. A module that serializes the same logical value differently on two machines cannot be authoritative.

Replay is the practical test of determinism. If a verifier receives the same prior state, input, protocol epoch, and contract package, it must recompute the same roots and receipts. If it cannot, EverArcade treats that as divergence, not as a renderer bug or UX inconsistency. Divergence is resolved by comparing roots, choosing valid lineage, restoring checkpoints, exchanging missing receipt ranges, and quarantining invalid peers or artifacts.

### Failure and Recovery Pattern

Corruption is detected by mismatched hashes, invalid signatures, malformed manifests, invalid proof material, or checkpoint ancestry gaps. Divergence is detected by root comparison and replay. Recovery chooses the smallest safe repair: import missing receipts, hydrate an archive, restore a checkpoint, roll back a window, resynchronize from a peer, or rebuild an index. Reconciliation must never invent authoritative state; it must select from verifiable material or recompute from canonical inputs.

## Future Evolution

Future work should turn partial and scaffold areas into validated runtime paths only when they have deterministic tests, operational runbooks, release gates, and clear owner modules.


# Implemented and Missing

## Purpose

Implemented foundations include execution core, WASM ABI, state engine, receipts, replay, checkpoints, lineage, and many test vectors. Partial systems include federation, synchronization, recovery, topology, world runtime, simulation, scheduler, economy, governance, partitioning, deployment, release certification, Evernode, and SDKs. Scaffold domains include renderer projection/history/federation and inventory user-facing surfaces. Planned work includes production persistence, signed registry, enforced gates, provider automation, and polished creator flows.

## Responsibilities

- Preserve deterministic authority and explicit ownership.
- Keep inputs, state changes, receipts, roots, checkpoints, replay material, and synchronization artifacts verifiable.
- Maintain contributor clarity by distinguishing implementation evidence from roadmap intent.

## Non-Responsibilities

- This subsystem narrative does not claim every named module is production-grade.
- It does not move authority into renderer, dashboard, observer, or convenience tooling paths.
- It does not replace targeted source review and tests before changing Rust behavior.

## Internal Components

- Source modules named in the subsystem chapters.
- Tests, fixtures, validation reports, and runbooks that support the architecture claim.
- Roadmap rows that identify implemented, partial, scaffold, and planned status.

### Canonical Data Flow

Input enters through a host, SDK, contract, peer, checkpoint, replay archive, or package boundary. Processing validates schema, version, signature, hash ancestry, and capability scope. State mutation occurs only through deterministic modules that own the relevant state. Receipt generation records roots, input hashes, output hashes, execution roots, state changes, snapshots, and protocol epoch metadata. Verification reruns execution or compares signed roots, receipt chains, checkpoint ancestry, and continuity records.

### Determinism and Validation

Hashing is the common language between modules. Canonical state roots prove the state that resulted from execution. Execution roots summarize the planned work. Receipt hashes summarize evidence. Checkpoint roots and snapshot hashes bridge recovery. Continuity roots and lineage records explain why later history descends from earlier history. Canonical serialization is mandatory before hashing. A module that serializes the same logical value differently on two machines cannot be authoritative.

Replay is the practical test of determinism. If a verifier receives the same prior state, input, protocol epoch, and contract package, it must recompute the same roots and receipts. If it cannot, EverArcade treats that as divergence, not as a renderer bug or UX inconsistency. Divergence is resolved by comparing roots, choosing valid lineage, restoring checkpoints, exchanging missing receipt ranges, and quarantining invalid peers or artifacts.

### Failure and Recovery Pattern

Corruption is detected by mismatched hashes, invalid signatures, malformed manifests, invalid proof material, or checkpoint ancestry gaps. Divergence is detected by root comparison and replay. Recovery chooses the smallest safe repair: import missing receipts, hydrate an archive, restore a checkpoint, roll back a window, resynchronize from a peer, or rebuild an index. Reconciliation must never invent authoritative state; it must select from verifiable material or recompute from canonical inputs.

## Future Evolution

Future work should turn partial and scaffold areas into validated runtime paths only when they have deterministic tests, operational runbooks, release gates, and clear owner modules.


# How to Contribute

## Purpose

Contributors should start by locating the owning subsystem document, checking the status matrix, reading the module tree, finding targeted tests, and preserving deterministic boundaries. Do not turn renderer state into authority. Do not add non-canonical serialization to hashed artifacts. Do not rely on wall-clock ordering for protocol decisions. Add or update docs when changing subsystem ownership or maturity.

## Responsibilities

- Preserve deterministic authority and explicit ownership.
- Keep inputs, state changes, receipts, roots, checkpoints, replay material, and synchronization artifacts verifiable.
- Maintain contributor clarity by distinguishing implementation evidence from roadmap intent.

## Non-Responsibilities

- This subsystem narrative does not claim every named module is production-grade.
- It does not move authority into renderer, dashboard, observer, or convenience tooling paths.
- It does not replace targeted source review and tests before changing Rust behavior.

## Internal Components

- Source modules named in the subsystem chapters.
- Tests, fixtures, validation reports, and runbooks that support the architecture claim.
- Roadmap rows that identify implemented, partial, scaffold, and planned status.

### Canonical Data Flow

Input enters through a host, SDK, contract, peer, checkpoint, replay archive, or package boundary. Processing validates schema, version, signature, hash ancestry, and capability scope. State mutation occurs only through deterministic modules that own the relevant state. Receipt generation records roots, input hashes, output hashes, execution roots, state changes, snapshots, and protocol epoch metadata. Verification reruns execution or compares signed roots, receipt chains, checkpoint ancestry, and continuity records.

### Determinism and Validation

Hashing is the common language between modules. Canonical state roots prove the state that resulted from execution. Execution roots summarize the planned work. Receipt hashes summarize evidence. Checkpoint roots and snapshot hashes bridge recovery. Continuity roots and lineage records explain why later history descends from earlier history. Canonical serialization is mandatory before hashing. A module that serializes the same logical value differently on two machines cannot be authoritative.

Replay is the practical test of determinism. If a verifier receives the same prior state, input, protocol epoch, and contract package, it must recompute the same roots and receipts. If it cannot, EverArcade treats that as divergence, not as a renderer bug or UX inconsistency. Divergence is resolved by comparing roots, choosing valid lineage, restoring checkpoints, exchanging missing receipt ranges, and quarantining invalid peers or artifacts.

### Failure and Recovery Pattern

Corruption is detected by mismatched hashes, invalid signatures, malformed manifests, invalid proof material, or checkpoint ancestry gaps. Divergence is detected by root comparison and replay. Recovery chooses the smallest safe repair: import missing receipts, hydrate an archive, restore a checkpoint, roll back a window, resynchronize from a peer, or rebuild an index. Reconciliation must never invent authoritative state; it must select from verifiable material or recompute from canonical inputs.

## Future Evolution

Future work should turn partial and scaffold areas into validated runtime paths only when they have deterministic tests, operational runbooks, release gates, and clear owner modules.

# Consolidated Engineering Notes

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.

EverArcade architecture should be read from evidence to abstraction: source modules define behavior, tests and vectors prove expected behavior, reports describe validation campaigns, and this book normalizes the conclusions. Contributors must downgrade claims that cannot be tied to current code or tests. Determinism is not a slogan; it is the requirement that independent machines can compute the same roots from the same prior state, inputs, protocol epoch, and package. Federation, deployment, rendering, SDKs, and operator workflows are valuable only when they preserve that property.
