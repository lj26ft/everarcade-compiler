use crate::runtime::RenderProjectionFrame;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackMode { Playing, Paused }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaybackRenderer { pub mode: PlaybackMode, pub index: usize }

impl Default for PlaybackRenderer { fn default() -> Self { Self { mode: PlaybackMode::Paused, index: 0 } } }

impl PlaybackRenderer {
    pub fn step(&mut self, frames: &[RenderProjectionFrame]) -> Option<RenderProjectionFrame> { let f = frames.get(self.index).cloned(); if f.is_some() { self.index += 1; } f }
    pub fn rewind(&mut self) { self.index = 0; self.mode = PlaybackMode::Paused; }
}
