use execution_core::fiscal::{fiscal_policy::FiscalPolicy,fiscal_transition::transition_fiscal_policy};
#[test] fn fiscal_determinism(){ let p=FiscalPolicy{policy_id:[1;32],constitutional_root:[2;32],fiscal_root:[0;32],lineage_root:[3;32]}; let a=transition_fiscal_policy(&p,[5;32],[7;32]); let b=transition_fiscal_policy(&p,[5;32],[7;32]); assert_eq!(a.fiscal_root,b.fiscal_root); }
