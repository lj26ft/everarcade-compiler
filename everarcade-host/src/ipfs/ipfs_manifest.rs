#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IpfsManifest {
    pub package_cid: String,
    pub receipt_cid: String,
    pub checkpoint_cid: String,
}
