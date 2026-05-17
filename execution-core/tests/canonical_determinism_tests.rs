mod common;

use everarcade_abi::StateChange;
use execution_core::canonical::{
    canonical_decode, canonical_encode, generate_execution_manifest, lineage_hash, manifest_hash,
    receipt_hash, state_root_hash, CanonicalExecutionManifest,
};
use execution_core::lineage::{ExecutionLineageChain, ExecutionLineageRecord};
use execution_core::vm::VmExecutionReceipt;

fn sample_hash(v: u8) -> [u8; 32] {
    [v; 32]
}

fn sample_lineage() -> ExecutionLineageChain {
    ExecutionLineageChain {
        world_id: sample_hash(9),
        package_root: sample_hash(1),
        records: vec![ExecutionLineageRecord {
            sequence: 0,
            previous_execution_id: None,
            execution_id: sample_hash(2),
            pre_state_root: sample_hash(3),
            post_state_root: sample_hash(4),
            receipt_hash: sample_hash(6),
            package_root: sample_hash(1),
        }],
    }
}
fn sample_receipt() -> VmExecutionReceipt {
    VmExecutionReceipt {
        receipt_id: sample_hash(6),
        package_root: sample_hash(1),
        prior_replay_root: sample_hash(2),
        next_replay_root: sample_hash(3),
        execution_root: sample_hash(4),
        checkpoint_root: sample_hash(5),
        anchor_root: sample_hash(7),
        state_diff: vec![StateChange {
            key: "k".into(),
            before: "0".into(),
            after: "1".into(),
        }],
    }
}
#[test]
fn test_manifest_hash_stable() {
    let m = CanonicalExecutionManifest {
        package_root: sample_hash(1),
        receipt_hash: sample_hash(2),
        lineage_hash: sample_hash(3),
        checkpoint_root: sample_hash(4),
        final_state_root: sample_hash(5),
    };
    assert_eq!(manifest_hash(&m), manifest_hash(&m));
}
#[test]
fn test_manifest_roundtrip() {
    let m = CanonicalExecutionManifest {
        package_root: sample_hash(1),
        receipt_hash: sample_hash(2),
        lineage_hash: sample_hash(3),
        checkpoint_root: sample_hash(4),
        final_state_root: sample_hash(5),
    };
    let b = canonical_encode(&m).unwrap();
    let d: CanonicalExecutionManifest = canonical_decode(&b).unwrap();
    assert_eq!(m, d);
}
#[test]
fn test_canonical_encoding_stable() {
    let m = CanonicalExecutionManifest {
        package_root: sample_hash(1),
        receipt_hash: sample_hash(2),
        lineage_hash: sample_hash(3),
        checkpoint_root: sample_hash(4),
        final_state_root: sample_hash(5),
    };
    assert_eq!(canonical_encode(&m).unwrap(), canonical_encode(&m).unwrap());
}
#[test]
fn test_receipt_hash_stable() {
    let r = sample_receipt();
    assert_eq!(receipt_hash(&r), receipt_hash(&r));
}
#[test]
fn test_lineage_hash_stable() {
    let l = sample_lineage();
    assert_eq!(lineage_hash(&l), lineage_hash(&l));
}
#[test]
fn test_state_root_hash_stable() {
    let s = execution_core::state::CanonicalState::default();
    assert_eq!(state_root_hash(&s), state_root_hash(&s));
}
#[test]
fn test_cross_run_manifest_equivalence() {
    let l = sample_lineage();
    let r = sample_receipt();
    let m1 = generate_execution_manifest(
        sample_hash(1),
        receipt_hash(&r),
        &l,
        sample_hash(7),
        sample_hash(8),
    );
    let m2 = generate_execution_manifest(
        sample_hash(1),
        receipt_hash(&r),
        &l,
        sample_hash(7),
        sample_hash(8),
    );
    assert_eq!(manifest_hash(&m1), manifest_hash(&m2));
}
#[test]
fn test_determinism_verify_success() {
    let m = CanonicalExecutionManifest {
        package_root: sample_hash(1),
        receipt_hash: sample_hash(2),
        lineage_hash: sample_hash(3),
        checkpoint_root: sample_hash(4),
        final_state_root: sample_hash(5),
    };
    assert_eq!(manifest_hash(&m), manifest_hash(&m));
}
#[test]
fn test_manifest_mismatch_fails() {
    let a = CanonicalExecutionManifest {
        package_root: sample_hash(1),
        receipt_hash: sample_hash(2),
        lineage_hash: sample_hash(3),
        checkpoint_root: sample_hash(4),
        final_state_root: sample_hash(5),
    };
    let b = CanonicalExecutionManifest {
        package_root: sample_hash(1),
        receipt_hash: sample_hash(2),
        lineage_hash: sample_hash(3),
        checkpoint_root: sample_hash(4),
        final_state_root: sample_hash(6),
    };
    assert_ne!(manifest_hash(&a), manifest_hash(&b));
}

#[test]
fn test_fixture_generation_stable() {
    let a = common::fixtures::generate_counter_world_fixture();
    let b = common::fixtures::generate_counter_world_fixture();
    assert_eq!(a.checkpoint_0, b.checkpoint_0);
    assert_eq!(a.checkpoint_1, b.checkpoint_1);
    assert_eq!(a.checkpoint_2, b.checkpoint_2);
    assert_eq!(
        execution_core::canonical::receipt_hash(&a.receipt_1),
        execution_core::canonical::receipt_hash(&b.receipt_1)
    );
    assert_eq!(
        execution_core::canonical::receipt_hash(&a.receipt_2),
        execution_core::canonical::receipt_hash(&b.receipt_2)
    );
    assert_eq!(
        execution_core::canonical::lineage_hash(&a.lineage),
        execution_core::canonical::lineage_hash(&b.lineage)
    );

    let ta = tempfile::tempdir().unwrap();
    let tb = tempfile::tempdir().unwrap();
    let oa = common::fixtures::generate_operator_recovery_fixture(ta.path());
    let ob = common::fixtures::generate_operator_recovery_fixture(tb.path());
    assert_eq!(oa.manifest_hash, ob.manifest_hash);
    assert_eq!(oa.descriptor_hash, ob.descriptor_hash);
}
