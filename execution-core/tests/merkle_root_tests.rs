use execution_core::merkle::{leaf_hash::leaf_hash, merkle_tree::build_merkle_root};

#[test]
fn merkle_root_is_deterministic() {
    let leaves = vec![leaf_hash(b"a"), leaf_hash(b"b"), leaf_hash(b"c")];
    assert_eq!(build_merkle_root(&leaves), build_merkle_root(&leaves));
}
