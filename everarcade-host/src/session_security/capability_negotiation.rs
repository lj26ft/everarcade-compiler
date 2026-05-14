pub type Hash = [u8; 32];
pub fn negotiate_capabilities(local: &[Hash], remote: &[Hash]) -> Vec<Hash> { local.iter().copied().filter(|r| remote.contains(r)).collect() }
