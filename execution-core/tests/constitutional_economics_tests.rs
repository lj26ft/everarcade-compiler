use execution_core::{simulation::sovereign_economics::sovereign_economic_root, treasury::treasury_root::derive_treasury_root};
#[test] fn constitutional_economics_continuity(){ let t=derive_treasury_root([1;32],[2;32]); let e=sovereign_economic_root(t,[2;32],[1;32]); assert_eq!(e,sovereign_economic_root(t,[2;32],[1;32])); }
