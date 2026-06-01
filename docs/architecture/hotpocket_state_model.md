# HotPocket POSIX State Model

## Purpose

This document defines the canonical EverArcade `state/` directory used by a HotPocket contract process. The directory is the POSIX authority boundary: every validator must be able to reconstruct the same authoritative roots from the same inputs, replay records, rustrig package hashes, and checkpoint lineage.

## Canonical layout

```text
state/
  world/
  replay/
  checkpoints/
  receipts/
  rustrigs/
  packages/
  anchors/
  operator/
```

| Path | Classification | Contents | Regeneration and anchoring rule |
| --- | --- | --- | --- |
| `state/world/` | Consensus state | Canonical world snapshots, deterministic entity/component data, active runtime cursors, and the current world root. | The latest world can be rebuilt from `state/replay/` plus the latest valid `state/checkpoints/`; the active world root is authority and may be externally anchored. |
| `state/replay/` | Consensus state | Append-only records emitted by rustrigs and accepted runtime envelopes, ordered by deterministic tick/sequence. | Primary source for history. It must be sufficient to regenerate world state after the latest checkpoint and to audit receipts. Replay roots are externally anchorable. |
| `state/checkpoints/` | Consensus state | Recovery checkpoints, checkpoint manifests, lineage roots, and restore cursors. | Checkpoints are recovery accelerators, but each checkpoint must be verifiable against replay lineage. Checkpoint roots are externally anchorable. |
| `state/receipts/` | Consensus state | Execution receipts, package verification receipts, runtime acceptance receipts, and settlement intent receipts. | Receipts can be recomputed only when their source inputs and canonical serialization are retained; receipt roots are externally anchorable. |
| `state/rustrigs/` | Consensus state for selected package metadata; immutable code payload cache for selected rustrigs | Rustrig identifiers, versions, hashes, deterministic ABI metadata, and approved package payloads or references. | Package bytes may be restored from `state/packages/` or an external content store, but the selected hash/version set is authority. Never write generated build artifacts here. |
| `state/packages/` | Consensus state for accepted package manifests; optional package payload cache | Game package manifests, package hashes, content-addressed bundles, and reproducible-build metadata accepted by consensus. | Package manifests and accepted hashes are authority; bulky package payloads may be rehydrated from release storage when hashes match. Package/deployment roots are externally anchorable. |
| `state/anchors/` | Consensus state for outbound intent records and inbound verified settlement receipts | `XRPLIntentRecord`, `ReceiptAnchorRecord`, `ReplayAnchorRecord`, `CheckpointAnchorRecord`, `VaultIntentRecord`, observed transaction ids, and verification status. | External chains are not authority for gameplay; they notarize selected roots. Anchor intents can be replayed by an external settlement service. |
| `state/operator/` | Consensus state only for operator decisions that affect runtime authority | Lease assignment records, deployment manifests, recovery decisions, upgrade approvals, and health-gate outcomes accepted by consensus. | Operator dashboards and log indexes are derived caches; accepted operator decisions are replayable records and may be restored from replay/checkpoints. |

## Consensus state

Consensus state is data that changes the authoritative EverArcade outcome if it is missing or modified. It includes:

- the accepted world root and canonical world data under `state/world/`;
- append-only replay records under `state/replay/`;
- checkpoint manifests and lineage roots under `state/checkpoints/`;
- execution, package, deployment, and settlement-intent receipts under `state/receipts/`;
- selected rustrig ids, versions, ABI metadata, and content hashes under `state/rustrigs/`;
- accepted package manifests and hashes under `state/packages/`;
- outbound anchor intent records and verified external anchor receipts under `state/anchors/`;
- operator decisions that alter deployment, recovery, lease, or upgrade authority under `state/operator/`.

## Derived cache

Derived cache is data that improves runtime speed or UX but must not be the only copy of authority. It may live outside `state/` or under a clearly marked non-authoritative cache root managed by the host, not by HotPocket consensus. Examples include:

- renderer frames, GPU buffers, textures, meshes, thumbnails, and viewport state;
- replay indexes, search indexes, timeline summaries, and analytics rollups;
- decompressed package payloads when the package hash is already in authority;
- local build outputs, Cargo targets, WASM optimization outputs, and generated SDK artifacts;
- Studio UI workspace layout, selection state, and inspector caches;
- Prometheus scrape buffers, exported logs, alert delivery state, and dashboard materializations.

## Data that must never be written inside authority

The HotPocket authority path must never contain nondeterministic or host-local material:

- private XRPL/Xahau keys, wallet seeds, HSM handles, or signing credentials;
- live XRPL/Xahau submission queues whose ordering depends on wall-clock/network conditions;
- GPU driver output, render frames, shader caches, neural upscaling outputs, or projection-only service state;
- wall-clock timestamps used as gameplay input unless already canonicalized as a consensus record;
- host process ids, socket descriptors, ports allocated by the OS, temp files, PID files, or systemd state;
- logs that include local timing, environment paths, secrets, or machine-only diagnostics;
- mutable package build directories, dependency caches, or compiler outputs;
- speculative operator UI edits that have not become accepted operator records.

## Regeneration from replay

A fresh validator or recovering node must be able to:

1. verify package and rustrig hashes from `state/packages/` and `state/rustrigs/`;
2. select the latest valid checkpoint from `state/checkpoints/`;
3. replay records from `state/replay/` after the checkpoint cursor;
4. reproduce `state/world/` roots and deterministic receipts;
5. compare generated receipt/replay/checkpoint roots with records in `state/anchors/`;
6. rebuild derived indexes, projections, Studio views, and monitoring materializations outside authority.

If a derived cache cannot be regenerated from replay, checkpoints, packages, rustrig hashes, and accepted operator records, then it is not a cache and must be promoted to an explicit consensus record or removed.

## External anchoring

External anchors notarize roots; they do not replace HotPocket authority. EverArcade may emit anchor intents for:

- replay window roots;
- checkpoint roots and lineage roots;
- execution receipt roots;
- deployment/package roots;
- vault and settlement intents.

The external settlement service performs signing, submission, retry, and transaction confirmation. HotPocket stores only deterministic intents and verified receipt facts under `state/anchors/`.
