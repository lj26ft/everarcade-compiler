pub fn render_projection(projection_hash: &str, camera_hash: &str) -> String { crate::stable_hash(&["render-projection-only", projection_hash, camera_hash]) }
pub fn renderer_is_authoritative() -> bool { false }
