use execution_core::{
    federation::node::FederationNodeId,
    operator::continuity::Hash256,
    sync::{
        cursor::SyncCursor,
        history::{append_cursor_history, verify_cursor_monotonicity, CursorHistoryEntry},
        observer::{advance_observer, hash_observer_state, ObserverState},
        persistence::{load_observer_state, save_observer_state},
        resume::resume_from_cursor,
        rollback::detect_rollback,
    },
};
use tempfile::tempdir;

fn z(v: u8) -> Hash256 {
    [v; 32]
}
fn cursor(seq: u64, v: u8) -> SyncCursor {
    SyncCursor {
        latest_sequence: seq,
        latest_execution_id: z(v),
        latest_checkpoint_root: z(v + 1),
        latest_manifest_hash: z(v + 2),
        latest_lineage_hash: z(v + 3),
    }
}
fn state() -> ObserverState {
    ObserverState {
        world_id: "w".into(),
        operator: FederationNodeId::new([9; 32]),
        current_cursor: cursor(0, 1),
        highest_verified_sequence: 0,
        latest_checkpoint_root: z(2),
        synchronized: false,
    }
}

#[test]
fn test_observer_state_roundtrip() {
    let t = tempdir().unwrap();
    let s = state();
    save_observer_state(t.path(), &s).unwrap();
    assert_eq!(load_observer_state(t.path()).unwrap(), s);
}
#[test]
fn test_observer_state_hash_stable() {
    let s = state();
    assert_eq!(hash_observer_state(&s), hash_observer_state(&s));
}
#[test]
fn test_cursor_history_monotonic() {
    let mut h = vec![CursorHistoryEntry {
        sequence: 1,
        checkpoint_root: z(1),
        execution_id: z(2),
    }];
    append_cursor_history(
        &mut h,
        CursorHistoryEntry {
            sequence: 2,
            checkpoint_root: z(3),
            execution_id: z(4),
        },
    )
    .unwrap();
    assert!(verify_cursor_monotonicity(&h).is_ok());
}
#[test]
fn test_cursor_history_gap_fails() {
    let h = vec![
        CursorHistoryEntry {
            sequence: 1,
            checkpoint_root: z(1),
            execution_id: z(2),
        },
        CursorHistoryEntry {
            sequence: 3,
            checkpoint_root: z(3),
            execution_id: z(4),
        },
    ];
    assert!(verify_cursor_monotonicity(&h).is_err());
}
#[test]
fn test_rollback_detection() {
    let r = detect_rollback(&cursor(3, 1), &cursor(2, 1), &[]);
    assert!(r.rollback_detected);
}
#[test]
fn test_rollback_rejected() {
    let t = tempdir().unwrap();
    let s = ObserverState {
        current_cursor: cursor(2, 2),
        highest_verified_sequence: 2,
        latest_checkpoint_root: z(3),
        ..state()
    };
    save_observer_state(t.path(), &s).unwrap();
    let mut h = vec![];
    assert!(resume_from_cursor(t.path(), &[cursor(1, 1)], &mut h).is_ok());
}
#[test]
fn test_resume_from_cursor_success() {
    let t = tempdir().unwrap();
    save_observer_state(t.path(), &state()).unwrap();
    let mut h = vec![];
    let (_ws, we, s) = resume_from_cursor(t.path(), &[cursor(1, 4), cursor(2, 5)], &mut h).unwrap();
    assert_eq!(we, 2);
    assert_eq!(s.highest_verified_sequence, 2);
}
#[test]
fn test_resume_skips_verified_ranges() {
    let t = tempdir().unwrap();
    save_observer_state(
        t.path(),
        &ObserverState {
            current_cursor: cursor(2, 2),
            highest_verified_sequence: 2,
            latest_checkpoint_root: z(3),
            ..state()
        },
    )
    .unwrap();
    let mut h = vec![];
    let (_ws, we, s) = resume_from_cursor(t.path(), &[cursor(1, 1), cursor(2, 2)], &mut h).unwrap();
    assert_eq!(we, 2);
    assert_eq!(s.highest_verified_sequence, 2);
}
#[test]
fn test_resume_detects_divergence() {
    let r = detect_rollback(
        &cursor(2, 1),
        &SyncCursor {
            latest_sequence: 2,
            latest_execution_id: z(9),
            latest_checkpoint_root: z(2),
            latest_manifest_hash: z(3),
            latest_lineage_hash: z(4),
        },
        &[CursorHistoryEntry {
            sequence: 2,
            checkpoint_root: z(2),
            execution_id: z(1),
        }],
    );
    assert!(r.rollback_detected);
}
#[test]
fn test_observer_advancement_monotonic() {
    let mut s = state();
    assert!(advance_observer(&mut s, cursor(1, 3), true).is_ok());
    assert!(advance_observer(&mut s, cursor(3, 5), true).is_err());
}
#[test]
fn test_observer_sync_replay_consistency() {
    let mut s = state();
    let c = cursor(1, 6);
    advance_observer(&mut s, c.clone(), true).unwrap();
    assert_eq!(s.current_cursor.latest_execution_id, c.latest_execution_id);
}
#[test]
fn test_observer_checkpoint_continuity() {
    let mut s = state();
    let c = cursor(1, 6);
    advance_observer(&mut s, c.clone(), true).unwrap();
    assert_eq!(s.latest_checkpoint_root, c.latest_checkpoint_root);
}
#[test]
fn test_observer_resume_deterministic() {
    let t = tempdir().unwrap();
    save_observer_state(t.path(), &state()).unwrap();
    let mut h1 = vec![];
    let mut h2 = vec![];
    let a = resume_from_cursor(t.path(), &[cursor(1, 4)], &mut h1).unwrap();
    save_observer_state(t.path(), &state()).unwrap();
    let b = resume_from_cursor(t.path(), &[cursor(1, 4)], &mut h2).unwrap();
    assert_eq!(a.2, b.2);
}
