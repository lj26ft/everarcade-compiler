use super::interpretation::ConstitutionalInterpretation;
pub fn validate_interpretation(i: &ConstitutionalInterpretation) -> bool {
    i.interpretation_id != [0u8; 32] && i.lineage_root != [0u8; 32]
}
