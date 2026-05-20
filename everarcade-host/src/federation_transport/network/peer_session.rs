#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PeerSession {
    pub peer: String,
    pub topology_epoch: u64,
    pub lease_id: String,
    pub active: bool,
}

pub fn establish_peer_session(peer: String, topology_epoch: u64, lease_id: String) -> PeerSession {
    PeerSession {
        peer,
        topology_epoch,
        lease_id,
        active: true,
    }
}

pub fn verify_peer_session(
    session: &PeerSession,
    expected_epoch: u64,
    expected_lease: &str,
) -> bool {
    session.active && session.topology_epoch == expected_epoch && session.lease_id == expected_lease
}

pub fn resume_peer_session(session: &mut PeerSession) {
    session.active = true;
}

pub fn terminate_peer_session(session: &mut PeerSession) {
    session.active = false;
}
