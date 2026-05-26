#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IncrementalReplayCursor {
    pub offset: usize,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IncrementalReplayWindow {
    pub start: usize,
    pub end: usize,
}
#[derive(Clone, Debug, Default)]
pub struct IncrementalReplayValidationRuntime;
impl IncrementalReplayValidationRuntime {
    pub fn resume(&self, window: IncrementalReplayWindow) -> IncrementalReplayCursor {
        IncrementalReplayCursor { offset: window.end }
    }
}
