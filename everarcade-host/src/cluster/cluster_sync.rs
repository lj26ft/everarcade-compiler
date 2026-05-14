use super::local_cluster::LocalCluster;

pub fn sync_artifacts(cluster: &LocalCluster) -> bool {
    cluster.nodes.len() >= 3
}
