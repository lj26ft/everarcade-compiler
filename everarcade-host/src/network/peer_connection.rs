#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PeerConnection {
    pub local_node_id: String,
    pub remote_node_id: String,
}
