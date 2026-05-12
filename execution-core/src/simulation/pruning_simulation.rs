pub fn pruning_replay_safe(has_checkpoint: bool, has_summary: bool, has_proof_commitment: bool) -> bool {
    has_checkpoint && has_summary && has_proof_commitment
}
