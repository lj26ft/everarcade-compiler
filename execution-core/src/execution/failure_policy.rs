use super::execution_node::ExecutionPolicy;

pub fn should_rollback(policy: ExecutionPolicy) -> bool {
    matches!(policy, ExecutionPolicy::Required | ExecutionPolicy::ForkOnFailure)
}
