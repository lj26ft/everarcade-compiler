use super::execution_window::GovernanceExecutionWindow;

pub fn schedule_governance_window(
    windows: &[GovernanceExecutionWindow],
    window: GovernanceExecutionWindow,
) -> Vec<GovernanceExecutionWindow> {
    let mut ordered = windows.to_vec();
    ordered.push(window);
    ordered.sort_by(|a, b| a.window_root.cmp(&b.window_root));
    ordered
}
