use everarcade_host::session_security::{
    authenticated_session::AuthenticatedSession, session_root::derive_session_root,
    session_validation::session_valid,
};
#[test]
fn session_validates() {
    let root = derive_session_root([1; 32], [2; 32], Some([3; 32]));
    let s = AuthenticatedSession {
        session_root: root,
        local_peer_root: [1; 32],
        remote_peer_root: [2; 32],
        federation_scope_root: Some([3; 32]),
    };
    assert!(session_valid(&s));
}
