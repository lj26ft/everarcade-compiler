#[derive(Debug, Clone, Default)]
pub struct AdapterSecrets {
    pub xrpl_secret: Option<String>,
    pub ipfs_token: Option<String>,
    pub evernode_key: Option<String>,
}
