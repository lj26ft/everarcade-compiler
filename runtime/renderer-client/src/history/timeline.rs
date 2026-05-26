#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalReplayEraWindow { pub era_id: String, pub start_frame: u64, pub end_frame: u64 }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalReplayEraManifest { pub era_id: String, pub continuity_root: String, pub frame_count: u64 }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalReplayEra { pub manifest: HistoricalReplayEraManifest, pub windows: Vec<HistoricalReplayEraWindow> }
#[derive(Debug, Default, Clone)]
pub struct HistoricalReplayTimeline { pub eras: Vec<HistoricalReplayEra> }
impl HistoricalReplayTimeline {
    pub fn append_era(&mut self, era: HistoricalReplayEra) -> Result<(), String> { if self.eras.iter().any(|e| e.manifest.era_id==era.manifest.era_id){return Err("duplicate_era".into())} self.eras.push(era); Ok(()) }
    pub fn restore_window(&self, era_id: &str, start_frame: u64, end_frame: u64) -> Option<HistoricalReplayEraWindow> { self.eras.iter().find(|e| e.manifest.era_id==era_id).and_then(|e| e.windows.iter().find(|w| w.start_frame==start_frame && w.end_frame==end_frame)).cloned() }
}
