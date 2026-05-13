#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConvergenceReport {
    pub converged: bool,
    pub local_root: [u8; 32],
    pub remote_root: [u8; 32],
}
