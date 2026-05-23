use execution_core::economy::vault::{vault_manifest_hash, VaultOwnershipRecord};
#[test]
fn vault_continuity() {
    let r1 = VaultOwnershipRecord {
        sequence: 1,
        owner: "alice".into(),
        settlement_hash: "s1".into(),
        previous_record_hash: String::new(),
    };
    let h1 = r1.canonical_hash().unwrap();
    let r2 = VaultOwnershipRecord {
        sequence: 2,
        owner: "bob".into(),
        settlement_hash: "s2".into(),
        previous_record_hash: h1,
    };
    assert_eq!(vault_manifest_hash(&[r1, r2]).unwrap().len(), 64);
}
