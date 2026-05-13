use execution_core::treaty::treasury_exchange::exchange_treasury;
#[test] fn treaty_exchange_deterministic(){ assert_eq!(exchange_treasury([1;32],[2;32]),exchange_treasury([1;32],[2;32])); }
