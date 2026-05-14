use super::ResourceUsage;
use crate::budget::ExecutionBudget;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CostValidationError {
    ExecutionOverflow,
    ReplayOverflow,
    ProofOverflow,
    StorageOverflow,
}

pub fn validate_usage_against_budget(
    usage: ResourceUsage,
    budget: ExecutionBudget,
) -> Result<(), CostValidationError> {
    if usage.execution_units > budget.max_execution_units {
        return Err(CostValidationError::ExecutionOverflow);
    }
    if usage.replay_units > budget.max_replay_units {
        return Err(CostValidationError::ReplayOverflow);
    }
    if usage.proof_units > budget.max_proof_units {
        return Err(CostValidationError::ProofOverflow);
    }
    if usage.storage_units > budget.max_storage_units {
        return Err(CostValidationError::StorageOverflow);
    }
    Ok(())
}
