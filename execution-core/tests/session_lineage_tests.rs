use execution_core::session::{
    session_lineage::session_continuity, session_root::materialize_session,
};

#[test]
fn session_parent_lineage_is_preserved() {
    let parent = materialize_session(None, [1; 32], [2; 32]);
    let child = materialize_session(Some(parent.session_id), [3; 32], [4; 32]);
    assert!(session_continuity(&parent, &child));
}
