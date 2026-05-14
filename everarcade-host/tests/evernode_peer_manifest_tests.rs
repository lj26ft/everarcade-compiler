use everarcade_host::evernode::{
    discovery::discover, peer_manifest::EvernodePeerManifest, peer_registry::PeerRegistry,
};

#[test]
fn manifest_is_advisory_only() {
    let manifest = EvernodePeerManifest {
        host_id: [1; 32],
        package_root: [2; 32],
        latest_checkpoint_root: [3; 32],
        replay_root: [4; 32],
        sync_endpoint: "127.0.0.1:7777".into(),
    };
    let mut reg = PeerRegistry::default();
    reg.register(manifest);
    assert_eq!(discover(&reg).len(), 1);
}
