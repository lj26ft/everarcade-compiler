use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PaymentWitness {
    pub payment_id: String,
    pub source: String,
    pub destination: String,
    pub amount_drops: u64,
}
