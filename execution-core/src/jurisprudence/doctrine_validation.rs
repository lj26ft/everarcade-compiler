use super::precedent::LegalPrecedent;
pub fn validate_doctrine(precedent: &LegalPrecedent) -> bool {
    precedent.precedent_id != [0u8; 32] && precedent.lineage_root != [0u8; 32]
}
