use everarcade_host::signing::signature_validation::derive_signature_root;
#[test]
fn signature_stability_receipt() {
    assert_eq!(
        derive_signature_root([3; 32], [9; 32]),
        derive_signature_root([3; 32], [9; 32])
    );
}
