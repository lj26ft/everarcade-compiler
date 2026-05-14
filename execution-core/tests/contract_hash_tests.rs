use execution_core::hashing;

#[test]
fn test_identical_wasm_same_hash() {
    let wasm = b"\0asm\x01\0\0\0";
    assert_eq!(
        hashing::compute_contract_hash(wasm),
        hashing::compute_contract_hash(wasm)
    );
}

#[test]
fn test_modified_wasm_changes_hash() {
    let a = b"\0asm\x01\0\0\0";
    let b = b"\0asm\x01\0\0\x01";
    assert_ne!(
        hashing::compute_contract_hash(a),
        hashing::compute_contract_hash(b)
    );
}
