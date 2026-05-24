use execution_core::deployment::{
    bundle_builder::BundleGenerationReceipt, operational_store::OperationalLedgerCheckpoint,
    runtime_restore::RestorationExecutionReceipt, stdout_runtime::DeterministicRuntimeLog,
};

pub fn assert_receipt_equivalence<T: PartialEq + core::fmt::Debug>(a: &T, b: &T) {
    assert_eq!(a, b, "receipt mismatch");
}

pub fn assert_bundle_equivalence(a: &BundleGenerationReceipt, b: &BundleGenerationReceipt) {
    assert_eq!(a.bundle_hash, b.bundle_hash);
    assert_eq!(a.proof, b.proof);
}

pub fn assert_restore_equivalence(
    a: &RestorationExecutionReceipt,
    b: &RestorationExecutionReceipt,
) {
    assert_eq!(a, b);
}

pub fn assert_stdout_determinism(a: &DeterministicRuntimeLog, b: &DeterministicRuntimeLog) {
    assert_eq!(a, b);
}

pub fn assert_operational_ledger_stability(
    a: &OperationalLedgerCheckpoint,
    b: &OperationalLedgerCheckpoint,
) {
    assert_eq!(a, b);
}
