use super::stream::ReplayTransportStream;

pub fn reject_chunk_injection(stream: &ReplayTransportStream, sequence: u64) -> bool {
    sequence >= stream.cursor.next_sequence
}

pub fn reject_reordering(expected: u64, observed: u64) -> bool {
    expected == observed
}
