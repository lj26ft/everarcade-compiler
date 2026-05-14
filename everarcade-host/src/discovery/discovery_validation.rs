use super::discovery_message::DiscoveryMessage;

pub fn validate_discovery(msg: &DiscoveryMessage) -> bool {
    msg.peer_id != [0; 32]
}
