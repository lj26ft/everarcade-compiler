# Warning Cleanup Report

## Replay/runtime warning status
- historical replay and transport runtime tests compile cleanly with intentional scaffold warnings only.
- stale `history` wildcard re-export warning sources were removed from test-facing export paths.

## Remaining intentional warnings
- scaffold runtime modules may still emit dead-code warnings where future deterministic layers are intentionally unintegrated.

## Namespace lineage summary
- explicit imports are now used for replay fabric integration tests.
- replay runtime export integrity covered by dedicated namespace continuity tests.
