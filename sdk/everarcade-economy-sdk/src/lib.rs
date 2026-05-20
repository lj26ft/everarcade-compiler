pub trait EconomyHooks {
    fn on_settlement(&mut self, _tx_id: &str, _amount_drops: u64) {}
    fn on_settlement_event(&mut self, _event: &str) {}
}
