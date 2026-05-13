use everarcade_host::discovery::{discovery_message::DiscoveryMessage,discovery_validation::validate_discovery};
#[test] fn discovery_is_advisory_and_validated(){assert!(validate_discovery(&DiscoveryMessage{peer_id:[9;32],available:true}));assert!(!validate_discovery(&DiscoveryMessage{peer_id:[0;32],available:true}));}
