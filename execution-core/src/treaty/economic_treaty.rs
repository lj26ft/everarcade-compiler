pub type Hash=[u8;32];
#[derive(Clone,Debug,PartialEq,Eq)] pub struct EconomicTreaty{ pub treaty_id:Hash,pub treasury_scope_root:Hash,pub fiscal_scope_root:Hash,pub monetary_scope_root:Hash }
