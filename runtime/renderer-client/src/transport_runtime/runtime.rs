use super::{chunk::ReplayChunk, session::ReplayTransportSession};

#[derive(Debug, Default)]
pub struct ReplayTransportRuntime;

impl ReplayTransportRuntime {
    pub fn ingest_chunk(session: &mut ReplayTransportSession, chunk: ReplayChunk) -> Result<(), String> {
        session.stream.ingest(chunk)
    }
}
