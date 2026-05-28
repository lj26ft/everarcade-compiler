#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionDescriptor {
    pub session_id: String,
    pub lineage: String,
    pub tick_rate: u16,
}
impl SessionDescriptor {
    pub fn new(session_id: impl Into<String>, lineage: impl Into<String>, tick_rate: u16) -> Self {
        Self {
            session_id: session_id.into(),
            lineage: lineage.into(),
            tick_rate,
        }
    }
    pub fn validate(&self) -> bool {
        !self.session_id.is_empty() && !self.lineage.is_empty() && self.tick_rate > 0
    }
}
