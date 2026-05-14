use everarcade_host::reconciliation::{
    peer_comparison::PeerContinuity, replay_selection::select_highest_valid_replay,
};

#[test]
fn highest_valid_replay_continuity_selected() {
    let peers = vec![
        PeerContinuity {
            peer_id: "a".into(),
            continuity_height: 10,
            valid: true,
        },
        PeerContinuity {
            peer_id: "b".into(),
            continuity_height: 15,
            valid: true,
        },
    ];
    assert_eq!(select_highest_valid_replay(&peers).unwrap().peer_id, "b");
}
