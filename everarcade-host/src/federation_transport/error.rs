use thiserror::Error;

#[derive(Debug, Error)]
pub enum FederationTransportError {
    #[error("malformed continuity artifact")]
    MalformedArtifact,
    #[error("malformed continuity bundle")]
    MalformedBundle,
}
