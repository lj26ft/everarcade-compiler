use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncError {
    pub field: &'static str,
    pub expected: String,
    pub actual: String,
}

impl SyncError {
    pub fn mismatch(
        field: &'static str,
        expected: impl Into<String>,
        actual: impl Into<String>,
    ) -> Self {
        Self {
            field,
            expected: expected.into(),
            actual: actual.into(),
        }
    }
}

impl fmt::Display for SyncError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "sync {} expected={} actual={}",
            self.field, self.expected, self.actual
        )
    }
}

impl std::error::Error for SyncError {}

pub type Result<T> = std::result::Result<T, SyncError>;
