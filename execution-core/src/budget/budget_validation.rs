use super::{BudgetCheckResult, BudgetRejectionReason, ExecutionBudget};
use crate::economics::ResourceUsage;

pub fn validate_budget(usage: ResourceUsage, budget: ExecutionBudget) -> BudgetCheckResult {
    if usage.execution_units > budget.max_execution_units {
        return BudgetCheckResult::Rejected(BudgetRejectionReason::ExecutionOverflow);
    }
    if usage.replay_units > budget.max_replay_units {
        return BudgetCheckResult::Rejected(BudgetRejectionReason::ReplayOverflow);
    }
    if usage.proof_units > budget.max_proof_units {
        return BudgetCheckResult::Rejected(BudgetRejectionReason::ProofOverflow);
    }
    if usage.storage_units > budget.max_storage_units {
        return BudgetCheckResult::Rejected(BudgetRejectionReason::StorageOverflow);
    }
    BudgetCheckResult::Accepted
}
