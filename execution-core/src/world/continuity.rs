use crate::{canonical::encoding::canonical_encode, hashing::hash_bytes};
use serde::{Deserialize, Serialize};

use super::epochs::EpochContinuityProof;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldContinuityRoot(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityDivergence {
    pub expected: WorldContinuityRoot,
    pub observed: WorldContinuityRoot,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldEpochChain {
    pub epoch_hashes: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldLineage {
    pub world_id: String,
    pub chain: WorldEpochChain,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldRestorationProof {
    pub world_id: String,
    pub restored_continuity_root: WorldContinuityRoot,
    pub chain_tip: String,
}

impl WorldLineage {
    pub fn continuity_root(&self) -> Result<WorldContinuityRoot, String> {
        Ok(WorldContinuityRoot(hash_bytes(
            &canonical_encode(self).map_err(|e| e.to_string())?,
        )))
    }

    pub fn append_epoch(&mut self, proof: &EpochContinuityProof) {
        self.chain.epoch_hashes.push(proof.epoch_hash.clone());
    }

    pub fn restoration_proof(&self) -> Result<WorldRestorationProof, String> {
        let root = self.continuity_root()?;
        let tip = self.chain.epoch_hashes.last().cloned().unwrap_or_default();
        Ok(WorldRestorationProof {
            world_id: self.world_id.clone(),
            restored_continuity_root: root,
            chain_tip: tip,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityWitness {
    pub world_id: String,
    pub chain_root: String,
}
