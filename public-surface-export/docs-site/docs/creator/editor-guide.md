# EverArcade Visual Editor Guide

The visual editor presents deterministic runtime state as a read-only creator surface. It visualizes world state, session state, entity state, checkpoints, and replay continuity roots without becoming authoritative.

Workflow:
1. Open a replay-backed editor session.
2. Inspect world/entity state through deterministic snapshots.
3. Restore checkpoints visually by reconstructing from replay lineage.
4. Run validation before packaging or deployment.

Guarantees: renderer and editor views are non-authoritative; replay lineage remains append-only; authority remains inside deterministic execution runtime boundaries.
