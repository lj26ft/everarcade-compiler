use super::{execution::BehaviorExecution, runtime::BehaviorTreeRuntime};
pub fn restore_behavior(executions: &[BehaviorExecution]) -> BehaviorTreeRuntime {
    let mut r = BehaviorTreeRuntime::default();
    for e in executions {
        r.tick = r.tick.max(e.tick + 1);
        r.executions.push(e.clone());
    }
    r
}
