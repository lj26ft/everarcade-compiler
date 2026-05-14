use everarcade_host::cluster::{
    cluster_sync::sync_artifacts, local_cluster::LocalCluster, operator_node::OperatorNode,
};

#[test]
fn three_nodes_can_sync() {
    let cluster = LocalCluster::with_nodes(vec![
        OperatorNode {
            node_id: "A".into(),
            state_dir: "a".into(),
        },
        OperatorNode {
            node_id: "B".into(),
            state_dir: "b".into(),
        },
        OperatorNode {
            node_id: "C".into(),
            state_dir: "c".into(),
        },
    ]);
    assert!(sync_artifacts(&cluster));
}
