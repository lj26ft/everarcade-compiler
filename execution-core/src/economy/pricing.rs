#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PricingInput {
    pub fuel_units: u64,
    pub proof_units: u64,
    pub archival_bytes: u64,
    pub routing_packets: u64,
    pub settlement_ops: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PricingVector {
    pub fuel_price: u64,
    pub proof_price: u64,
    pub archival_price: u64,
    pub routing_price: u64,
    pub settlement_price: u64,
    pub total: u64,
}

pub fn price(input: PricingInput) -> PricingVector {
    let fuel_price = input.fuel_units.saturating_mul(2);
    let proof_price = input.proof_units.saturating_mul(7);
    let archival_price = input.archival_bytes / 1024;
    let routing_price = input.routing_packets.saturating_mul(3);
    let settlement_price = input.settlement_ops.saturating_mul(11);
    let total = fuel_price
        .saturating_add(proof_price)
        .saturating_add(archival_price)
        .saturating_add(routing_price)
        .saturating_add(settlement_price);

    PricingVector {
        fuel_price,
        proof_price,
        archival_price,
        routing_price,
        settlement_price,
        total,
    }
}
