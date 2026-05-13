use super::{amendment::{ConstitutionalAmendment, Hash}, amendment_lineage::derive_amendment_lineage};
pub fn transition_amendment(prior_constitution_root: Hash, next_constitution_root: Hash) -> ConstitutionalAmendment {
    let amendment_lineage_root = derive_amendment_lineage(prior_constitution_root, next_constitution_root);
    ConstitutionalAmendment { amendment_id: amendment_lineage_root, prior_constitution_root, next_constitution_root, amendment_lineage_root }
}
