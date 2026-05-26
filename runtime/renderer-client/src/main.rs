mod event_renderer;
mod hud;
mod inventory_renderer;
mod playback_renderer;
mod render_validation;
mod runtime;
mod stream_transport;
mod world_renderer;

use runtime::RendererRuntime;

fn main() {
    let runtime = RendererRuntime::default();
    let session = runtime.run_local_projection_demo().expect("renderer runtime");
    println!("renderer_client_ready session={} frames={}", session.session_id, session.rendered_frames.len());
}
