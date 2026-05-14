use super::pricing::{price, PricingInput, PricingVector};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionOffer {
    pub executor_id: String,
    pub capacity: u64,
    pub latency_hint_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionBid {
    pub workload: PricingInput,
}

pub fn match_offer(
    offers: &[ExecutionOffer],
    bid: &ExecutionBid,
) -> Option<(ExecutionOffer, PricingVector)> {
    let mut eligible: Vec<_> = offers
        .iter()
        .filter(|o| o.capacity >= bid.workload.fuel_units)
        .cloned()
        .collect();
    eligible.sort_by_key(|o| (o.latency_hint_ms, o.executor_id.clone()));
    eligible.into_iter().next().map(|offer| {
        let priced = price(bid.workload);
        (offer, priced)
    })
}
