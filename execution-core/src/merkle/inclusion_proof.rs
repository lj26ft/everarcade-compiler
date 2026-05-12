use super::{leaf_hash::inner_hash, merkle_tree::build_merkle_root, Hash};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InclusionProof {
    pub leaf_index: usize,
    pub leaf_count: usize,
    pub siblings: Vec<Hash>,
}

pub fn generate_inclusion_proof(leaves: &[Hash], target_index: usize) -> InclusionProof {
    assert!(target_index < leaves.len(), "target index out of bounds");
    let mut index = target_index;
    let mut level = leaves.to_vec();
    let mut siblings = Vec::new();

    while level.len() > 1 {
        if level.len() % 2 == 1 {
            level.push(*level.last().expect("non-empty"));
        }
        let sibling_index = if index % 2 == 0 { index + 1 } else { index - 1 };
        siblings.push(level[sibling_index]);
        let mut next = Vec::with_capacity(level.len() / 2);
        for pair in level.chunks_exact(2) {
            next.push(inner_hash(pair[0], pair[1]));
        }
        index /= 2;
        level = next;
    }

    InclusionProof { leaf_index: target_index, leaf_count: leaves.len(), siblings }
}

pub fn verify_inclusion_proof(root: Hash, leaf: Hash, proof: &InclusionProof) -> bool {
    if proof.leaf_count == 0 || proof.leaf_index >= proof.leaf_count {
        return false;
    }
    let mut index = proof.leaf_index;
    let mut cur = leaf;
    for sibling in &proof.siblings {
        cur = if index % 2 == 0 { inner_hash(cur, *sibling) } else { inner_hash(*sibling, cur) };
        index /= 2;
    }
    cur == root
}

pub fn proof_root(leaves: &[Hash], idx: usize) -> Hash {
    let proof = generate_inclusion_proof(leaves, idx);
    let leaf = leaves[idx];
    if verify_inclusion_proof(build_merkle_root(leaves), leaf, &proof) { build_merkle_root(leaves) } else { [0u8;32] }
}
