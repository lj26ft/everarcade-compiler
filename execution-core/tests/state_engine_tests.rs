use std::collections::BTreeMap;

use execution_core::state_engine::{history, merkle, proof, snapshot::StateSnapshot, store::StateStore};
use everarcade_abi::StateChange;

#[test]
fn test_state_root_determinism() {
    let entries = vec![("a".to_string(), "1".to_string()), ("b".to_string(), "2".to_string())];
    assert_eq!(merkle::compute_state_root(&entries), merkle::compute_state_root(&entries));
}

#[test]
fn test_state_order_independence() {
    let mut a = BTreeMap::new(); a.insert("b".into(), "2".into()); a.insert("a".into(), "1".into());
    let mut b = BTreeMap::new(); b.insert("a".into(), "1".into()); b.insert("b".into(), "2".into());
    let ra = StateStore::new(a).root();
    let rb = StateStore::new(b).root();
    assert_eq!(ra, rb);
}

#[test]
fn test_state_change_application() {
    let mut s = StateStore::default();
    s.apply_changes(&[StateChange{key:"x".into(),before:"".into(),after:"1".into()}]);
    assert_eq!(s.get("x"), Some(&"1".to_string()));
}

#[test]
fn test_snapshot_replay() {
    let mut map = BTreeMap::new(); map.insert("k".into(), "v".into());
    let snap = StateSnapshot::new(map, None);
    assert_eq!(history::replay_from_snapshot(&snap), snap.state_root);
    assert!(history::verify_historical_state(&snap));
}

#[test]
fn test_merkle_proof_verification() {
    let p = proof::create_inclusion_proof("k", "v");
    let leaf = merkle::to_hex(&merkle::hash_leaf("k", "v"));
    assert!(proof::verify_inclusion_proof(&p, &leaf));
}

#[test]
fn test_large_state_determinism() {
    let mut map = BTreeMap::new();
    for i in 0..1000 { map.insert(format!("k{i}"), format!("v{i}")); }
    let r1 = StateStore::new(map.clone()).root();
    let r2 = StateStore::new(map).root();
    assert_eq!(r1, r2);
}
