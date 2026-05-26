#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalCorruptionScenario {
    pub name: String,
    pub violated: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalCorruptionResult {
    pub accepted: bool,
    pub rejected_scenarios: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalCorruptionMatrix {
    pub scenarios: Vec<HistoricalCorruptionScenario>,
}

impl HistoricalCorruptionMatrix {
    pub fn evaluate(&self) -> HistoricalCorruptionResult {
        let rejected_scenarios = self
            .scenarios
            .iter()
            .filter(|s| s.violated)
            .map(|s| s.name.clone())
            .collect::<Vec<_>>();
        HistoricalCorruptionResult {
            accepted: rejected_scenarios.is_empty(),
            rejected_scenarios,
        }
    }
}
