use everarcade_host::network::message_framing::{deframe_payload, frame_message, PeerMessage};

#[test]
fn frames_roundtrip() {
    let msg = PeerMessage {
        message_id: [1; 32],
        message_type: "x".into(),
        payload_root: [2; 32],
        payload_bytes: vec![1, 2, 3],
    };
    let framed = frame_message(&msg);
    assert_eq!(deframe_payload(&framed), Some(vec![1, 2, 3]));
}
