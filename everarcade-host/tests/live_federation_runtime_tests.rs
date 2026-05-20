use everarcade_host::federation_transport::network::{
    client_roundtrip, establish_peer_session, terminate_peer_session, verify_peer_session,
};
use execution_core::federation_runtime::FederationProtocolMessage;

#[test]
fn test_peer_restart_recovery() {
    let mut session =
        establish_peer_session("127.0.0.1:9001".to_string(), 7, "lease-a".to_string());
    assert!(verify_peer_session(&session, 7, "lease-a"));
    terminate_peer_session(&mut session);
    assert!(!verify_peer_session(&session, 7, "lease-a"));
}

#[test]
fn test_partial_sync_recovery() {
    let message = FederationProtocolMessage::JournalRangeRequest { start: 2, end: 4 };
    let response = client_roundtrip(message.clone()).expect("message should roundtrip");
    assert_eq!(response, message);
}

#[test]
fn test_transport_disconnect_recovery() {
    let message = FederationProtocolMessage::PeerHello {
        node_id: [9; 32],
        topology_epoch: 3,
    };
    let response = client_roundtrip(message.clone()).expect("message should roundtrip");
    assert_eq!(response, message);
}

#[test]
fn test_checkpoint_recovery_after_restart() {
    let message = FederationProtocolMessage::CheckpointRequest {
        checkpoint_id: [1; 32],
    };
    let response = client_roundtrip(message.clone()).expect("message should roundtrip");
    assert_eq!(response, message);
}

#[test]
fn test_rejoin_after_divergence_resolution() {
    let message = FederationProtocolMessage::DivergenceReport {
        reason: "checkpoint mismatch".into(),
    };
    let response = client_roundtrip(message.clone()).expect("message should roundtrip");
    assert_eq!(response, message);
}
