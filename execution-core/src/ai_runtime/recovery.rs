use super::{decision::AiDecision, runtime::AiRuntime};

pub fn restore_ai(decisions: &[AiDecision]) -> AiRuntime {
    let mut runtime = AiRuntime::default();
    for decision in decisions {
        let _ = runtime
            .memory
            .append(&decision.entity_id, &decision.action, &decision.replay_root);
        runtime.decisions.push(decision.clone());
        runtime.tick = runtime.tick.max(decision.tick + 1);
    }
    runtime
}
