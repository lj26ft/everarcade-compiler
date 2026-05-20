use super::{
    bundle::ContinuityBundle, checkpoint_sync::sync_checkpoint, reconciliation::reconcile_peer,
};

pub fn synchronize(local: &ContinuityBundle, peer: &ContinuityBundle) -> Option<ContinuityBundle> {
    if sync_checkpoint(local, peer) {
        Some(local.clone())
    } else {
        reconcile_peer(local, peer)
    }
}
