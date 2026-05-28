use crate::transport_runtime::peer_handshake::ReplayPeerHandshake;
use crate::transport_runtime::wire::{
    ReplayAckWireMessage, ReplayChunkWireMessage, ReplayTransportError,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TcpReplayFrame {
    Handshake(ReplayPeerHandshake),
    Chunk(ReplayChunkWireMessage),
    Ack(ReplayAckWireMessage),
    Error(ReplayTransportError),
}

pub fn encode_frame(frame: &TcpReplayFrame) -> Result<Vec<u8>, String> {
    let mut bytes = serde_json::to_vec(frame).map_err(|e| e.to_string())?;
    bytes.push(b'\n');
    Ok(bytes)
}

pub fn decode_frame(line: &str) -> Result<TcpReplayFrame, String> {
    serde_json::from_str(line.trim_end()).map_err(|e| format!("malformed_tcp_replay_frame:{e}"))
}
