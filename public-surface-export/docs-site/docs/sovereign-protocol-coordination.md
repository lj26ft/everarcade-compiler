# Sovereign Protocol Coordination

This layer adds deterministic protocol coordination primitives for long-lived federations.

## Components

- Protocol upgrade continuity: `coordination::upgrade::ProtocolUpgradeManifest` binds protocol version jumps to compatibility, migration, and replay roots.
- Epoch transitions: `coordination::epoch::FederationEpochManifest` captures federation/checkpoint/topology/capability state per epoch.
- Governance manifest: `coordination::governance::GovernanceManifest` captures deterministic coordination state (non-political).
- Replay continuity validation: `coordination::replay_continuity` validates lineage, checkpoints, and migration journal alignment.
- Multi-version compatibility: `coordination::compatibility::ProtocolCompatibility` enforces explicit supported versions.
- Canonical migration journals: `coordination::migration::MigrationJournal` is append-only with ordered transitions.
- Capability negotiation: `coordination::capability::negotiate_capabilities` computes explicit intersection outcomes.
- Topology manifests: `coordination::topology::FederationTopologyManifest` canonicalizes node order.
- Routing manifests: `coordination::routing::RoutingManifest` produces deterministic routing outcomes.
- Persistence continuity: `coordination::persistence::SovereignPersistenceManifest` validates restoration continuity.

## Replay continuity guarantees

Replay continuity is preserved when:

1. epoch continuity roots remain aligned for compared histories,
2. checkpoint hashes are present across transitions,
3. migration journals maintain one transition per upgrade manifest.

## Determinism limitations

- Not Byzantine consensus.
- Assumes trusted deterministic input acquisition and canonical serialization.
- Compatibility assumes honest node capability declarations.
- Replay continuity requires stable continuity/checkpoint lineage inputs.
- Migration compatibility is limited to append-only journal semantics.
- Topology/routing assume canonical input ordering and deterministic workload sets.
- Persistence recovery assumes preserved sovereign identity and continuity roots.
