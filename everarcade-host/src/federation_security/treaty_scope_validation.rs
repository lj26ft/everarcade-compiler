pub type Hash = [u8; 32];
pub fn treaty_scope_valid(expected: Hash, received: Hash) -> bool { expected == received && expected != [0; 32] }
