use super::{treasury::{Hash, SovereignTreasury}, treasury_root::derive_treasury_root};

pub fn transition_treasury(prior: &SovereignTreasury, monetary_root: Hash, fiscal_root: Hash) -> SovereignTreasury {
    SovereignTreasury {
        treasury_id: prior.treasury_id,
        sovereign_domain: prior.sovereign_domain,
        treasury_root: derive_treasury_root(monetary_root, fiscal_root),
        monetary_root,
        fiscal_root,
    }
}
