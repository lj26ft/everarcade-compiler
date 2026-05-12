pub fn federated_namespace_members(primary: [u8; 32], peers: &[[u8; 32]]) -> Vec<[u8; 32]> {
    let mut members = vec![primary];
    members.extend_from_slice(peers);
    members
}
