pub fn continuity_score(archival_replicas: u64, replay_nodes: u64, proof_backends: u64, epoch_depth: u64) -> u64 {
    archival_replicas
        .saturating_mul(10)
        .saturating_add(replay_nodes.saturating_mul(20))
        .saturating_add(proof_backends.saturating_mul(30))
        .saturating_add(epoch_depth)
}
