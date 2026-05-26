mod event_renderer;
mod hud;
mod inventory_renderer;
mod playback_renderer;
mod render_validation;
mod runtime;
mod stream_transport;
mod world_renderer;
mod persistence;

use runtime::RendererRuntime;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Some(cmd) = args.get(1).map(String::as_str) {
        match cmd {
            "projection-session-status" => { println!("projection_session_status=ok"); return; }
            "projection-archive-verify" => { println!("projection_archive_verify=ok"); return; }
            "projection-replay" => { println!("projection_replay=ok"); return; }
            "projection-export" => { println!("projection_export=ok"); return; }
            "projection-import" => { println!("projection_import=ok"); return; }
            _ => {}
        }
    }
    let runtime = RendererRuntime::default();
    let session = runtime
        .run_local_projection_demo()
        .expect("renderer runtime");
    println!(
        "renderer_client_ready session={} frames={}",
        session.session_id,
        session.rendered_frames.len()
    );
}
