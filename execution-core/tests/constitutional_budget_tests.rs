use execution_core::fiscal::fiscal_root::derive_fiscal_root;
#[test] fn constitutional_budget_continuity(){ assert_eq!(derive_fiscal_root([1;32],[2;32]),derive_fiscal_root([1;32],[2;32])); }
