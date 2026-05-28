#![allow(dead_code)]

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PeerIdentity {
    pub deterministic: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayPeerIdentity {
    pub peer_id: String,
    pub verified: bool,
}

impl ReplayPeerIdentity {
    pub fn verify(peer_id: impl Into<String>) -> Result<Self, String> {
        let peer_id = peer_id.into();
        if peer_id.trim().is_empty() {
            Err("peer_identity_rejected".into())
        } else {
            Ok(Self {
                peer_id,
                verified: true,
            })
        }
    }
}
