use execution_core::monetary::{monetary_policy::MonetaryPolicy, monetary_root::derive_monetary_root, monetary_validation::validate_monetary_policy};
#[test] fn issuance_validation(){ let issuance=[2u8;32]; let supply=[3u8;32]; let policy=MonetaryPolicy{ monetary_root:derive_monetary_root(issuance,supply), issuance_root:issuance, supply_root:supply}; assert!(validate_monetary_policy(&policy)); }
