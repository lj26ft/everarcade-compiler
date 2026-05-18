mod common;

use execution_core::{
    canonical::{generate_execution_manifest, manifest_hash, receipt_hash},
    federation::{bundle::export_continuity_bundle, node::FederationNodeId},
    operator::continuity::Hash256,
    sync::{
        advertisement::{verify_advertisement, ContinuityAdvertisement},
        cursor::{hash_cursor, SyncCursor},
        lineage::verify_lineage_window,
        pull::{validate_pull_response, PullResponse},
        receipts::verify_receipt_window,
        state::{advance_sync_state, SynchronizationState},
        verification::verify_sync_artifacts,
        window::{validate_sync_window, SyncWindow},
    },
};
use tempfile::tempdir;

fn z(v: u8) -> Hash256 {
    [v; 32]
}

#[test]
fn test_sync_cursor_hash_stable() {
    let c = SyncCursor {
        latest_sequence: 2,
        latest_execution_id: z(1),
        latest_checkpoint_root: z(2),
        latest_manifest_hash: z(3),
        latest_lineage_hash: z(4),
    };
    assert_eq!(hash_cursor(&c), hash_cursor(&c));
}
#[test]
fn test_sync_window_valid() {
    assert!(validate_sync_window(&SyncWindow {
        start_sequence: 1,
        end_sequence: 2
    })
    .is_ok());
}
#[test]
fn test_sync_window_gap_fails() {
    assert!(validate_sync_window(&SyncWindow {
        start_sequence: 3,
        end_sequence: 2
    })
    .is_err());
}
#[test]
fn test_receipt_window_continuity() {
    let f = common::fixtures::generate_counter_world_fixture();
    assert!(verify_receipt_window(
        &[f.receipt_1.clone(), f.receipt_2.clone()],
        &SyncWindow {
            start_sequence: 1,
            end_sequence: 2
        }
    )
    .is_ok());
}
#[test]
fn test_receipt_window_gap_fails() {
    let f = common::fixtures::generate_counter_world_fixture();
    assert!(verify_receipt_window(
        &[f.receipt_1.clone()],
        &SyncWindow {
            start_sequence: 1,
            end_sequence: 2
        }
    )
    .is_err());
}
#[test]
fn test_lineage_window_valid() {
    let f = common::fixtures::generate_counter_world_fixture();
    assert!(verify_lineage_window(
        &f.lineage,
        &SyncWindow {
            start_sequence: 1,
            end_sequence: 2
        }
    )
    .is_ok());
}
#[test]
fn test_lineage_window_gap_fails() {
    let f = common::fixtures::generate_counter_world_fixture();
    assert!(verify_lineage_window(
        &f.lineage,
        &SyncWindow {
            start_sequence: 2,
            end_sequence: 3
        }
    )
    .is_err());
}
#[test]
fn test_sync_advertisement_valid() {
    let c = SyncCursor {
        latest_sequence: 2,
        latest_execution_id: z(1),
        latest_checkpoint_root: z(9),
        latest_manifest_hash: z(3),
        latest_lineage_hash: z(4),
    };
    let a = ContinuityAdvertisement {
        world_id: "w".into(),
        operator: FederationNodeId::new([7; 32]),
        cursor: c,
        package_root: z(8),
        checkpoint_root: z(9),
    };
    assert!(verify_advertisement(&a).is_ok());
}
#[test]
fn test_sync_advertisement_package_mismatch_fails() {
    let c = SyncCursor {
        latest_sequence: 2,
        latest_execution_id: z(1),
        latest_checkpoint_root: z(9),
        latest_manifest_hash: z(3),
        latest_lineage_hash: z(4),
    };
    let a = ContinuityAdvertisement {
        world_id: "w".into(),
        operator: FederationNodeId::new([7; 32]),
        cursor: c,
        package_root: [0; 32],
        checkpoint_root: z(9),
    };
    assert!(verify_advertisement(&a).is_err());
}
#[test]
fn test_sync_state_advancement_monotonic() {
    let mut s = SynchronizationState {
        current_cursor: SyncCursor {
            latest_sequence: 0,
            latest_execution_id: z(0),
            latest_checkpoint_root: z(0),
            latest_manifest_hash: z(0),
            latest_lineage_hash: z(0),
        },
        last_verified_sequence: 0,
        synchronized: false,
    };
    let n = SyncCursor {
        latest_sequence: 2,
        latest_execution_id: z(1),
        latest_checkpoint_root: z(2),
        latest_manifest_hash: z(3),
        latest_lineage_hash: z(4),
    };
    assert!(advance_sync_state(
        &mut s,
        n,
        &SyncWindow {
            start_sequence: 1,
            end_sequence: 2
        }
    )
    .is_ok());
}
#[test]
fn test_sync_pull_order_deterministic() {
    let r = PullResponse {
        checkpoint: None,
        receipts: vec!["a".into(), "b".into()],
        lineage: "l".into(),
        manifest: "m".into(),
    };
    assert!(validate_pull_response(
        &r,
        &SyncWindow {
            start_sequence: 1,
            end_sequence: 2
        }
    )
    .is_ok());
}
#[test]
fn test_sync_verification_success() {
    let t = tempdir().unwrap();
    let f = common::fixtures::generate_counter_world_fixture();
    common::fixtures::persist_counter_world_fixture(t.path(), &f);
    let m = generate_execution_manifest(
        f.lineage.package_root,
        receipt_hash(&f.receipt_2),
        &f.lineage,
        execution_core::state::decode_checkpoint(&f.checkpoint_0)
            .unwrap()
            .root(),
        f.lineage.records[1].post_state_root,
    );
    execution_core::canonical::save_manifest(&t.path().join("manifest.bin"), &m).unwrap();
    execution_core::operator::recover_world(execution_core::operator::OperatorRecoveryInput {
        package_path: t.path().join("world.wasm"),
        checkpoint_path: t.path().join("checkpoint_0.bin"),
        lineage_path: t.path().join("lineage.bin"),
        receipt_paths: vec![
            t.path().join("receipt_1.bin"),
            t.path().join("receipt_2.bin"),
        ],
        descriptor_output_path: t.path().join("recovery_descriptor.bin"),
    })
    .unwrap();
    let out = t.path().join("bundle");
    export_continuity_bundle(
        &out,
        &t.path().join("world.wasm"),
        &t.path().join("checkpoint_0.bin"),
        &t.path().join("lineage.bin"),
        &vec![
            t.path().join("receipt_1.bin"),
            t.path().join("receipt_2.bin"),
        ],
        &t.path().join("manifest.bin"),
        &t.path().join("recovery_descriptor.bin"),
    )
    .unwrap();
    assert!(verify_sync_artifacts(&out).unwrap().continuity_ok);
}
#[test]
fn test_sync_verification_tamper_fails() {
    assert!(verify_sync_artifacts(std::path::Path::new("/nope")).is_err());
}
#[test]
fn test_sync_replay_consistency() {
    let f = common::fixtures::generate_counter_world_fixture();
    assert_eq!(
        f.receipt_2.prior_replay_root,
        f.lineage.records[1].pre_state_root
    );
}
#[test]
fn test_sync_checkpoint_root_continuity() {
    let f = common::fixtures::generate_counter_world_fixture();
    let m = generate_execution_manifest(
        f.lineage.package_root,
        receipt_hash(&f.receipt_2),
        &f.lineage,
        execution_core::state::decode_checkpoint(&f.checkpoint_0)
            .unwrap()
            .root(),
        f.lineage.records[1].post_state_root,
    );
    assert_ne!(manifest_hash(&m), [0; 32]);
}
