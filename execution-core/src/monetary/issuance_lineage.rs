use super::monetary_policy::Hash; pub fn derive_issuance_lineage(prior:Hash, issuance:Hash)->Hash{ let mut o=[0;32]; for i in 0..32{o[i]=prior[i].wrapping_add(issuance[i]);} o }
