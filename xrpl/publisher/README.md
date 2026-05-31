# XRPL Publisher Boundary

The EverArcade runtime emits deterministic anchor records. Publication to XRPL remains external to runtime execution and is represented by publisher surfaces in `execution_core::xrpl::publisher`:

- `ReceiptPublisher`
- `ReplayPublisher`
- `CheckpointPublisher`
- `DeploymentPublisher`

The boundary validates generation, serialization, publication payload construction, and payload verification without submitting transactions from the runtime.
