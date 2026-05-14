pub type Hash = [u8; 32];
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MessageCommit {
    pub root: Hash,
}
