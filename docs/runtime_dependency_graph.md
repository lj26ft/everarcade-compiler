# Runtime Dependency Graph

## Replay/Runtime Boundaries
- replay/history runtime -> archive/provenance/query/timeline modules
- replay transport runtime -> chunk/stream/recovery/equivalence modules
- federation runtime -> node/session/transport/verification modules
- validation runtime -> dag/checkpoint/recovery/report/stress modules
- CI orchestration runtime -> pipeline/scheduler/release/signing/report modules

Renderer remains non-authoritative; replay remains reconstruction-only.
