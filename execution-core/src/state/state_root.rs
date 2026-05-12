pub type Hash = [u8; 32];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StateRoot {
    pub prior_root: Hash,
    pub next_root: Hash,
    pub transition_root: Hash,
}
