use super::local_cluster::LocalCluster;

pub fn recover_cluster(cluster: &LocalCluster) -> bool {
    !cluster.nodes.is_empty()
}
