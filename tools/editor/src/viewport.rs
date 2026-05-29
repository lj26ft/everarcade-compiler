#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ViewportFrame { pub label: String, pub tick: u64, pub renderer_authoritative: bool }

pub fn viewport_frame(label: &str, tick: u64) -> ViewportFrame {
    ViewportFrame { label: label.to_owned(), tick, renderer_authoritative: false }
}
