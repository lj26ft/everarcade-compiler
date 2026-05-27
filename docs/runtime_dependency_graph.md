# Runtime Dependency Graph

## Replay/Runtime Dependency Lineage

- History replay runtime depends on timeline, anchor, compression, and corruption-verification surfaces.
- Replay transport runtime depends on stream/chunk/window modules and continuity validation.
- Federation runtime depends on transport, verification, and continuity surfaces.
- Validation runtime depends on dag/stage/checkpoint/runtime modules.
- CI runtime depends on execution/release/report/runtime modules.

## Canonical Public API Rules

1. Canonical imports use direct module ownership paths.
2. Indirect alias namespace exports are prohibited.
3. Integration tests must use explicit symbol imports.
