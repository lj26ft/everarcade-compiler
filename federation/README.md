# Multi-Lease Federation Runtime Layout

This directory contains the deterministic evidence-exchange runtime for Multi-Lease Federation Runtime v0.1.

- `members/` records replay-safe membership events: join, leave, suspend, and recover.
- `identity/` records federation, lease, node, epoch, and identity-root material.
- `topology/` records lease membership, evidence-only connectivity, and epoch state.
- `checkpoints/` records checkpoint export/import/verification evidence.
- `replay/` records replay export/import/verification evidence.
- `settlement/` records settlement export/import/verification evidence.
- `synchronization/` records civilization epoch synchronization roots.
- `recovery/` records lease failure, recovery, membership recovery, and checkpoint recovery evidence.
- `multi_lease_model.sh` is the deterministic root-generation and verification model used by validation and certification scripts.

## Pseudocode authority boundary

- **Input:** member events, identity roots, topology records, checkpoint/replay/settlement exchange records, and recovery evidence.
- **Output:** deterministic federation roots and equivalence reports.
- **Authority:** only the deterministic evidence model in this directory; it is not a live Byzantine-fault-tolerant network.
- **EverArcade fit:** documents and tests how leases could exchange evidence before a production federation exists.
