# Runtime Scaffold Inventory

This document tracks intentional scaffold runtime surfaces that are explicitly non-authoritative.

- `runtime/renderer-client/src/history`: replay/history facade scaffolding, replay-derived only.
- `runtime/renderer-client/src/federation`: future renderer federation integration hooks.
- `runtime/renderer-client/src/transport_runtime`: renderer-side transport runtime protocol scaffolding.

## Guarantees
- Renderer remains non-authoritative.
- Replay lineage and continuity semantics remain execution-core authoritative.
- Scaffolds may carry `#![allow(dead_code)]` until integration closure.
