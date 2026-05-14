use super::amendment::ConstitutionalAmendment;
pub fn validate_amendment(amendment: &ConstitutionalAmendment) -> bool {
    amendment.prior_constitution_root != amendment.next_constitution_root
        && amendment.amendment_lineage_root != [0u8; 32]
}
