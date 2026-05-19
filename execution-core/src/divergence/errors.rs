use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DivergenceError {
    DuplicateFork,
    InvalidWindow,
    OverlappingWindow,
}

impl Display for DivergenceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateFork => write!(f, "duplicate fork"),
            Self::InvalidWindow => write!(f, "invalid divergence window"),
            Self::OverlappingWindow => write!(f, "overlapping divergence window"),
        }
    }
}

impl std::error::Error for DivergenceError {}
