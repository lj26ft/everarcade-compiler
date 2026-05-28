use crate::transport_runtime::peer_handshake::ReplayPeerHandshake;
use crate::transport_runtime::tcp::protocol::{decode_frame, encode_frame, TcpReplayFrame};
use crate::transport_runtime::tcp::stream::TcpReplayStream;
use crate::transport_runtime::wire::{ReplayAckWireMessage, ReplayChunkWireMessage};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

pub struct TcpReplaySession {
    pub peer: ReplayPeerHandshake,
    pub replay_stream: TcpReplayStream,
}

impl TcpReplaySession {
    pub fn send_frame(stream: &mut TcpStream, frame: &TcpReplayFrame) -> Result<(), String> {
        stream
            .write_all(&encode_frame(frame)?)
            .map_err(|e| e.to_string())
    }
    pub fn read_frame(reader: &mut BufReader<TcpStream>) -> Result<TcpReplayFrame, String> {
        let mut line = String::new();
        reader.read_line(&mut line).map_err(|e| e.to_string())?;
        if line.is_empty() {
            return Err("tcp_replay_session_closed".into());
        }
        decode_frame(&line)
    }
    pub fn ack(peer_id: impl Into<String>, chunk: &ReplayChunkWireMessage) -> ReplayAckWireMessage {
        ReplayAckWireMessage {
            peer_id: peer_id.into(),
            acknowledged_sequence: chunk.sequence,
            continuity_root: chunk.continuity_root.clone(),
            accepted: true,
            replay_only: true,
        }
    }
}
