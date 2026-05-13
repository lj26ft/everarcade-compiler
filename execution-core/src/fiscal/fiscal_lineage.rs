use super::fiscal_policy::Hash;

pub fn derive_fiscal_lineage(prior: Hash, fiscal_root: Hash) -> Hash { let mut out=[0;32]; for i in 0..32 { out[i]=prior[i]^fiscal_root[i]; } out }
