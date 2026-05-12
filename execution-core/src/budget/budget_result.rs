#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BudgetRejectionReason { ExecutionOverflow, ReplayOverflow, ProofOverflow, StorageOverflow }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BudgetCheckResult { Accepted, Rejected(BudgetRejectionReason) }
