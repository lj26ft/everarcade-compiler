#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AiExecutionView { pub agent_id: String, pub deterministic: bool }

pub fn inspect_ai_execution(agent_id: &str) -> AiExecutionView { AiExecutionView { agent_id: agent_id.to_owned(), deterministic: true } }
