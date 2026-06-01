use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FederationTopology {
    pub nodes: Vec<String>,
    pub leader: String,
    pub observers: Vec<String>,
    pub checkpoint_owners: Vec<String>,
    pub recovery_candidates: Vec<String>,
    pub health_score: f64,
}
impl FederationTopology {
    pub fn supported_node_counts() -> [usize; 5] {
        [1, 2, 5, 10, 25]
    }
    pub fn new(node_count: usize) -> Result<Self, String> {
        if !Self::supported_node_counts().contains(&node_count) {
            return Err("unsupported topology size".into());
        }
        let nodes: Vec<String> = (1..=node_count).map(|n| format!("node-{n}")).collect();
        Ok(Self {
            leader: nodes[0].clone(),
            observers: nodes.iter().skip(1).cloned().collect(),
            checkpoint_owners: nodes.iter().take(node_count.min(3)).cloned().collect(),
            recovery_candidates: nodes
                .iter()
                .rev()
                .take(node_count.saturating_sub(1).min(3))
                .cloned()
                .collect(),
            health_score: 1.0,
            nodes,
        })
    }
    pub fn federation_health(&self) -> &'static str {
        if self.health_score >= 0.90 {
            "healthy"
        } else if self.health_score >= 0.60 {
            "warning"
        } else {
            "critical"
        }
    }
}
