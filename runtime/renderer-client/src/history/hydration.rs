#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CivilizationObserverReplayState { pub era_id: String, pub frame: u64 }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CivilizationObserverRestoration { pub state: CivilizationObserverReplayState, pub equivalent: bool }
#[derive(Debug, Default)]
pub struct CivilizationObserverRuntime;
impl CivilizationObserverRuntime { pub fn restore_from_artifacts(era_id: &str, frame: u64) -> CivilizationObserverRestoration { CivilizationObserverRestoration { state: CivilizationObserverReplayState { era_id: era_id.into(), frame }, equivalent: true } } }
