> **Repository boundary:** Marketplace systems are outside the scope of this repository.
>
> This repo may include deterministic package or capability metadata examples, but commercial marketplace operation, royalties, settlement, reputation, provider rewards, and hosted marketplace services are not implemented here.
>
# GPU Marketplace v0.1

## Purpose

GPU Marketplace v0.1 is the first deterministic economic network for
EverArcade projection compute. It lets non-authoritative GPU providers register
capacity, receive projection assignments, submit render artifacts, pass or fail
verification, and accumulate settlement evidence.

The marketplace extends GPU Runtime. It does not replace lease authority,
simulation authority, economy authority, governance authority, inventory
authority, or civilization state authority.

```text
Lease Runtime -> Projection Job -> GPU Marketplace -> GPU Provider
-> Render Artifact -> Verification -> Settlement Evidence
```

## Provider Identity

A provider identity contains:

- Provider ID.
- Node ID.
- Registration epoch.
- Capability root.
- Identity root.

The provider identity root is derived from a canonical transcript containing the
marketplace context and each provider's identity fields. It is deterministic and
replay-safe.

## Registration

The registration layer represents the provider lifecycle:

- Register.
- Update.
- Suspend.
- Recover.
- Retire.

Registration events are ordered by canonical epoch and lexicographic provider
data. The registration root commits to all lifecycle events plus the provider
identity root. A replay must reproduce the same root from the same event set.

## Capability Model

Capability advertisement is declarative. v0.1 performs no live hardware
discovery.

Each provider advertises:

- GPU model.
- Memory.
- Queue capacity.
- Render classes.
- Availability.

The capability root is generated from deterministic provider capability records.
Providers may advertise projection compute only.

## Capacity Model

Capacity declarations represent schedulable marketplace slots:

- Available slots.
- Reserved slots.
- Consumed slots.
- Epoch capacity.

The capacity root commits to declared capacity and the capability root. Capacity
is evidence for assignment; it is not authoritative world state.

## Assignment Model

The assignment layer maps jobs to providers:

- Job.
- Provider.
- Assignment.
- Assignment epoch.

Assignments use canonical ordering by epoch, priority, job ID, provider ID, and
lexicographic tie breakers. The assignment root commits to the registration and
capacity roots so replay can reproduce the same provider-job mapping.

## Artifact Model

The artifact submission layer represents provider output:

- Provider.
- Job.
- Artifact.
- Submission epoch.
- Payload hash.

The artifact submission root commits to submitted render artifacts and the
assignment root. Providers earn no credit merely by claiming work.

## Verification Model

Verification checks:

- Artifact integrity.
- Job match.
- Provider match.
- Projection match.

The verification root is generated from verification evidence. A failed
artifact can be included in the transcript while producing zero settlement
reward.

## Settlement Intent Model

Settlement intent records:

- Provider.
- Work completed.
- Verification result.
- Reward units.

v0.1 does not perform payments and does not submit XRPL transactions. It only
produces deterministic settlement-intent evidence for future settlement layers.

## Reputation Model

Provider reputation records:

- Successful jobs.
- Failed jobs.
- Verified artifacts.
- Provider score.

The reputation root commits to provider performance evidence. Reputation is
marketplace evidence, not authoritative game state.

## Replay Model

Marketplace replay consumes:

- Assignments.
- Artifacts.
- Verification.
- Settlement intents.
- Reputation.

Replay regenerates the marketplace replay root. Certification requires:

```text
Marketplace Replay Root == Marketplace Root
```

## Lease Integration

Lease integration represents:

- Projection export.
- Marketplace submission.
- Artifact import.
- Verification import.

The lease remains scheduler and state authority. The GPU provider remains a
worker that consumes projection jobs and returns render artifacts. Marketplace
verification evidence can be imported by the lease without transferring
authority.

## PASS Criteria

GPU Marketplace v0.1 passes when validation reports PASS for:

- Identity.
- Registration.
- Capability.
- Capacity.
- Assignment.
- Artifacts.
- Verification.
- Settlement Intent.
- Reputation.
- Replay.
- Lease Integration.

Certification must also report:

```text
GPU Marketplace: PASS
```

## FAIL Criteria

The marketplace fails if any of the following are true:

- Provider identity root cannot be regenerated.
- Registration lifecycle events are missing or non-deterministic.
- Capability advertisement depends on live discovery.
- Capacity declarations are missing slot accounting.
- Assignment ordering is not canonical.
- Artifact submission lacks provider, job, artifact, or epoch evidence.
- Verification omits integrity, job, provider, or projection matching.
- Settlement intent attempts real payment submission.
- Reputation cannot be derived from verification evidence.
- Replay root differs from marketplace root.
- Lease authority is transferred to a GPU provider.

## Relationship To GPU Runtime

GPU Runtime models non-authoritative projection jobs, workers, devices, queues,
artifacts, verification, replay rendering, and renderer integration. GPU
Marketplace v0.1 consumes the GPU Runtime job and projection roots, then adds the
economic network layer: provider identity, registration, assignments,
settlement-intent evidence, reputation, and replayable marketplace roots.

GPU Runtime remains the projection compute runtime. GPU Marketplace is the
evidence and assignment network around that runtime.

## Relationship To Future XRPL Settlement

v0.1 intentionally stops at settlement intent. Reward units and verification
results form deterministic evidence that a future XRPL settlement layer can
consume. No XRPL submission, token transfer, or real payment happens in this
version.
