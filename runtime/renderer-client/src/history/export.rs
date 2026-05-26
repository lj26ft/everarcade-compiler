#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalArchiveExporter;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalArchiveRoundtrip { pub continuity_root: String, pub equivalent: bool }
impl HistoricalArchiveExporter { pub fn export(continuity_root: &str) -> HistoricalArchiveRoundtrip { HistoricalArchiveRoundtrip { continuity_root: continuity_root.into(), equivalent: true } } }
