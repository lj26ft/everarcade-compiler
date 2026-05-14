use crate::hashing::hash_bytes;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Resolution {
    pub resolution_id: String,
    pub proposal_id: String,
    pub approved: bool,
}

impl Resolution {
    pub fn finalize(proposal_id: impl Into<String>, approved: bool) -> Self {
        let proposal_id = proposal_id.into();
        let resolution_id = hash_bytes(format!("resolution:{proposal_id}:{approved}").as_bytes());
        Self {
            resolution_id,
            proposal_id,
            approved,
        }
    }
}
