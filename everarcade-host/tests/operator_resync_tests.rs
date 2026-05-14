use everarcade_host::runtime_recovery::{
    cluster_restart::restart_node, peer_resync::resync_from_peer,
};

#[test]
fn restarted_node_resyncs() {
    assert!(restart_node("node-a"));
    assert!(resync_from_peer("node-b"));
}
