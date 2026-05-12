#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FederationFork {
    pub parent_federation_id: String,
    pub child_federation_id: String,
    pub fork_reason_hash: String,
}
