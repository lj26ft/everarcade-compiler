use super::stream::ReplayTransportStream;

#[derive(Debug, Clone)]
pub struct ReplayTransportSession {
    pub session_id: String,
    pub stream: ReplayTransportStream,
}

impl ReplayTransportSession {
    pub fn new(session_id: impl Into<String>) -> Self {
        Self { session_id: session_id.into(), stream: ReplayTransportStream::default() }
    }
}
