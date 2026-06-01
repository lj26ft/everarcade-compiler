use crate::{ReplaySafeRustrig, Rustrig, RustrigDescriptor, VersionedRustrig};
use contract_api::protocol_records::{fields, EconomyRecord, ProtocolRecord, XrplIntentRecord};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EconomyInput {
    pub account: String,
    pub counterparty: String,
    pub asset: String,
    pub amount: u64,
    pub memo: String,
    pub tick: u64,
}
fn e(action: &str, i: &EconomyInput) -> EconomyRecord {
    EconomyRecord::new(
        action,
        i.account.clone(),
        fields(&[
            ("counterparty", i.counterparty.clone()),
            ("asset", i.asset.clone()),
            ("amount", i.amount.to_string()),
            ("memo", i.memo.clone()),
            ("tick", i.tick.to_string()),
        ]),
    )
}
fn x(action: &str, i: &EconomyInput) -> XrplIntentRecord {
    XrplIntentRecord::new(
        action,
        i.account.clone(),
        fields(&[
            ("destination", i.counterparty.clone()),
            ("asset", i.asset.clone()),
            ("amount", i.amount.to_string()),
            ("memo", i.memo.clone()),
            ("submission", "intent-only".to_string()),
            ("tick", i.tick.to_string()),
        ]),
    )
}
macro_rules! rig {
    ($name:ident,$action:literal) => {
        pub struct $name;
        impl Rustrig for $name {
            type Input = EconomyInput;
            type Output = EconomyRecord;
            fn execute(input: Self::Input) -> Self::Output {
                e($action, &input)
            }
        }
        impl ReplaySafeRustrig for $name {}
        impl VersionedRustrig for $name {
            const NAME: &'static str = stringify!($name);
            const VERSION: &'static str = "1.0.0";
            const RECORD_TYPE: &'static str = "EconomyRecord";
        }
    };
}
rig!(CreateLedgerEntry, "create-ledger-entry");
rig!(TransferAsset, "transfer-asset");
rig!(MintGameAsset, "mint-game-asset");
rig!(BurnGameAsset, "burn-game-asset");
pub fn create_ledger_entry(i: EconomyInput) -> Vec<ProtocolRecord> {
    vec![ProtocolRecord::Economy(e("create-ledger-entry", &i))]
}
pub fn transfer_asset(i: EconomyInput) -> Vec<ProtocolRecord> {
    vec![ProtocolRecord::Economy(e("transfer-asset", &i))]
}
pub fn mint_game_asset(i: EconomyInput) -> Vec<ProtocolRecord> {
    vec![ProtocolRecord::Economy(e("mint-game-asset", &i))]
}
pub fn burn_game_asset(i: EconomyInput) -> Vec<ProtocolRecord> {
    vec![ProtocolRecord::Economy(e("burn-game-asset", &i))]
}
pub fn create_settlement_intent(i: EconomyInput) -> Vec<ProtocolRecord> {
    vec![
        ProtocolRecord::Economy(e("create-settlement-intent", &i)),
        ProtocolRecord::XrplIntent(x("create-settlement-intent", &i)),
    ]
}
pub fn descriptors() -> Vec<RustrigDescriptor> {
    [
        "CreateLedgerEntry",
        "TransferAsset",
        "MintGameAsset",
        "BurnGameAsset",
        "CreateSettlementIntent",
    ]
    .into_iter()
    .map(|n| RustrigDescriptor::new(n, "1.0.0", "EconomyRecord,XrplIntentRecord"))
    .collect()
}
