# Simulation Workflow Report

## Coverage
- Play, pause, step, fast forward, checkpoint, and restore controls are exposed for creator simulation workflow.
- Simulation controls update deterministic replay hashes and checkpoint hashes.
- Replay remains append-only and restore is checkpoint-driven.

## Determinism
Simulation control equivalence tests validate deterministic restore and replay behavior.
