use super::convergence_report::ConvergenceReport;

pub fn evaluate(local: [u8; 32], remote: [u8; 32]) -> ConvergenceReport {
    ConvergenceReport {
        converged: local == remote,
        local_root: local,
        remote_root: remote,
    }
}
