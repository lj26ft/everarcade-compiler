use crate::sync::cursor::SyncCursor;

use super::mesh::ObserverMesh;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConvergenceReport {
    pub converged: bool,
    pub matching_observers: usize,
    pub diverged_observers: usize,
}

pub fn verify_convergence(mesh: &ObserverMesh) -> ConvergenceReport {
    let expected: Option<&SyncCursor> = mesh.observers.values().next().map(|n| &n.latest_cursor);
    let mut matching = 0usize;
    let mut diverged = 0usize;
    for node in mesh.observers.values() {
        if Some(&node.latest_cursor) == expected {
            matching += 1;
        } else {
            diverged += 1;
        }
    }
    ConvergenceReport {
        converged: diverged == 0,
        matching_observers: matching,
        diverged_observers: diverged,
    }
}
