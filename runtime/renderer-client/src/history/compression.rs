#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayCompressionNode {
    pub id: String,
    pub payload: Vec<u8>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayCompressionRoot {
    pub hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayCompressionTree {
    pub nodes: Vec<ReplayCompressionNode>,
    pub root: ReplayCompressionRoot,
}
impl ReplayCompressionTree {
    pub fn decompress(&self) -> Vec<u8> {
        self.nodes.iter().flat_map(|n| n.payload.clone()).collect()
    }
}
