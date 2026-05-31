# Two-Node Certification Report

## evidence
- Latest script run executed `cargo test -p execution-core --test two_node_certification_tests`.
- Harness covers deterministic Node A / Node B join, tick, replay, checkpoint, failure, recovery, divergence, corruption, authority, partition, and Arena Vanguard scenarios.

## test coverage
- `test_node_join_certification`
- `test_world_convergence`
- `test_replay_convergence`
- `test_checkpoint_convergence`
- `test_node_failure_survival`
- `test_node_recovery_convergence`
- `test_divergence_detection`
- `test_corrupt_checkpoint_rejection`
- `test_corrupt_replay_rejection`
- `test_network_partition_detection`
- `test_authority_preservation`
- `test_arena_vanguard_two_node_runtime`

## known limitations
- In-process deterministic certification harness only; no WAN, QUIC, EverNode, or XRPL live submission.
- Federation transport and renderer/history domains remain scaffold-level runtime domains.

## remaining scaffolds
- Public federation membership.
- External checkpoint transport.
- Live operator orchestration.

## next risks
- Real network timing and operator storage must be validated before deployment readiness can be claimed.
