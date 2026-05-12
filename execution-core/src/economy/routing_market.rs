pub fn routing_cost(hops: u64, packets: u64) -> u64 {
    hops.saturating_mul(packets).saturating_mul(2)
}
