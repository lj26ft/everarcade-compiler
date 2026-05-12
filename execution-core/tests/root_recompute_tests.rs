use std::collections::BTreeMap;
use execution_core::hash_runtime::state_hash::state_root;

#[test]
fn state_root_is_deterministic() {
    let mut s = BTreeMap::new();
    s.insert("a".to_string(), "1".to_string());
    s.insert("b".to_string(), "2".to_string());
    assert_eq!(state_root(&s), state_root(&s));
}
