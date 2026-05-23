use execution_core::economy::inventory::InventoryState;
use std::collections::BTreeMap;
#[test]
fn inventory_determinism() {
    let mut st = InventoryState {
        ownership: BTreeMap::from([("asset".to_string(), "alice".to_string())]),
    };
    st.apply_transfer("asset", "alice", "bob").unwrap();
    let h = st.manifest_hash().unwrap();
    assert_eq!(h, st.manifest_hash().unwrap());
}
#[test]
fn divergence_rejection() {
    let mut st = InventoryState {
        ownership: BTreeMap::from([("asset".to_string(), "alice".to_string())]),
    };
    assert!(st.apply_transfer("asset", "mallory", "bob").is_err());
}
