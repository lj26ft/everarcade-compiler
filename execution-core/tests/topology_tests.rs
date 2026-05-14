use execution_core::execution::topology::{canonical_topological_sort, detect_execution_cycles};
use std::collections::{BTreeMap, BTreeSet};

#[test]
fn cycle_detected() {
    let mut edges = BTreeMap::new();
    edges.insert("a".to_string(), BTreeSet::from(["b".to_string()]));
    edges.insert("b".to_string(), BTreeSet::from(["a".to_string()]));
    assert!(detect_execution_cycles(&edges));
    assert!(canonical_topological_sort(&edges).is_err());
}
