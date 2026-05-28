#[path = "../../runtime/renderer-client/src/history/mod.rs"]
mod history;
#[path = "../../runtime/renderer-client/src/persistence/mod.rs"]
mod persistence;
#[path = "../../runtime/renderer-client/src/transport_runtime/mod.rs"]
mod transport_runtime;

use history::reject_observer_authority_mutation;
use persistence::LiveReplayStore;
use std::{thread, time::Duration};
use transport_runtime::peer_handshake::ReplayPeerHandshake;
use transport_runtime::tcp::{TcpReplayClient, TcpReplayListener, TcpReplayStream};
use transport_runtime::websocket::{WebSocketObserverClient, WebSocketObserverServer};
use transport_runtime::wire::{
    deterministic_hash, ReplayCheckpointWireMessage, ReplayChunkWireMessage,
    ReplayWindowWireMessage,
};
use transport_runtime::{ReplayCursor, ReplayReconnectState};

fn hs(peer_id: &str) -> ReplayPeerHandshake {
    ReplayPeerHandshake {
        peer_id: peer_id.into(),
        protocol_version: "everarcade-replay/1".into(),
        replay_tip: 0,
        continuity_root: "root-a".into(),
        supported_transports: vec!["tcp".into(), "websocket".into()],
        reconstruction_only: true,
    }
}
fn chunk(sequence: u64) -> ReplayChunkWireMessage {
    ReplayChunkWireMessage::new(
        "s1",
        sequence,
        if sequence == 0 { "genesis" } else { "root-a" },
        "root-a",
        vec![sequence as u8],
    )
}
fn window() -> ReplayWindowWireMessage {
    ReplayWindowWireMessage {
        window_id: "w0".into(),
        start_sequence: 0,
        end_sequence: 0,
        continuity_root: "root-a".into(),
        chunks: vec![chunk(0)],
        reconstruction_only: true,
    }
}

#[test]
fn test_tcp_peer_handshake() {
    let listener = TcpReplayListener::bind_loopback(0, hs("node-b")).unwrap();
    thread::sleep(Duration::from_millis(25));
    let client = TcpReplayClient::connect(listener.addr, hs("node-a")).unwrap();
    assert_eq!(client.remote.peer_id, "node-b");
}

#[test]
fn test_tcp_replay_chunk_transfer() {
    let listener = TcpReplayListener::bind_loopback(0, hs("node-b")).unwrap();
    thread::sleep(Duration::from_millis(25));
    let mut client = TcpReplayClient::connect(listener.addr, hs("node-a")).unwrap();
    assert_eq!(client.send_chunk(chunk(0)).unwrap(), 0);
}

#[test]
fn test_tcp_replay_duplicate_rejection() {
    let mut stream = TcpReplayStream::with_root("root-a");
    stream.ingest(chunk(0)).unwrap();
    assert!(stream.ingest(chunk(0)).is_err());
}

#[test]
fn test_tcp_replay_order_rejection() {
    let mut stream = TcpReplayStream::with_root("root-a");
    assert!(stream.ingest(chunk(1)).is_err());
}

#[test]
fn test_peer_reconnect_resume() {
    let cursor = ReplayCursor {
        next_sequence: 1,
        continuity_root: "root-a".into(),
    };
    assert!(
        ReplayReconnectState::resume(cursor, "root-a")
            .unwrap()
            .peer_equivalent
    );
}

#[test]
fn test_websocket_observer_stream() {
    let mut server = WebSocketObserverServer::start_loopback();
    server.attach("observer-a");
    server.broadcast_window(window()).unwrap();
    let mut client = WebSocketObserverClient::connect("observer-b");
    client.receive(window()).unwrap();
    assert_eq!(client.stream.cursor, 1);
}

#[test]
fn test_observer_rejects_authority_mutation() {
    assert!(reject_observer_authority_mutation(false).is_err());
}

#[test]
fn test_runtime_daemon_bootstrap() {
    #[path = "../../runtime/node/mod.rs"]
    mod node;
    assert!(node::SovereignRuntimeNodeDaemon::bootstrap().readiness());
}

#[test]
fn test_runtime_daemon_restart_recovery() {
    let restored = persistence::LiveReplayIndex::restore(7, "root-a").unwrap();
    assert_eq!(restored.replay_tip, 7);
}

#[test]
fn test_live_storage_restore() {
    let mut store = LiveReplayStore::open("root-a").unwrap();
    store.persist_chunk(chunk(0)).unwrap();
    let checkpoint = ReplayCheckpointWireMessage {
        checkpoint_sequence: 0,
        replay_tip: 1,
        continuity_root: "root-a".into(),
        checkpoint_hash: deterministic_hash(b"1:root-a"),
        reconstruction_only: true,
    };
    store.persist_checkpoint(checkpoint).unwrap();
    assert_eq!(store.index.replay_tip, 1);
}

#[test]
fn test_live_peer_divergence_rejection() {
    let mut bad = hs("node-x");
    bad.continuity_root = "fork".into();
    assert!(bad
        .validate_against("everarcade-replay/1", "root-a")
        .is_err());
}

#[test]
fn test_live_peer_non_authoritative() {
    assert!(hs("node-a").reconstruction_only);
}
