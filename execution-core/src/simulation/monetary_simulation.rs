use crate::monetary::monetary_policy::Hash; pub fn simulate_monetary_step(root:Hash, issuance:u8)->Hash{ let mut o=root; o[0]=o[0].wrapping_add(issuance); o }
