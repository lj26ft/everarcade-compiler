pub type Hash=[u8;32];
#[derive(Clone,Debug,PartialEq,Eq)] pub struct AssetConstitution{ pub asset_id:Hash,pub constitutional_root:Hash,pub owner_domain:Hash }
