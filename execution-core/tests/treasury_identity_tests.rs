use execution_core::treasury::treasury_identity::derive_treasury_id;
#[test] fn treasury_identity_deterministic(){ let a=[1u8;32]; let b=[2u8;32]; assert_eq!(derive_treasury_id(a,b),derive_treasury_id(a,b)); }
