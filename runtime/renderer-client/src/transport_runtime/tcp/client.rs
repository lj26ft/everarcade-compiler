use crate::transport_runtime::peer_handshake::ReplayPeerHandshake;
use crate::transport_runtime::tcp::protocol::TcpReplayFrame;
use crate::transport_runtime::tcp::session::TcpReplaySession;
use crate::transport_runtime::wire::ReplayChunkWireMessage;
use std::io::BufReader;
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

pub struct TcpReplayClient {
    pub local: ReplayPeerHandshake,
    pub remote: ReplayPeerHandshake,
    stream: TcpStream,
    reader: BufReader<TcpStream>,
}

impl TcpReplayClient {
    pub fn connect(addr: SocketAddr, local: ReplayPeerHandshake) -> Result<Self, String> {
        let mut stream = TcpStream::connect(addr).map_err(|e| e.to_string())?;
        stream
            .set_read_timeout(Some(Duration::from_secs(2)))
            .map_err(|e| e.to_string())?;
        TcpReplaySession::send_frame(&mut stream, &TcpReplayFrame::Handshake(local.clone()))?;
        let mut reader = BufReader::new(stream.try_clone().map_err(|e| e.to_string())?);
        let remote = match TcpReplaySession::read_frame(&mut reader)? {
            TcpReplayFrame::Handshake(h) => h,
            _ => return Err("tcp_handshake_expected".into()),
        };
        remote.validate_against(&local.protocol_version, &local.continuity_root)?;
        Ok(Self {
            local,
            remote,
            stream,
            reader,
        })
    }
    pub fn send_chunk(&mut self, chunk: ReplayChunkWireMessage) -> Result<u64, String> {
        chunk.validate()?;
        TcpReplaySession::send_frame(&mut self.stream, &TcpReplayFrame::Chunk(chunk))?;
        match TcpReplaySession::read_frame(&mut self.reader)? {
            TcpReplayFrame::Ack(ack) if ack.accepted && ack.replay_only => {
                Ok(ack.acknowledged_sequence)
            }
            _ => Err("tcp_ack_expected".into()),
        }
    }
}
