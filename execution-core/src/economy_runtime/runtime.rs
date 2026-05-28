use super::{
    continuity::economy_continuity_root,
    ledger::{next_entry, LedgerEntry},
    validation::validate_economy,
};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EconomyRuntime {
    pub tick: u64,
    pub ledger: Vec<LedgerEntry>,
    pub ledger_root: String,
    pub continuity_root: String,
}
impl EconomyRuntime {
    pub fn genesis() -> Self {
        let ledger_root = "ledger:genesis".to_string();
        Self {
            tick: 0,
            ledger: vec![],
            continuity_root: economy_continuity_root(0, &ledger_root),
            ledger_root,
        }
    }
    pub fn transfer(&mut self, from: &str, to: &str, amount: u64) -> Result<(), &'static str> {
        let e = next_entry(
            self.ledger.len() as u64 + 1,
            from,
            to,
            amount,
            &self.ledger_root,
        );
        self.tick += 1;
        self.ledger_root = e.entry_root.clone();
        self.continuity_root = economy_continuity_root(self.tick, &self.ledger_root);
        self.ledger.push(e);
        if validate_economy(self) {
            Ok(())
        } else {
            Err("ledger divergence rejected")
        }
    }
}
