use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum OperatorProfile {
    Local,
    Live,
    TestnetAnchor,
}

impl OperatorProfile {
    pub fn state_layout(self) -> &'static str {
        match self {
            Self::Local => ".everarcade",
            Self::Live => "evernode/state",
            Self::TestnetAnchor => "evernode/testnet-anchor",
        }
    }
}
