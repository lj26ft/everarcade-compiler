# Evernode Sizing Notes (Early)

These are early estimates and should be refined with production traces.

- **CPU expectations:** prioritize sustained single-node deterministic execution; profile by fuel/call counts first.
- **Memory ceilings:** bounded by WASM memory pages and replay window retention.
- **Storage growth:** receipts, state diffs, and replay archives are primary drivers.
- **Replay retention cost:** increases linearly with retained replay windows.
- **Archive cost:** depends on checkpoint cadence and compression settings.
- **Stdout log volume:** JSONL diagnostics can grow quickly under full profiling mode.
- **Profiling overhead:** expected low for count-based metrics; higher when host timing is enabled.
- **Unknowns:** workload skew, federation-scale contention, long-horizon archive compaction behavior.
