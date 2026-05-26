use super::window::ProjectionFederationWindow;

pub fn window_equivalent(a: &ProjectionFederationWindow, b: &ProjectionFederationWindow) -> bool {
    a.start_frame == b.start_frame
        && a.end_frame == b.end_frame
        && a.continuity_root == b.continuity_root
        && a.artifact_root == b.artifact_root
}
