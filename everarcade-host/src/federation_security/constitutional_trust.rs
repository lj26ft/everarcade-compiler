pub type Hash = [u8; 32];
pub fn constitutional_trust_valid(constitution_root: Hash, treaty_scope_root: Hash) -> bool { constitution_root == treaty_scope_root }
