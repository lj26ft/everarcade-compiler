use super::stream::WebSocketObserverStream;
#[derive(Debug, Clone)]
pub struct WebSocketObserverSession {
    pub observer_id: String,
    pub stream: WebSocketObserverStream,
    pub reconstruction_only: bool,
}
impl WebSocketObserverSession {
    pub fn new(observer_id: impl Into<String>) -> Self {
        Self {
            observer_id: observer_id.into(),
            stream: WebSocketObserverStream::new(),
            reconstruction_only: true,
        }
    }
    pub fn reject_authority_mutation(&self) -> Result<(), String> {
        if self.reconstruction_only {
            Ok(())
        } else {
            Err("observer_originated_authority_mutation_rejected".into())
        }
    }
}
