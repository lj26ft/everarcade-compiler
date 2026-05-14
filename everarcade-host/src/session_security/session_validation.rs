pub type Hash = [u8; 32];
use super::{authenticated_session::AuthenticatedSession, session_root::derive_session_root};
pub fn session_valid(session: &AuthenticatedSession) -> bool {
    session.session_root
        == derive_session_root(
            session.local_peer_root,
            session.remote_peer_root,
            session.federation_scope_root,
        )
}
