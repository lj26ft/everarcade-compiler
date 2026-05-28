use super::stream::WebSocketObserverStream;
use crate::transport_runtime::wire::ReplayWindowWireMessage;
#[derive(Debug, Clone, Default)]
pub struct WebSocketObserverClient {
    pub observer_id: String,
    pub stream: WebSocketObserverStream,
}
impl WebSocketObserverClient {
    pub fn connect(observer_id: impl Into<String>) -> Self {
        Self {
            observer_id: observer_id.into(),
            stream: WebSocketObserverStream::new(),
        }
    }
    pub fn receive(&mut self, window: ReplayWindowWireMessage) -> Result<(), String> {
        self.stream.ingest_window(window)
    }
}
