pub type Hash = [u8; 32];

pub fn identity_in_scope(scope: Option<Hash>, candidate: Option<Hash>) -> bool { scope == candidate }
