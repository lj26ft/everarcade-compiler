#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionDemand {
    pub package_id: String,
    pub required_capacity: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionSupply {
    pub node_id: String,
    pub offered_capacity: u64,
}

#[derive(Debug, Default)]
pub struct ExecutionMarket;

impl ExecutionMarket {
    pub fn match_supply(
        demand: &ExecutionDemand,
        supplies: &[ExecutionSupply],
    ) -> Option<ExecutionSupply> {
        let mut valid: Vec<&ExecutionSupply> = supplies
            .iter()
            .filter(|s| s.offered_capacity >= demand.required_capacity)
            .collect();
        valid.sort_by(|a, b| {
            a.offered_capacity
                .cmp(&b.offered_capacity)
                .then(a.node_id.cmp(&b.node_id))
        });
        valid.first().cloned().cloned()
    }
}
