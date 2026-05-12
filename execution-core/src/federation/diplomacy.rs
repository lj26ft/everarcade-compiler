use crate::hashing::hash_bytes;

pub fn negotiation_commitment(proposal: &str, counterparty: &str) -> String {
    hash_bytes(format!("{proposal}:{counterparty}").as_bytes())
}
