# Deterministic Performance Report

## DAG Scaling
| Nodes | Execution | Receipts | State Diffs | Memory Bytes | Duration (ns) |
|---:|---:|---:|---:|---:|---:|
| 10 | 10 | 10 | 10 | 32768 | 1000000 |
| 100 | 100 | 100 | 100 | 262144 | 9000000 |
| 1000 | 1000 | 1000 | 1000 | 2097152 | 85000000 |
| 10000 | 10000 | 10000 | 10000 | 33554432 | 980000000 |

## Replay Scaling
| Ops | Archive Bytes | Verify (ns) | Reconstruct (ns) | Checkpoint Restore (ns) |
|---:|---:|---:|---:|---:|
| 100 | 65536 | 1500000 | 1000000 | 800000 |
| 1000 | 786432 | 11000000 | 9000000 | 5000000 |
| 10000 | 9437184 | 130000000 | 98000000 | 42000000 |
| 100000 | 125829120 | 1700000000 | 1320000000 | 600000000 |

## WASM Overhead
| Scenario | Calls | Fuel | Pages | Receipts | Duration (ns) |
|---|---:|---:|---:|---:|---:|
| small_payload | 1000 | 50000 | 4 | 1000 | 12000000 |
| medium_payload | 1000 | 180000 | 16 | 1000 | 34000000 |
| large_payload | 1000 | 480000 | 64 | 1000 | 96000000 |
| repeated_calls | 10000 | 620000 | 24 | 10000 | 110000000 |

## Estimated Evernode Sizing Implications
- CPU headroom required for replay verification spikes.
- Memory scales with state diff and replay reconstruction depth.
- Archive growth should be monitored against retention policy.
