use crate::federation::node::FederationNodeId;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct OperatorIdentity {
    pub node_id: FederationNodeId,
    pub operator_name: String,
    pub namespace: String,
    pub public_key_hint: Vec<u8>,
}

impl OperatorIdentity {
    pub fn canonicalized(mut self) -> Self {
        self.operator_name = self.operator_name.trim().to_ascii_lowercase();
        self.namespace = self.namespace.trim().to_ascii_lowercase();
        self
    }
}
