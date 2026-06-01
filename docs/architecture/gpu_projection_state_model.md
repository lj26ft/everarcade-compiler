# GPU Projection State Model

## Runtime rule

```text
GPU output is derived projection.
Replay is source.
State folder is authority.
XRPL anchors verify receipts/checkpoints.
```

## Authority model

The CPU/runtime authority applies deterministic records and writes canonical state under the HotPocket `state/` directory. GPU services, renderers, viewport previews, neural upscalers, video encoders, and projection streamers are readers. They may consume replay windows, checkpoints, packages, and assets, but their outputs do not become gameplay authority.

## Projection inputs

A projection service may read:

- replay records and replay roots;
- checkpoint snapshots and checkpoint lineage roots;
- world state snapshots exposed by the authority runtime;
- package/rustrig manifests and content hashes;
- verified receipt/checkpoint anchors;
- non-authoritative local renderer settings.

## Projection outputs

Projection outputs include:

- frames, frame envelopes, video streams, thumbnails, and screenshots;
- GPU buffers, shader caches, render graphs, texture atlases, and mesh caches;
- visual replay timelines and observer stream indexes;
- projection validation reports and frame hashes.

These outputs are derived cache. They must be stored outside the HotPocket authority path unless represented as explicit validation receipts that do not change gameplay state.

## Verification

A projection can be trusted for display only when it identifies:

- source replay window and replay root;
- checkpoint id/root used for fast start;
- package and rustrig hashes;
- renderer/projection version;
- optional frame/projection hash.

XRPL/Xahau anchors can verify receipt and checkpoint roots, but they do not make GPU output authoritative. If a rendered frame conflicts with replay-derived state, replay wins.

## Recovery behavior

After crash, migration, or fresh VM restore:

1. restore or verify HotPocket `state/` authority;
2. select checkpoint and replay cursor;
3. regenerate world state;
4. restart projection service;
5. rebuild GPU caches and frame indexes from replay/checkpoints;
6. discard any projection cache that cannot be tied to the current replay/checkpoint roots.

## Launch posture

Renderer/history/federation projection domains should remain scaffold-level for launch unless they are read-only consumers with explicit source roots and no write path into authority.
