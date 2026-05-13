use super::{treasury::SovereignTreasury, treasury_root::derive_treasury_root};

pub fn validate_treasury_replay(treasury: &SovereignTreasury) -> bool {
    treasury.treasury_root == derive_treasury_root(treasury.monetary_root, treasury.fiscal_root)
}
