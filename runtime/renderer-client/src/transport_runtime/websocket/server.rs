use super::session::WebSocketObserverSession;
use crate::transport_runtime::wire::ReplayWindowWireMessage;
#[derive(Debug, Clone, Default)]
pub struct WebSocketObserverServer {
    pub sessions: Vec<WebSocketObserverSession>,
    pub active: bool,
}
impl WebSocketObserverServer {
    pub fn start_loopback() -> Self {
        Self {
            sessions: vec![],
            active: true,
        }
    }
    pub fn attach(&mut self, observer_id: impl Into<String>) {
        self.sessions
            .push(WebSocketObserverSession::new(observer_id));
    }
    pub fn broadcast_window(&mut self, window: ReplayWindowWireMessage) -> Result<(), String> {
        for session in &mut self.sessions {
            session.stream.ingest_window(window.clone())?;
        }
        Ok(())
    }
}
