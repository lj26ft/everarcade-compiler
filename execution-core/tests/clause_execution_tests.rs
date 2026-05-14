use execution_core::constitution::{
    clause::ConstitutionalClause, clause_execution::execute_clause,
};

#[test]
fn clause_replay_validity() {
    let clause = ConstitutionalClause {
        clause_id: [1; 32],
        constitutional_scope_root: [2; 32],
        execution_root: [3; 32],
        lineage_root: [4; 32],
    };
    assert_eq!(
        execute_clause(&clause, [9; 32]),
        execute_clause(&clause, [9; 32])
    );
}
