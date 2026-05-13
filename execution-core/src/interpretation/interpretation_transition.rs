use super::{interpretation::{ConstitutionalInterpretation, Hash}, interpretation_lineage::interpretation_lineage};
pub fn transition_interpretation(prior: &ConstitutionalInterpretation, next_scope_root: Hash) -> ConstitutionalInterpretation {
    let lineage_root = interpretation_lineage(prior.lineage_root, next_scope_root);
    ConstitutionalInterpretation { interpretation_id: lineage_root, constitutional_root: prior.constitutional_root, interpretation_scope_root: next_scope_root, lineage_root }
}
