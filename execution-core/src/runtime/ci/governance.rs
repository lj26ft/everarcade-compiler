#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SovereignGovernanceRuntime {
    pub policy: SovereignCertificationPolicy,
    pub operational_policy: SovereignOperationalPolicy,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SovereignCertificationPolicy {
    pub require_certification: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SovereignOperationalPolicy {
    pub append_only_lineage: bool,
}

impl SovereignGovernanceRuntime {
    pub fn enforce_certified_state(&self, certified: bool) -> Result<(), String> {
        if self.policy.require_certification && !certified {
            return Err("uncertified release state rejected".into());
        }
        Ok(())
    }
}
