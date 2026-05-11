use std::collections::BTreeMap;

use super::voting::VerifierVote;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiptConsensusResult {
    pub canonical_receipt_hash: String,
    pub agreeing_verifiers: Vec<String>,
}

pub fn resolve_receipt_consensus(votes: &[VerifierVote]) -> Option<ReceiptConsensusResult> {
    let mut buckets: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for vote in votes {
        buckets
            .entry(vote.receipt_hash.clone())
            .or_default()
            .push(vote.verifier_id.clone());
    }

    buckets
        .into_iter()
        .max_by_key(|(_, verifiers)| verifiers.len())
        .map(|(canonical_receipt_hash, agreeing_verifiers)| ReceiptConsensusResult {
            canonical_receipt_hash,
            agreeing_verifiers,
        })
}
