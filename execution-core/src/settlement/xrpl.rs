use super::{anchor, checkpoint::SettlementCheckpoint};

pub fn anchor_checkpoint(checkpoint: &SettlementCheckpoint) -> String { anchor::xrpl_anchor_payload(checkpoint) }
