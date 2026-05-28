use crate::transport_runtime::peer_handshake::ReplayPeerHandshake;
use crate::transport_runtime::tcp::protocol::TcpReplayFrame;
use crate::transport_runtime::tcp::session::TcpReplaySession;
use crate::transport_runtime::tcp::stream::TcpReplayStream;
use std::io::BufReader;
use std::net::{SocketAddr, TcpListener};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct TcpReplayListener {
    pub addr: SocketAddr,
    pub accepted_chunks: Arc<Mutex<usize>>,
}

impl TcpReplayListener {
    pub fn bind_loopback(port: u16, expected: ReplayPeerHandshake) -> Result<Self, String> {
        let listener = TcpListener::bind(("127.0.0.1", port)).map_err(|e| e.to_string())?;
        let addr = listener.local_addr().map_err(|e| e.to_string())?;
        let accepted_chunks = Arc::new(Mutex::new(0usize));
        let accepted_for_thread = accepted_chunks.clone();
        thread::spawn(move || {
            if let Ok((stream, _)) = listener.accept() {
                let mut writer = match stream.try_clone() {
                    Ok(w) => w,
                    Err(_) => return,
                };
                let mut reader = BufReader::new(stream);
                let frame = match TcpReplaySession::read_frame(&mut reader) {
                    Ok(f) => f,
                    Err(_) => return,
                };
                let remote = match frame {
                    TcpReplayFrame::Handshake(h) => h,
                    _ => return,
                };
                if remote
                    .validate_against(&expected.protocol_version, &expected.continuity_root)
                    .is_err()
                {
                    return;
                }
                let _ = TcpReplaySession::send_frame(
                    &mut writer,
                    &TcpReplayFrame::Handshake(expected.clone()),
                );
                let mut replay = TcpReplayStream::with_root(expected.continuity_root.clone());
                while let Ok(frame) = TcpReplaySession::read_frame(&mut reader) {
                    if let TcpReplayFrame::Chunk(chunk) = frame {
                        if replay.ingest(chunk.clone()).is_ok() {
                            *accepted_for_thread.lock().unwrap() += 1;
                            let ack = TcpReplaySession::ack(expected.peer_id.clone(), &chunk);
                            let _ = TcpReplaySession::send_frame(
                                &mut writer,
                                &TcpReplayFrame::Ack(ack),
                            );
                        } else {
                            break;
                        }
                    }
                }
            }
        });
        Ok(Self {
            addr,
            accepted_chunks,
        })
    }
}
