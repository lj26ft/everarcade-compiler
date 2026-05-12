use execution_core::budget::{budget_validation::validate_budget, BudgetCheckResult, BudgetRejectionReason, ExecutionBudget};
use execution_core::economics::ResourceUsage;

#[test]
fn deterministic_budget_rejection() {
    let budget = ExecutionBudget { max_execution_units: 1, max_replay_units: 1, max_proof_units: 1, max_storage_units: 1 };
    let usage = ResourceUsage { execution_units: 2, replay_units: 0, proof_units: 0, storage_units: 0 };
    assert_eq!(validate_budget(usage, budget), BudgetCheckResult::Rejected(BudgetRejectionReason::ExecutionOverflow));
}
