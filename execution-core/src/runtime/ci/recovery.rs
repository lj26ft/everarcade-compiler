#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CiExecutionTimeout {
    pub max_ticks: u64,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CiRecoveryWindow {
    pub checkpoint: String,
    pub resume_tick: u64,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CiRecoveryState {
    pub interrupted: bool,
    pub window: CiRecoveryWindow,
}
#[derive(Clone, Debug, Default)]
pub struct CiTimeoutRuntime;
impl CiTimeoutRuntime {
    pub fn recover(&self, timeout: &CiExecutionTimeout, checkpoint: String) -> CiRecoveryState {
        CiRecoveryState {
            interrupted: true,
            window: CiRecoveryWindow {
                checkpoint,
                resume_tick: timeout.max_ticks,
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LongRunningValidationRuntime;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LongRunningValidationCheckpoint {
    pub checkpoint_id: String,
    pub resume_from_stage: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LongRunningValidationRecovery {
    pub recovered: bool,
}

impl LongRunningValidationRuntime {
    pub fn recover(
        checkpoint: &LongRunningValidationCheckpoint,
        total_stages: usize,
    ) -> LongRunningValidationRecovery {
        LongRunningValidationRecovery {
            recovered: checkpoint.resume_from_stage <= total_stages,
        }
    }
}
