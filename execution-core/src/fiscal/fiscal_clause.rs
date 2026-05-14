pub type Hash = [u8; 32];

pub fn execute_fiscal_clause(clause_root: Hash, budget_root: Hash) -> Hash {
    let mut out = [0; 32];
    for i in 0..32 {
        out[i] = clause_root[i] ^ budget_root[i];
    }
    out
}
