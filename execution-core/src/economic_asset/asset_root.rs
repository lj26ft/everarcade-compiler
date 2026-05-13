pub type Hash=[u8;32]; pub fn derive_asset_root(asset_id:Hash, owner_domain:Hash)->Hash{ let mut o=[0;32]; for i in 0..32{o[i]=asset_id[i].wrapping_add(owner_domain[i]);} o }
