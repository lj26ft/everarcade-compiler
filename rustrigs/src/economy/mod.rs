use crate::{ReplaySafeRustrig, Rustrig, RustrigDescriptor, VersionedRustrig};
use contract_api::protocol_records::{fields, EconomyRecord};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EconomyInput {
    pub account: String,
    pub asset: String,
    pub amount: u64,
    pub counterparty: String,
    pub tick: u64,
}
fn rec(action: &str, i: EconomyInput) -> EconomyRecord {
    EconomyRecord::new(
        action,
        i.account,
        fields(&[
            ("asset", i.asset),
            ("amount", i.amount.to_string()),
            ("counterparty", i.counterparty),
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
                rec($action, input)
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
rig!(MintAsset, "mint-asset");
rig!(TransferAsset, "transfer-asset");
rig!(BurnAsset, "burn-asset");
rig!(CreateLedgerEntry, "create-ledger-entry");
rig!(CreateSettlement, "create-settlement");
pub fn descriptors() -> Vec<RustrigDescriptor> {
    [
        "MintAsset",
        "TransferAsset",
        "BurnAsset",
        "CreateLedgerEntry",
        "CreateSettlement",
    ]
    .into_iter()
    .map(|n| RustrigDescriptor::new(n, "1.0.0", "EconomyRecord"))
    .collect()
}
