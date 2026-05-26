use super::{equivalence::ReplayEquivalenceRuntime, stream::ReplayTransportStream};

#[derive(Debug, Clone, Default)]
pub struct ObserverReplayRuntime;

#[derive(Debug, Clone)]
pub struct ObserverReplaySession {
    pub observer_id: String,
    pub state: ObserverReplayState,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ObserverReplayWindow {
    pub start_sequence: u64,
    pub end_sequence: u64,
    pub continuity_root: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ObserverReplayState {
    pub applied_chunks: u64,
    pub last_continuity_hash: String,
}

impl ObserverReplayRuntime {
    pub fn restore(
        observer_id: impl Into<String>,
        stream: &ReplayTransportStream,
    ) -> ObserverReplaySession {
        let observer_id = observer_id.into();
        let last_hash = stream.cursor.last_continuity_hash.clone();
        ObserverReplaySession {
            observer_id,
            state: ObserverReplayState {
                applied_chunks: stream.accepted.len() as u64,
                last_continuity_hash: last_hash,
            },
        }
    }

    pub fn validate_equivalence(
        source: &ReplayTransportStream,
        observer: &ReplayTransportStream,
    ) -> bool {
        ReplayEquivalenceRuntime::compare_streams(source, observer).equivalent
    }
}
