use crate::federation::{identity::OperatorIdentity, trust::TrustDomain};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PeerDescriptor {
    pub identity: OperatorIdentity,
    pub trust_domain: TrustDomain,
    pub capabilities: Vec<String>,
}

impl PeerDescriptor {
    pub fn canonicalized(mut self) -> Self {
        self.capabilities.sort();
        self.capabilities.dedup();
        self
    }
}
