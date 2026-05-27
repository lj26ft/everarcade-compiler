#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CiExecutionHistoryRuntime;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CiExecutionReplay {
    pub ordered_stage_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CiExecutionReplayWindow {
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CiExecutionRestoration {
    pub restored: bool,
}

impl CiExecutionHistoryRuntime {
    pub fn restore(
        replay: &CiExecutionReplay,
        window: &CiExecutionReplayWindow,
    ) -> CiExecutionRestoration {
        let ordered = replay.ordered_stage_ids.windows(2).all(|w| w[0] <= w[1]);
        CiExecutionRestoration {
            restored: ordered
                && window.start <= window.end
                && window.end <= replay.ordered_stage_ids.len(),
        }
    }
}
