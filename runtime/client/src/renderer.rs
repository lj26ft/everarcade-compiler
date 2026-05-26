use execution_core::render_bridge::{stream::ProjectionWindow, RenderFrameEnvelope};

use crate::{
    event_view::render_event_stream,
    inventory_view::render_inventory_view,
    world_view::{render_world_ascii, WorldViewConfig},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderViewport {
    pub width: i64,
    pub height: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderFrameState {
    pub tick: u64,
    pub projection_root: String,
    pub frame_text: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RenderProjectionSession {
    pub rendered: Vec<RenderFrameState>,
}

#[derive(Debug, Clone)]
pub struct TerminalRenderer {
    pub viewport: RenderViewport,
}

impl Default for TerminalRenderer {
    fn default() -> Self {
        Self {
            viewport: RenderViewport {
                width: 16,
                height: 8,
            },
        }
    }
}

impl TerminalRenderer {
    pub fn render_frame(&self, frame: &RenderFrameEnvelope) -> String {
        let projected = execution_core::render_bridge::stream::materialize_projected_state(frame);
        let world = render_world_ascii(
            &projected,
            &WorldViewConfig {
                width: self.viewport.width,
                height: self.viewport.height,
            },
        );
        let events = render_event_stream(&projected.events);
        let inventory = render_inventory_view(&projected.inventory);
        format!(
            "{world}events:\n{events}\ninventory:\n{inventory}\nprojection_root={} replay_root={}",
            frame.world.state_root, frame.replay.replay_root
        )
    }

    pub fn render_window(&self, window: &ProjectionWindow) -> RenderProjectionSession {
        let rendered = window
            .frames
            .iter()
            .map(|f| RenderFrameState {
                tick: f.world.tick,
                projection_root: f.world.state_root.clone(),
                frame_text: self.render_frame(f),
            })
            .collect();
        RenderProjectionSession { rendered }
    }
}
