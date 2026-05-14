use super::stale_node::StaleNodeStatus;

pub fn is_partitioned(status: &StaleNodeStatus) -> bool {
    status.disconnected && status.checkpoint_lag > 0
}
