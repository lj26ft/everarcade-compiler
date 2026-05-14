use everarcade_host::partition_recovery::{
    partition_detection::is_partitioned, stale_node::StaleNodeStatus,
};

#[test]
fn disconnected_node_detected_as_partitioned() {
    assert!(is_partitioned(&StaleNodeStatus {
        checkpoint_lag: 2,
        disconnected: true
    }));
}
