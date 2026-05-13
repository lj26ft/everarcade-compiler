use super::fiscal_policy::Hash;

pub fn derive_fiscal_root(constitutional_root: Hash, budget_root: Hash) -> Hash {
    let mut out=[0;32]; for i in 0..32 { out[i]=constitutional_root[i].wrapping_add(budget_root[i]); } out
}
