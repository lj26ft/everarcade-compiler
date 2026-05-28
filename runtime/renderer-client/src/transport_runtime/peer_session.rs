use super::peer_handshake::ReplayPeerHandshake;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayPeerSession {
    pub local: ReplayPeerHandshake,
    pub remote: ReplayPeerHandshake,
    pub last_acknowledged: u64,
    pub active: bool,
}

impl ReplayPeerSession {
    pub fn establish(
        local: ReplayPeerHandshake,
        remote: ReplayPeerHandshake,
    ) -> Result<Self, String> {
        remote.validate_against(&local.protocol_version, &local.continuity_root)?;
        Ok(Self {
            local,
            remote,
            last_acknowledged: 0,
            active: true,
        })
    }
}
