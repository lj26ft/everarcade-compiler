use super::precedent::LegalPrecedent;
pub fn precedent_chain_is_continuous(precedents: &[LegalPrecedent]) -> bool {
    precedents
        .windows(2)
        .all(|w| w[0].lineage_root != w[1].lineage_root)
}
