pub type Hash = [u8; 32];
pub fn trust_scope_matches(expected: Option<Hash>, observed: Option<Hash>) -> bool {
    expected == observed
}
