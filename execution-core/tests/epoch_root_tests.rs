use execution_core::epoch::{
    epoch_boundary::build_epoch_boundary, epoch_validation::validate_epoch,
};

#[test]
fn epoch_root_is_deterministic() {
    let epoch = build_epoch_boundary(2, [1; 32], [2; 32], [3; 32], [4; 32], [5; 32]);
    assert!(validate_epoch(&epoch));
}
