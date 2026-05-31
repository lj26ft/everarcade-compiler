# Multi-Node Federation and Load Gate

| Domain | Classification | Evidence |
| --- | --- | --- |
| multi-node federation | Ready | Four loopback TCP EverNode endpoints join with unique node IDs and endpoints. |
| checkpoint sync | Ready | Each node reports the same Arena Vanguard checkpoint root. |
| replay sync | Ready | Each node reports the same Arena Vanguard replay root. |
| recovery continuity | Ready | Each node reports the same deployment continuity root. |
| load balance | Ready | Each node accepts the configured deterministic message load within capacity. |

## Gate Criteria
- Node count must be at least the required node count and satisfy quorum.
- Node IDs and TCP endpoints must be unique.
- Package, replay, checkpoint, and continuity roots must be equivalent across nodes.
- Per-node observed load must equal the configured deterministic load and remain within capacity.
