# Operator Recovery

Persistence and queue state should be treated as recoverable state. If derived data is lost, rebuild from canonical artifacts.

## Truth vs Derived State

Protocol truth: packages, receipts, checkpoints, canonical replay roots.
Derived/rebuildable: indexes, manifests, caches, queue state.
