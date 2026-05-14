use super::{
    clause::{ConstitutionalClause, Hash},
    clause_validation::validate_clause,
};
pub fn execute_clause(clause: &ConstitutionalClause, input_root: Hash) -> Hash {
    if !validate_clause(clause) {
        return [0u8; 32];
    }
    let mut out = [0u8; 32];
    for i in 0..32 {
        out[i] = clause.execution_root[i] ^ input_root[i] ^ clause.clause_id[i];
    }
    out
}
