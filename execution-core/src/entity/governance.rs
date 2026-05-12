use crate::hashing::hash_bytes;

pub fn governance_decision_root(epoch: u64, proposal_id: &str, decision: &str) -> String {
    hash_bytes(format!("gov:{epoch}:{proposal_id}:{decision}").as_bytes())
}
