use super::{fiscal_policy::{FiscalPolicy, Hash}, fiscal_root::derive_fiscal_root, fiscal_lineage::derive_fiscal_lineage};

pub fn transition_fiscal_policy(prior: &FiscalPolicy, constitutional_root: Hash, budget_root: Hash) -> FiscalPolicy {
    let fiscal_root = derive_fiscal_root(constitutional_root, budget_root);
    FiscalPolicy { policy_id: prior.policy_id, constitutional_root, fiscal_root, lineage_root: derive_fiscal_lineage(prior.lineage_root, fiscal_root) }
}
