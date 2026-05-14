use everarcade_host::network::message_framing::{deframe_payload, frame_message, PeerMessage};

#[test]
fn framed_peer_payload_roundtrips() {
    let message = PeerMessage {
        message_id: [1; 32],
        message_type: "receipt".to_string(),
        payload_root: [2; 32],
        payload_bytes: vec![9, 8, 7],
    };
    let framed = frame_message(&message);
    assert_eq!(deframe_payload(&framed), Some(vec![9, 8, 7]));
}
