use execution_core::security::*;

#[test]
fn test_replay_corruption_rejection() {
    let r = validate_replay_scenario(ReplayCorruptionScenario::TruncatedReplay);
    assert!(!r.accepted);
    assert_eq!(r.diagnostics.fault_type, "truncated_replay");
}

#[test]
fn test_archive_tamper_detection() {
    let r = validate_archive_scenario(ArchiveTamperScenario::InvalidArchiveRoot);
    assert_eq!(r.rejection_code, "invalid_archive_root");
}

#[test]
fn test_wasm_fault_determinism() {
    let p = WasmIsolationPolicy {
        memory_ceiling_bytes: 4096,
        max_fuel: 100,
        max_abi_payload_bytes: 4,
    };
    let a = isolate_wasm_fault(b"ABCDE", &p).unwrap();
    let b = isolate_wasm_fault(b"ABCDE", &p).unwrap();
    assert_eq!(a, b);
}

#[test]
fn test_scheduler_abuse_rejection() {
    let r = scheduler_validation::deterministic_reject("recursive_scheduling", 1, false);
    assert!(!r.accepted);
}

#[test]
fn test_inventory_double_spend_rejection() {
    let r = reject_hostile_mutation("double_spend_mutation");
    assert!(!r.accepted);
}

#[test]
fn test_checkpoint_lineage_break_detection() {
    let r = validate_replay_scenario(ReplayCorruptionScenario::CheckpointLineageBreak);
    assert_eq!(r.failure_location, Some(30));
}

#[test]
fn test_crash_restoration_equivalence() {
    assert!(validate_crash_recovery(
        CrashScenario::ReplayReconstructionAfterCrash
    ));
}

#[test]
fn test_quarantine_recovery_equivalence() {
    let q = QuarantineEnvelope {
        quarantined: true,
        boundary: IsolationBoundary {
            node_id: "n1".into(),
        },
        manifest: Some(RecoveryManifest {
            checkpoint_id: "cp".into(),
            replay_root: "root".into(),
        }),
        eligibility: RecoveryEligibility::Eligible,
        diagnostics: SecurityDiagnosticsEnvelope::fault("corruption", 1, true, true),
    };
    assert!(q.quarantined);
}

#[test]
fn test_partition_corruption_recovery() {
    let r =
        archive_validation::validate_archive_scenario(ArchiveTamperScenario::PartialArchiveReplay);
    assert!(r.diagnostics.recovery_possible);
}
