use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FuelConsumptionRecord {
    pub requested: u64,
    pub consumed: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FuelExhaustionReceipt {
    pub consumed: u64,
    pub exhausted_at: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MeteredExecutionEnvelope {
    pub fuel: FuelConsumptionRecord,
    pub exhausted: Option<FuelExhaustionReceipt>,
}

#[derive(Clone, Debug)]
pub struct ExecutionFuelMeter {
    pub limit: u64,
    pub consumed: u64,
}
impl ExecutionFuelMeter {
    pub fn new(limit: u64) -> Self {
        Self { limit, consumed: 0 }
    }
    pub fn consume(&mut self, units: u64) -> Option<FuelExhaustionReceipt> {
        self.consumed = self.consumed.saturating_add(units);
        if self.consumed > self.limit {
            Some(FuelExhaustionReceipt {
                consumed: self.consumed,
                exhausted_at: self.limit,
            })
        } else {
            None
        }
    }
}
