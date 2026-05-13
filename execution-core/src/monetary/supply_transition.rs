use super::monetary_policy::Hash; pub fn derive_supply_root(prior:Hash, issuance:Hash)->Hash{ let mut o=[0;32]; for i in 0..32{o[i]=prior[i]^issuance[i];} o }
