#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SovereignReleaseLineageRuntime {
    pub chain: SovereignReleaseLineageChain,
    pub proof_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SovereignReleaseAncestor {
    pub release_id: String,
    pub replay_anchor: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SovereignReleaseLineageProof {
    pub proof_id: String,
    pub chain_len: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SovereignReleaseLineageChain {
    pub ancestors: Vec<SovereignReleaseAncestor>,
}

impl SovereignReleaseLineageRuntime {
    pub fn verify_continuity(&self) -> bool {
        !self.chain.ancestors.is_empty()
            && self
                .chain
                .ancestors
                .windows(2)
                .all(|w| w[0].release_id != w[1].release_id)
    }

    pub fn proof(&self) -> SovereignReleaseLineageProof {
        SovereignReleaseLineageProof {
            proof_id: self.proof_id.clone(),
            chain_len: self.chain.ancestors.len(),
        }
    }
}
