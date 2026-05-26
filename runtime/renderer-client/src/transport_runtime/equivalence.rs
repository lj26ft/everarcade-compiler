use super::stream::ReplayTransportStream;

#[derive(Debug, Clone, Default)]
pub struct ReplayEquivalenceRuntime;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReplayEquivalenceProof {
    pub source_tail_hash: String,
    pub observer_tail_hash: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReplayEquivalenceResult {
    pub equivalent: bool,
    pub reason: String,
    pub proof: ReplayEquivalenceProof,
}

impl ReplayEquivalenceRuntime {
    pub fn compare_streams(
        source: &ReplayTransportStream,
        observer: &ReplayTransportStream,
    ) -> ReplayEquivalenceResult {
        let proof = ReplayEquivalenceProof {
            source_tail_hash: source.cursor.last_continuity_hash.clone(),
            observer_tail_hash: observer.cursor.last_continuity_hash.clone(),
        };
        if source.accepted.len() != observer.accepted.len() {
            return ReplayEquivalenceResult {
                equivalent: false,
                reason: "chunk_count_mismatch".to_string(),
                proof,
            };
        }
        if proof.source_tail_hash != proof.observer_tail_hash {
            return ReplayEquivalenceResult {
                equivalent: false,
                reason: "continuity_hash_mismatch".to_string(),
                proof,
            };
        }
        ReplayEquivalenceResult {
            equivalent: true,
            reason: "equivalent".to_string(),
            proof,
        }
    }
}
