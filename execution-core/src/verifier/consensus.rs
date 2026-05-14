use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct ConsensusOutcome {
    pub receipt_hash: String,
    pub agreeing_nodes: Vec<String>,
}

pub struct ReceiptConsensus;

impl ReceiptConsensus {
    pub fn evaluate(receipts_by_node: &[(String, String)]) -> Option<ConsensusOutcome> {
        let mut buckets: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for (node_id, receipt_hash) in receipts_by_node {
            buckets
                .entry(receipt_hash.clone())
                .or_default()
                .push(node_id.clone());
        }
        buckets
            .into_iter()
            .max_by_key(|(_, nodes)| nodes.len())
            .map(|(receipt_hash, agreeing_nodes)| ConsensusOutcome {
                receipt_hash,
                agreeing_nodes,
            })
    }
}
