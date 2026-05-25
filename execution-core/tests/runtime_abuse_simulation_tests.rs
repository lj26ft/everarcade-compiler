use execution_core::security::*;

#[test]
fn simulate_abuse_patterns_deterministic_rejection() {
    let mut q = RuntimeQuarantine { receipts: vec![] };
    for (id, reason) in [
        ("spam", QuarantineReason::MaliciousExecutionArtifact),
        ("replay", QuarantineReason::InvalidReplayWindow),
        ("snapshot", QuarantineReason::InvalidSnapshot),
        ("restore", QuarantineReason::MalformedRestorationChain),
        ("witness", QuarantineReason::CorruptedWitnessBundle),
    ] {
        q.receipts.push(QuarantineReceipt {
            execution_id: id.into(),
            reason,
            deterministic: true,
        });
    }
    assert_eq!(q.receipts.len(), 5);
    assert!(q.receipts.iter().all(|r| r.deterministic));
}
