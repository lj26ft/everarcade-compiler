use crate::{diagnostic, reject_authority_bypass, simulation_debugger, CreatorDiagnostic};

pub fn validate_simulation_debugger() -> CreatorDiagnostic { diagnostic("simulation-debugger-validation", &["ai", "scheduler", "partition", "replay"] ) }

pub fn simulation_debugger_equivalence(ticks: &[&str]) -> bool { simulation_debugger::runtime::visualize_runtime_execution(ticks) == simulation_debugger::runtime::visualize_runtime_execution(ticks) }

pub fn request_authority_mutation(requested: bool) -> Result<(), &'static str> { reject_authority_bypass(requested) }
