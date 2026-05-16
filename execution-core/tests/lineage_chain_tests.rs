use execution_core::lineage::{
    load_lineage, save_lineage, validate_lineage_chain, ExecutionLineageChain, ExecutionLineageRecord,
    LineageError,
};

fn h(v: u8) -> [u8; 32] { [v; 32] }

fn build_chain() -> ExecutionLineageChain {
    let r1 = ExecutionLineageRecord {
        sequence: 1,
        previous_execution_id: None,
        execution_id: h(1),
        pre_state_root: h(10),
        post_state_root: h(11),
        receipt_hash: h(12),
        package_root: h(9),
    };
    let r2 = ExecutionLineageRecord {
        sequence: 2,
        previous_execution_id: Some(h(1)),
        execution_id: h(2),
        pre_state_root: h(11),
        post_state_root: h(13),
        receipt_hash: h(14),
        package_root: h(9),
    };
    ExecutionLineageChain { world_id: h(99), package_root: h(9), records: vec![r1, r2] }
}

#[test]
fn test_lineage_chain_valid() { assert!(validate_lineage_chain(&build_chain()).is_ok()); }

#[test]
fn test_lineage_sequence_gap_fails() {
    let mut c = build_chain(); c.records[1].sequence = 3;
    let err = validate_lineage_chain(&c).unwrap_err();
    assert!(matches!(err, LineageError::Validation(_)));
}

#[test]
fn test_lineage_previous_execution_mismatch_fails() {
    let mut c = build_chain(); c.records[1].previous_execution_id = Some(h(42));
    let err = validate_lineage_chain(&c).unwrap_err();
    assert!(matches!(err, LineageError::Validation(_)));
}

#[test]
fn test_lineage_state_root_mismatch_fails() {
    let mut c = build_chain(); c.records[1].pre_state_root = h(99);
    let err = validate_lineage_chain(&c).unwrap_err();
    assert!(matches!(err, LineageError::Validation(_)));
}

#[test]
fn test_lineage_package_root_mismatch_fails() {
    let mut c = build_chain(); c.records[1].package_root = h(88);
    let err = validate_lineage_chain(&c).unwrap_err();
    assert!(matches!(err, LineageError::Validation(_)));
}

#[test]
fn test_lineage_save_load_roundtrip() {
    let temp = tempfile::tempdir().unwrap();
    let p = temp.path().join("lineage.bin");
    let c = build_chain();
    save_lineage(&p, &c).unwrap();
    let out = load_lineage(&p).unwrap();
    assert_eq!(c, out);
}

#[test]
fn test_two_step_counter_world_lineage_shape() {
    let c = build_chain();
    assert_eq!(c.records.len(), 2);
    assert_eq!(c.records[0].post_state_root, c.records[1].pre_state_root);
    assert_eq!(c.records[1].previous_execution_id, Some(c.records[0].execution_id));
    assert!(validate_lineage_chain(&c).is_ok());
}
