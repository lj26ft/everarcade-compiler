pub type Hash = [u8; 32];
pub fn resolve_quorum(vote_root: Hash, quorum_threshold: u8) -> Result<Hash, &'static str> {
    if quorum_threshold == 0 {
        return Err("invalid quorum threshold");
    }
    Ok(std::array::from_fn(|i| {
        vote_root[i].wrapping_add(quorum_threshold)
    }))
}
