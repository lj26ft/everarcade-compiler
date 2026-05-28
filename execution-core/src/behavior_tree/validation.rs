use super::{execution::BehaviorExecution, runtime::BehaviorTreeRuntime};
pub fn execution_is_ordered(events: &[BehaviorExecution]) -> bool {
    events
        .windows(2)
        .all(|w| w[0].node_id <= w[1].node_id || w[0].tick <= w[1].tick)
        && events.iter().all(|e| !e.replay_root.is_empty())
}
pub fn behavior_equivalent(a: &BehaviorTreeRuntime, b: &BehaviorTreeRuntime) -> bool {
    a == b
}
