#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinkerPressureDiagnostic {
    pub pressure_score: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationResourceWindow {
    pub cpu_millis: u64,
    pub io_bytes: u64,
}
