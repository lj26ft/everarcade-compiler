use super::replay_cursor::ReplayCursor;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayResumeRequest {
    pub peer_id: String,
    pub cursor: ReplayCursor,
    pub reconstruction_only: bool,
}
impl ReplayResumeRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.reconstruction_only {
            Ok(())
        } else {
            Err("authority_resume_rejected".into())
        }
    }
}
