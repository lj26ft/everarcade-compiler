use super::clause::{ConstitutionalClause, Hash};
pub fn constitutional_root(clauses: &[ConstitutionalClause]) -> Hash {
    let mut out = [0u8; 32];
    for clause in clauses {
        for i in 0..32 {
            out[i] ^= clause.clause_id[i] ^ clause.execution_root[i] ^ clause.lineage_root[i];
        }
    }
    out
}
