use super::{
    clause::{ConstitutionalClause, Hash},
    clause_lineage::derive_lineage_root,
};
pub fn amend_clause(
    prior: &ConstitutionalClause,
    next_execution_root: Hash,
) -> ConstitutionalClause {
    ConstitutionalClause {
        clause_id: prior.clause_id,
        constitutional_scope_root: prior.constitutional_scope_root,
        execution_root: next_execution_root,
        lineage_root: derive_lineage_root(prior.lineage_root, next_execution_root),
    }
}
