#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContinuityBundle {
    pub manifest: ContinuityBundleManifest,
    pub lineage: Vec<u8>,
    pub checkpoint: Vec<u8>,
    pub receipts: Vec<Vec<u8>>,
    pub recovery_descriptor: Vec<u8>,
    pub package: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContinuityBundleManifest {
    pub package_root: [u8; 32],
    pub checkpoint_root: [u8; 32],
    pub lineage_root: [u8; 32],
    pub receipt_chain_root: [u8; 32],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContinuityImportReport {
    pub package_root_matches: bool,
    pub lineage_valid: bool,
    pub checkpoint_continuity_valid: bool,
    pub manifest_valid: bool,
    pub receipts_replay_valid: bool,
}

impl ContinuityImportReport {
    pub fn is_valid(&self) -> bool {
        self.package_root_matches
            && self.lineage_valid
            && self.checkpoint_continuity_valid
            && self.manifest_valid
            && self.receipts_replay_valid
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContinuityExportProof {
    pub bundle_hash: [u8; 32],
    pub export_root: [u8; 32],
}
