use crate::network::node::NetworkNode;

#[derive(Debug, Clone, Default)]
pub struct NodeDiscovery {
    registry: Vec<NetworkNode>,
}

impl NodeDiscovery {
    pub fn new(registry: Vec<NetworkNode>) -> Self {
        Self { registry }
    }

    pub fn discover_for_epoch(&self, epoch: u64) -> Vec<NetworkNode> {
        self.registry
            .iter()
            .filter(|n| n.supports_epoch(epoch))
            .cloned()
            .collect()
    }
}
