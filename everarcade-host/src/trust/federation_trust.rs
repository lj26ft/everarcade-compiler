pub type Hash = [u8; 32];
pub fn treaty_scoped_trust(scope_root: Option<Hash>, federation_scope_root: Option<Hash>) -> bool { scope_root==federation_scope_root }
