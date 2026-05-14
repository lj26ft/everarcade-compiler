use crate::hashing::hash_bytes;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Proposal {
    pub proposal_id: String,
    pub proposer: String,
    pub body_hash: String,
}

impl Proposal {
    pub fn new(proposer: impl Into<String>, body: &[u8]) -> Self {
        let proposer = proposer.into();
        let body_hash = hash_bytes(body);
        let proposal_id = hash_bytes(format!("proposal:{proposer}:{body_hash}").as_bytes());
        Self {
            proposal_id,
            proposer,
            body_hash,
        }
    }
}
