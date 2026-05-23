use crate::{canonical::encoding::canonical_encode, hashing::hash_bytes};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuorumRule {
    pub numerator: u64,
    pub denominator: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rational {
    pub numerator: u64,
    pub denominator: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuorumResult {
    pub quorum_reached: bool,
    pub approval_ratio: Rational,
    pub canonical_voters: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FederationQuorumProof {
    pub receipt_hash: String,
    pub state_root: String,
    pub replay_root: String,
    pub participating_nodes: Vec<String>,
    pub quorum_hash: String,
}

impl QuorumRule {
    pub fn reached(&self, votes: u64, members: u64) -> bool {
        if members == 0 || self.denominator == 0 {
            return false;
        }
        votes.saturating_mul(self.denominator) >= members.saturating_mul(self.numerator)
    }

    pub fn resolve(
        &self,
        approvals: &[String],
        abstentions: &[String],
        members: &[String],
    ) -> QuorumResult {
        let mut canonical_voters = approvals.to_vec();
        canonical_voters.extend_from_slice(abstentions);
        canonical_voters.sort();
        canonical_voters.dedup();

        let approval_count = approvals
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len() as u64;
        let member_count = members
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len() as u64;

        QuorumResult {
            quorum_reached: self.reached(approval_count, member_count),
            approval_ratio: Rational {
                numerator: approval_count,
                denominator: member_count.max(1),
            },
            canonical_voters,
        }
    }
}

impl FederationQuorumProof {
    pub fn new(
        receipt_hash: String,
        state_root: String,
        replay_root: String,
        mut participating_nodes: Vec<String>,
    ) -> Self {
        participating_nodes.sort();
        participating_nodes.dedup();
        let quorum_hash = hash_bytes(
            format!(
                "{receipt_hash}|{state_root}|{replay_root}|{}",
                participating_nodes.join(",")
            )
            .as_bytes(),
        );
        Self {
            receipt_hash,
            state_root,
            replay_root,
            participating_nodes,
            quorum_hash,
        }
    }

    pub fn canonical_hash(&self) -> Result<String, String> {
        Ok(hash_bytes(
            &canonical_encode(self).map_err(|e| e.to_string())?,
        ))
    }
}
