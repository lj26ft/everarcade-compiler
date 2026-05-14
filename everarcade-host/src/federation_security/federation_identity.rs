pub type Hash = [u8; 32];
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FederationIdentity {
    pub node_root: Hash,
    pub treaty_scope_root: Hash,
}
