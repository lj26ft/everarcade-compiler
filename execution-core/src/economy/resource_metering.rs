#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceUsage {
    pub compute: u64,
    pub proof: u64,
    pub storage: u64,
    pub routing: u64,
    pub bandwidth: u64,
}

pub fn canonical_meter(usage: ResourceUsage) -> u64 {
    usage.compute
        .saturating_mul(2)
        .saturating_add(usage.proof.saturating_mul(4))
        .saturating_add(usage.storage / 1024)
        .saturating_add(usage.routing.saturating_mul(2))
        .saturating_add(usage.bandwidth / 256)
}
