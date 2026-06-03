# 05. World Runtime

## Purpose

The world runtime defines persistent world domains such as entities, economy, inventory, governance, scheduling, simulation, and restoration. It exists to make long-running worlds deterministic, recoverable, and inspectable.

## Authority

World state is authoritative only when mutated through runtime execution and persisted through receipts, journal entries, and checkpoints. SDK world types and developer tools assist authoring but do not own canonical state.

## Domains

| Domain | Role | Status |
|---|---|---|
| Entity runtime | Identifies and mutates world entities. | Partial |
| Economy runtime | Models balances and value flows. | Partial |
| Inventory runtime | Tracks possessions and item state. | Partial |
| Simulation runtime | Advances deterministic world simulation. | Partial |
| Governance runtime | Models world policy and governance actions. | Partial |
| Scheduler | Orders world work and partition execution. | Partial |
| Restoration | Restores world state from checkpoints and replay. | Implemented foundation |

## Boundaries

The world runtime owns canonical world state domains. Renderers, editors, observers, dashboards, and analytics consume world projections but do not mutate authority.

## Readiness

World runtime capability is not production ready as a complete persistent-world product. See `11-production-readiness.md` and `12-gap-analysis.md` for gates and required work.
