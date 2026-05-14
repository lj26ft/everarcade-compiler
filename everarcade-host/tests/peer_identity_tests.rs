use everarcade_host::network::peer_identity::derive_peer_id;
#[test]
fn peer_id_is_deterministic() {
    let a = derive_peer_id([1; 32], [2; 32]);
    let b = derive_peer_id([1; 32], [2; 32]);
    assert_eq!(a, b);
}
