use crate::distributed_execution::execution_window::DistributedExecutionWindow;

pub fn reconcile_windows(
    local: &[DistributedExecutionWindow],
    remote: &[DistributedExecutionWindow],
) -> Vec<DistributedExecutionWindow> {
    if remote.len() > local.len() {
        remote.to_vec()
    } else {
        local.to_vec()
    }
}
