use super::{fiscal_policy::FiscalPolicy, fiscal_root::derive_fiscal_root};

pub fn validate_fiscal_policy(policy: &FiscalPolicy, budget_root: [u8; 32]) -> bool {
    policy.fiscal_root == derive_fiscal_root(policy.constitutional_root, budget_root)
}
