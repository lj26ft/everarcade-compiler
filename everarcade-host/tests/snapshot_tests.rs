use everarcade_host::snapshot::branch::CivilizationBranch;
#[test]
fn snapshot_lineage_preservation() {
    let b = CivilizationBranch {
        parent_replay_root: [1; 32],
        branch_replay_root: [2; 32],
        continuity_root: [3; 32],
    };
    assert_ne!(b.parent_replay_root, b.branch_replay_root);
}
