use super::clause::ConstitutionalClause;
pub fn validate_clause(clause: &ConstitutionalClause) -> bool {
    clause.clause_id != [0u8; 32] && clause.constitutional_scope_root != [0u8; 32] && clause.execution_root != [0u8; 32] && clause.lineage_root != [0u8; 32]
}
