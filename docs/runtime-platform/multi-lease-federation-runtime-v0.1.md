# Multi-Lease Federation Runtime v0.1

## Purpose

Multi-Lease Federation Runtime v0.1 is the operational evidence-exchange layer for sovereign EverArcade protocol nodes running on separate leases. Each lease owns only its local state. The federation shares deterministic evidence roots so other leases can verify continuity, replay, settlement, membership, topology, synchronization, and recovery without acquiring mutation authority over peers.

This milestone does not implement consensus, peer discovery, public networking, HotPocket cluster networking, XRPL RPC, renderer runtime, or GPU runtime.

## Federation Identity

Federation identity captures:

- Federation ID.
- Lease IDs.
- Node IDs.
- Membership epoch.
- Lease authority labels.
- Identity root.
- Federation identity root.

The identity model is deterministic and canonicalized by `federation/multi_lease_model.sh`. The generated identity artifact is `federation/identity/federation_identity.txt`.

## Membership

Membership represents Join, Leave, Suspend, and Recover events. Events are ordered, epoch-aware, and replay-safe. Suspend and Recover are modeled as evidence events; they do not mutate another lease. The membership root commits to the identity root and the full membership event stream.

The generated membership artifact is `federation/members/membership_events.txt`.

## Topology

Topology represents:

- Federation membership.
- Lease membership.
- Connectivity relationships.
- Epoch state.

Connectivity is evidence-only. The topology model records intended coordination paths but does not implement networking or peer discovery. The topology root commits to the membership root.

The generated topology artifact is `federation/topology/topology_state.txt`.

## Checkpoint Exchange

Checkpoint exchange represents checkpoint export, import, and verification. It verifies:

- Checkpoint integrity.
- Checkpoint identity.
- Epoch match.

Checkpoint exchange is evidence-only. A lease may import and verify a peer checkpoint, but it may not use that exchange to mutate peer-owned state. The checkpoint exchange root commits to identity and topology roots.

The generated checkpoint artifact is `federation/checkpoints/checkpoint_exchange.txt`.

## Replay Exchange

Replay exchange represents replay export, import, and verification. It verifies:

- Replay integrity.
- Replay identity.
- Continuity match.

Replay remains authoritative. Given a checkpoint, replay stream, and settlement evidence, a lease must reproduce identical replay and continuity roots. The replay exchange root commits to the checkpoint exchange root.

The generated replay artifact is `federation/replay/replay_exchange.txt`.

## Settlement Exchange

Settlement exchange represents settlement export, import, and verification. It verifies:

- Settlement evidence.
- Receipt integrity.
- Authority match.

Settlement evidence is imported as proof material only. It does not grant cross-lease authority. The settlement exchange root commits to the replay exchange root.

The generated settlement artifact is `federation/settlement/settlement_exchange.txt`.

## Civilization Synchronization

Civilization synchronization represents:

- Civilization epoch.
- Checkpoint synchronization.
- Replay synchronization.
- Settlement synchronization.

The civilization synchronization root commits to checkpoint, replay, and settlement exchange roots. It also records sovereign mutation boundaries and failure isolation as PASS requirements.

The generated synchronization artifact is `federation/synchronization/civilization_synchronization.txt`.

## Recovery

Recovery represents:

- Lease failure.
- Lease recovery.
- Membership recovery.
- Checkpoint recovery.
- Recovered replay root.
- Recovered continuity root.

A failed lease is isolated. Recovery requires deterministic replay evidence and matching continuity evidence before membership recovery can pass. Recovery does not corrupt or rewrite other leases.

The generated recovery artifact is `federation/recovery/recovery_state.txt`.

## PASS Criteria

The runtime passes when validation and certification report PASS for:

- Identity.
- Membership.
- Topology.
- Checkpoint Exchange.
- Replay Exchange.
- Settlement Exchange.
- Synchronization.
- Recovery.

The validation report must include `Multi-Lease Federation Validation: PASS`.
The certification report must include `Multi-Lease Federation Runtime: PASS`.

## FAIL Criteria

The runtime fails if any of the following occur:

- A root is missing or non-deterministic.
- Membership events are not epoch-aware.
- Checkpoint identity or epoch matching fails.
- Replay continuity matching fails.
- Settlement receipt integrity or authority matching fails.
- Synchronization omits checkpoint, replay, or settlement evidence.
- Recovery cannot reproduce recovered replay and continuity roots.
- A lease exchange implies direct mutation authority over another lease.

## Relationship To Xaman Signing Layer

The Xaman signing layer remains the human authorization and signed receipt layer for settlement operations. Multi-lease federation consumes settlement evidence and receipt roots as verification material, but it does not request Xaman signatures, custody keys, or bypass human approval.

## Relationship To Future Public Testnet

This runtime is the prerequisite evidence layer for Public Testnet v0.1. Public networking, peer discovery, consensus, and HotPocket cluster networking can be layered on top later because the roots in this runtime already define deterministic identity, membership, topology, checkpoint, replay, settlement, synchronization, and recovery boundaries.
