use crate::transport_runtime::wire::ReplayWindowWireMessage;
#[derive(Debug, Clone, Default)]
pub struct WebSocketObserverStream {
    pub windows: Vec<ReplayWindowWireMessage>,
    pub cursor: u64,
    pub non_authoritative: bool,
}
impl WebSocketObserverStream {
    pub fn new() -> Self {
        Self {
            non_authoritative: true,
            ..Self::default()
        }
    }
    pub fn ingest_window(&mut self, window: ReplayWindowWireMessage) -> Result<(), String> {
        window.validate()?;
        if !self.non_authoritative {
            return Err("observer_authority_mutation_rejected".into());
        }
        if window.start_sequence != self.cursor {
            return Err("observer_order_rejected".into());
        }
        self.cursor = window.end_sequence + 1;
        self.windows.push(window);
        Ok(())
    }
}
