pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FederationScheduler {
    pub root: Hash,
}
