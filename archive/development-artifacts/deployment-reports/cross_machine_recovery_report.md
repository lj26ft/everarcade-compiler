# Cross-Machine Recovery Report

## Classification

Partially Implemented. The harness exercises deterministic cross-machine semantics through isolated runtime/storage roots, separate runtime identities, and actual TCP payload transfer inside the certification process. It does not claim that this repository has completed a physical two-host data-center run.

## Recovery Evidence

| Evidence | Status | Notes |
| --- | --- | --- |
| Recovery time | Implemented | `CrossMachineSession::recover_machine_a` records elapsed recovery time in milliseconds. |
| Checkpoint transfer count | Implemented | TCP checkpoint payloads increment `checkpoint_transfer_count`. |
| Replay transfer count | Implemented | TCP replay payloads and recovery synchronization increment `replay_transfer_count`. |
| Convergence evidence | Implemented | Recovery requires matching world, replay, checkpoint, and continuity roots before success is reported. |

## Current Certification Result

Machine A boots Arena Vanguard, Machine B joins from checkpoint state, Machine A can fail, Machine B can continue authoritatively, and Machine A can restore from Machine B. The proof uses actual TCP frames for validation payloads and isolated filesystem roots for each machine.

## Limitations

- Physical host isolation is not automatically provisioned by the test suite.
- Packet loss and latency are represented as deterministic interruption/resume events rather than kernel-level network shaping.
- Partition reconciliation is detection-first; automatic merge of divergent authoritative histories remains out of scope.
