# behavior_tree Report

- ECS continuity: preserved where applicable.
- AI continuity: preserved where applicable.
- Partition continuity: replay-derived and deterministic.
- Federation continuity: replay roots are synchronized before migration.
- Replay continuity: append-only; observer replay is reconstruction-only.
- Migration readiness: ready for deterministic checkpoint/restore scaffolding.
- Operational limitations: scaffold-level domain integration; renderer/history/federation remain non-authoritative runtime domains.
