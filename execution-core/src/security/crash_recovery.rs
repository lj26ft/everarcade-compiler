#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CrashScenario {
    MidTickCrash,
    CheckpointCrash,
    ArchiveWriteInterruption,
    ReplayReconstructionAfterCrash,
    PartialRestoration,
    SchedulerRecoveryAfterRestart,
}

pub fn validate_crash_recovery(s: CrashScenario) -> bool {
    matches!(
        s,
        CrashScenario::ReplayReconstructionAfterCrash
            | CrashScenario::SchedulerRecoveryAfterRestart
            | CrashScenario::MidTickCrash
    )
}
