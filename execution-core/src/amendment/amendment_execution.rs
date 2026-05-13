use super::{amendment::{ConstitutionalAmendment, Hash}, amendment_validation::validate_amendment};
pub fn execute_amendment(amendment: &ConstitutionalAmendment) -> Hash {
    if validate_amendment(amendment) { amendment.next_constitution_root } else { amendment.prior_constitution_root }
}
