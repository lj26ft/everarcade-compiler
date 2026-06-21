# GPU Runtime v0.1

## Purpose

GPU Runtime v0.1 is the first operational, deterministic GPU acceleration layer for the EverArcade Renderer Runtime. It consumes canonical projection artifacts and replay inputs, produces render artifacts and render roots, and never mutates protocol state.

Authoritative flow remains:

```text
Civilization Runtime -> Projection Root
```

GPU flow is consumer-only:

```text
Renderer Runtime -> GPU Runtime -> Rendered Output
```

The GPU Runtime may read projections, replay streams, and checkpoints. It may produce render artifacts. It may not modify state, inventory, economy, governance, settlement, authority, or marketplace data.

## GPU Job Model

A GPU job contains:

- Job ID.
- Projection Root.
- Job Type.
- Priority.
- Submission Epoch.
- Job Root.

Supported job types are:

- World Render.
- Entity Render.
- Physics Visualization.
- Inventory Visualization.
- Event Visualization.
- Replay Render.

`gpu/jobs/gpu_model.sh` derives a deterministic GPU Job Root from canonical job records, renderer projection root, runtime identity, replay-safety flags, and canonical ordering metadata.

## Worker Model

A GPU worker contains:

- Worker ID.
- Device ID.
- Capability Profile.
- Capacity.
- Availability.

Workers are runtime descriptors only. There is no marketplace registration, settlement, or payment in v0.1. The Worker Root commits to deterministic worker records and the Device Root.

## Device Model

A GPU device capability contains:

- GPU Identifier.
- Memory.
- Compute Capability.
- Queue Capacity.
- Runtime Version.

The Device Root is derived from static capability records. v0.1 does not inspect live CUDA, Vulkan, OpenCL, drivers, PCI devices, or host hardware.

## Queue Model

The queue model represents:

- Pending.
- Assigned.
- Running.
- Completed.
- Failed.

Queue ordering is deterministic and replay-safe. Assignment is canonical because queue transcripts include fixed job IDs, worker IDs, queue states, Job Root, and Worker Root. GPU failure is isolated by recording failed work as render-domain output only.

## Artifact Model

A render artifact contains:

- Projection Root.
- Worker Root.
- Job Root.
- Artifact Root.

The Render Root is derived from the artifact transcript and Artifact Root. Render artifacts are deterministic, replay-safe, and verifiable projection outputs. They are not authoritative state transitions.

## Verification Model

The verification layer validates:

- Projection Root Match.
- Job Root Match.
- Worker Match.
- Artifact Integrity.

It generates a Verification Root. Verification fails if any required match is false or any root is missing.

## Replay Model

The replay layer consumes:

- Projection.
- Job.
- Worker.
- Artifact.

It regenerates a Replay Render Root and verifies:

```text
Replay Render Root == Render Root
```

This preserves the requirement that checkpoint plus replay stream regenerates identical render output roots.

## Renderer Integration

Renderer integration represents:

- Projection Export.
- GPU Job Submission.
- Artifact Import.

The Integration Root commits to the renderer projection export root, GPU job submission root, artifact import root, and render root. The renderer remains authoritative over projection; the GPU Runtime remains a consumer-only acceleration layer.

## PASS Criteria

GPU Runtime v0.1 passes when:

- Jobs validate and include all supported job types.
- Workers validate and expose deterministic availability.
- Devices validate without live hardware inspection.
- Queues validate all required states.
- Artifacts produce Artifact Root and Render Root.
- Verification validates projection, job, worker, and artifact integrity.
- Replay Render Root equals Render Root.
- Renderer Integration validates projection export, job submission, and artifact import.
- Validation report ends with `GPU Runtime Validation: PASS`.
- Certification report ends with `GPU Runtime: PASS`.

## FAIL Criteria

GPU Runtime v0.1 fails if:

- Any root is missing or non-deterministic.
- Replay Render Root differs from Render Root.
- Live hardware inspection is required.
- GPU output attempts to mutate protocol state.
- Queue states are incomplete.
- Renderer integration makes GPU authoritative over projection.
- Validation or certification reports any domain as `FAIL`.

## Relationship To Renderer Runtime

The Renderer Runtime produces canonical projection roots from civilization/world projection artifacts. GPU Runtime v0.1 consumes those projection roots and returns render artifacts. It accelerates renderer workloads without changing renderer authority, projection semantics, checkpoint continuity, replay streams, or protocol sovereignty.

## Relationship To Future GPU Marketplace

v0.1 intentionally has no marketplace. It models workers, devices, capacity, and availability only as runtime descriptors. A future GPU Marketplace v0.1 can build on these descriptors to add provider registration, job acceptance, artifact delivery, and settlement while keeping GPU compute non-authoritative.
