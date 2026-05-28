use super::observer_stream::LiveObserverReplayStream;
#[derive(Debug, Clone, Default)]
pub struct LiveReplayObserver {
    pub observer_id: String,
    pub stream: LiveObserverReplayStream,
    pub non_authoritative: bool,
}
impl LiveReplayObserver {
    pub fn new(observer_id: impl Into<String>) -> Self {
        Self {
            observer_id: observer_id.into(),
            stream: LiveObserverReplayStream::default(),
            non_authoritative: true,
        }
    }
}
