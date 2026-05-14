#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DivergenceReason {
    StateRootMismatch,
    ReceiptMismatch,
    GraphMismatch,
    DependencyMismatch,
    ExecutionOrderMismatch,
}
