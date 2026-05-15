use crate::distributed_receipts::Hash;
use std::collections::BTreeMap;

#[derive(Default)]
pub struct CheckpointReceiptIndex(pub BTreeMap<Hash, Vec<Hash>>);
