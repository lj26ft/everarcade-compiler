# Evernode Capacity Baseline

## Measured
- DAG and replay benchmark harness emits deterministic operation/receipt/state-diff counts.
- Replay benchmark captures archive growth progression from 100 to 100,000 operations.
- WASM benchmark captures call/serialization/memory page metrics across payload classes.

## Inferred
- CPU: operators should reserve headroom for replay verification bursts.
- RAM: memory pressure scales with state diff and replay reconstruction depth.
- Storage: archive retention grows approximately linearly with operation count until compression policy changes.

## Speculative
- 10,000+ DAG nodes may expose traversal hotspots depending on topology fan-out.
- 100,000 replay operations may require checkpoint compaction policy tuning.
- Lock contention risk increases in shared CI hosts with high test-thread counts.
