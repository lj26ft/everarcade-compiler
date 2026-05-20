use everarcade_host::federation_transport::network::*;

#[test]
fn test_session_lifecycle_and_recovery() {
    let mut session = start_peer_session("127.0.0.1:9222".into(), 1, "lease-a".into());
    assert!(verify_peer_session(&session, 1, "lease-a"));

    maintain_peer_session(&mut session);
    recover_interrupted_sync(&mut session, 10, 20);
    assert!(resume_missing_range_sync(&mut session, 10, 20));

    recover_peer_session(&mut session, 7);
    assert!(verify_incremental_checkpoint(&session, 7));

    resume_peer_session(&mut session);
    shutdown_peer_session(&mut session);
    assert!(!session.active);
}

#[test]
fn test_incremental_continuity_and_safety() {
    let mut session = start_peer_session("127.0.0.1:9223".into(), 1, "lease-b".into());
    let ok =
        advance_incremental_continuity(&mut session, [0u8; 32], [1u8; 32], [2u8; 32], [3u8; 32]);
    assert!(ok);
    assert!(verify_incremental_checkpoint(&session, 1));

    assert!(validate_continuity_advancement(
        true, true, true, true, true, true
    ));
    assert!(verify_checkpoint_lineage([9u8; 32], [9u8; 32]));
    assert!(verify_execution_continuity([8u8; 32], [8u8; 32]));
    assert!(validate_recovery_state(false, false));
    assert!(verify_incremental_replay(true, true));
    assert!(reject_invalid_advancement(true, false));
}

#[test]
fn test_framing_and_codec_reject_bad_hash_or_version() {
    let payload = br#"{\"version\":1,\"message\":{\"Ping\":{\"nonce\":1}}}"#;
    let frame = encode_frame(42, payload).unwrap();
    let (msg_type, decoded) = decode_frame(&frame).unwrap();
    assert_eq!(msg_type, 42);
    assert_eq!(decoded, payload);

    let mut tampered = frame.clone();
    let last = tampered.len() - 1;
    tampered[last] ^= 0xFF;
    assert!(decode_frame(&tampered).is_none());

    let mut wrong_version = frame;
    wrong_version[1] = 2;
    assert!(decode_frame(&wrong_version).is_none());
}
