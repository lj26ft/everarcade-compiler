use execution_core::hashing::hash_bytes;

#[test]
fn same_ordered_leaves_same_root() {
    let leaves = ["a", "b", "c"];
    let root1 = hash_bytes(leaves.join("|").as_bytes());
    let root2 = hash_bytes(leaves.join("|").as_bytes());
    assert_eq!(root1, root2);
}
