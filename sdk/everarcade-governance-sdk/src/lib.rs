pub trait GovernanceHooks {
    fn on_governance_event(&mut self, _event_id: &str) {}
    fn on_governance_transition(&mut self, _from: &str, _to: &str) {}
}
