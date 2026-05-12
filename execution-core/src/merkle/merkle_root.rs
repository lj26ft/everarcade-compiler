use super::{merkle_tree::build_merkle_root, Hash};

pub fn canonical_merkle_root(leaves: &[Hash]) -> Hash {
    build_merkle_root(leaves)
}
