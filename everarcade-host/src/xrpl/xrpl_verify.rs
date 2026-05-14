pub fn verify_anchor_submission(tx_hash: &str) -> bool {
    !tx_hash.trim().is_empty()
}
