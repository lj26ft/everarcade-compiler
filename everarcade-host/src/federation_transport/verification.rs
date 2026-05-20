use execution_core::federation_runtime::bundle::ContinuityBundle;

pub fn verify_peer_checkpoint(local: &ContinuityBundle, peer: &ContinuityBundle) -> bool {
    local.checkpoint_root() == peer.checkpoint_root()
}

trait CheckpointRoot {
    fn checkpoint_root(&self) -> [u8; 32];
}

impl CheckpointRoot for ContinuityBundle {
    fn checkpoint_root(&self) -> [u8; 32] {
        self.checkpoint_hash
    }
}
