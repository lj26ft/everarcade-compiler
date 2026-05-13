use execution_core::{amendment::amendment_transition::transition_amendment, constitution::clause::ConstitutionalClause};

#[test]
fn executable_constitution_end_to_end() {
    let clause = ConstitutionalClause { clause_id:[1;32], constitutional_scope_root:[2;32], execution_root:[3;32], lineage_root:[4;32] };
    let amendment = transition_amendment(clause.execution_root, [5;32]);
    assert_ne!(amendment.prior_constitution_root, amendment.next_constitution_root);
}
