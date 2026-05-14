use super::merkle;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MerkleProof {
    pub key: String,
    pub value: String,
}

pub fn create_inclusion_proof(key: &str, value: &str) -> MerkleProof {
    MerkleProof {
        key: key.to_string(),
        value: value.to_string(),
    }
}

pub fn verify_inclusion_proof(proof: &MerkleProof, expected_leaf_hash_hex: &str) -> bool {
    merkle::to_hex(&merkle::hash_leaf(&proof.key, &proof.value)) == expected_leaf_hash_hex
}
