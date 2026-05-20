use super::{bundle::ContinuityBundle, divergence::verify_peer_replay};

pub fn verify_bundle_replay(local: &ContinuityBundle, peer: &ContinuityBundle) -> bool {
    verify_peer_replay(local, peer)
}
