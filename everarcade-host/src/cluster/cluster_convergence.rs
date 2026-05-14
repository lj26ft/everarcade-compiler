use super::local_cluster::LocalCluster;

pub fn converged(cluster: &LocalCluster) -> bool {
    cluster.nodes.len() >= 3
}
