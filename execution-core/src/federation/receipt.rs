#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FederationReceipt {
    pub federation_transition_hash: String,
    pub governance_root: String,
    pub treaty_root: String,
    pub quorum_result_root: String,
    pub constitutional_root: String,
    pub execution_result_root: String,
    pub replay_proof_root: String,
}
