use super::proof_verification::ReplayProofVerificationRuntime;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalContinuityRoot {
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalContinuityChainLink {
    pub artifact_id: String,
    pub previous_root: String,
    pub current_root: HistoricalContinuityRoot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalContinuityManifest {
    pub timeline_id: String,
    pub links: Vec<HistoricalContinuityChainLink>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayContinuityChain {
    pub links: Vec<HistoricalContinuityChainLink>,
}

impl ReplayContinuityChain {
    pub fn append(
        links: &mut Vec<HistoricalContinuityChainLink>,
        artifact_id: &str,
        payload: &[u8],
    ) -> HistoricalContinuityChainLink {
        let previous_root = links
            .last()
            .map(|l| l.current_root.value.clone())
            .unwrap_or_else(|| "genesis".into());
        let current_root = HistoricalContinuityRoot {
            value: ReplayProofVerificationRuntime::digest_bytes(
                &[previous_root.as_bytes(), payload].concat(),
            ),
        };
        let link = HistoricalContinuityChainLink {
            artifact_id: artifact_id.into(),
            previous_root,
            current_root,
        };
        links.push(link.clone());
        link
    }

    pub fn verify_append_only(links: &[HistoricalContinuityChainLink]) -> bool {
        links
            .windows(2)
            .all(|w| w[1].previous_root == w[0].current_root.value)
    }
}
