# Protocol Sovereignty Certification v0.1

## Purpose

The Protocol Sovereignty Certification validates that the EverArcade Runtime
Platform v0.1 composes its already-certified subsystems into one deterministic
protocol stack. It does not add runtime features. It proves that bootstrap,
execution, persistence, world runtime, physics, economy, ownership, inventory,
vault custody, authority, marketplace, governance, civilization, XRPL authority
mapping, XRPL settlement, and federated settlement can be treated as one
replayable and restorable protocol lifecycle.

## Sovereign Runtime Model

A sovereign runtime is a self-contained deterministic runtime whose protocol
state can be derived from certified local inputs. The certification rejects any
requirement for live XRPL connectivity, consensus, networking, HotPocket,
Evernode deployment, GPU authority, or another external dependency. Settlement
and federation are validated by deterministic transcripts and certified roots,
not by live network calls.

## Protocol Composition Model

The composed protocol chain is:

```text
Bootstrap -> Execution -> Persistence -> Recovery -> World Runtime -> Physics
-> Economy -> Ownership -> Inventory -> Vault Custody -> Authority
-> Marketplace -> Governance -> Civilization -> XRPL Authority Mapping
-> XRPL Settlement -> Federated Equivalence -> Protocol Continuity Root
-> Replay Root -> PASS
```

The certification first validates prior certification reports or reruns the
corresponding certification script when a report is missing or not passing. All
prior subsystems must report `PASS` before the composed protocol can pass.

## Continuity Model

The protocol lifecycle root is derived from certified subsystem roots plus a
canonical v0.1 lifecycle containing treasury activity, marketplace activity,
governance activity, settlement activity, and federated activity. The protocol
continuity root binds the lifecycle root, checkpoint identifier, restoration
root, settlement equivalence root, and federation equivalence root.

## Restoration Model

Checkpoint validation creates a protocol checkpoint identifier from the lifecycle
root and the certified protocol domains. Restoration validation then derives a
protocol restoration root from the checkpoint and verifies these restored domain
claims:

- Treasury restored.
- Civilization restored.
- Settlement restored.
- Federation restored.

Restoration succeeds only when the checkpoint identifier is created and verified
and the restoration root is deterministic.

## Replay Model

Replay validation recomputes the complete continuity transcript from the same
certified inputs. The certification passes replay only when:

```text
Protocol Replay Root == Protocol Continuity Root
```

A mismatch is treated as protocol divergence.

## Federation Model

Federation is modeled through the certified Federated Settlement output. The
sovereignty certification consumes the final federated root and requires the
Federated Settlement certification to pass. This demonstrates deterministic
multi-node equivalence without introducing networking or consensus.

## Settlement Model

Settlement is modeled through the certified XRPL Settlement output. The
sovereignty certification consumes the settlement continuity root and requires
XRPL Settlement to pass. It verifies settlement equivalence using deterministic
intent, authorization, receipt, checkpoint, restoration, and replay evidence
rather than live XRPL connectivity.

## Sovereignty Rules

The protocol is sovereign only when all of the following are true:

- No external dependency is required.
- Deterministic continuity is preserved.
- Replay equivalence is preserved.
- Restoration equivalence is preserved.
- Settlement equivalence is preserved.
- Federation equivalence is preserved.

## PASS Criteria

The certification passes when:

1. All prior certification statuses are `PASS`.
2. The deterministic protocol lifecycle root is generated.
3. The protocol checkpoint identifier is generated and verified.
4. The protocol restoration root is generated.
5. Treasury, civilization, settlement, and federation are restored.
6. The protocol replay root equals the protocol continuity root.
7. Sovereignty validation reports `PASS`.
8. The sovereignty report is written to
   `reports/protocol_sovereignty_certification_report.txt`.

## FAIL Criteria

The certification fails on any of the following:

- A prior required certification is not `PASS`.
- Checkpoint creation or verification fails.
- Treasury, civilization, settlement, or federation restoration fails.
- Protocol replay root differs from protocol continuity root.
- Settlement equivalence fails.
- Federation equivalence fails.
- Continuity diverges.
- Any external dependency is required to complete the certification.

## Relationship To Federated Settlement Certification

Federated Settlement Certification proves deterministic equivalence across the
settlement and civilization state of multiple independent nodes. Protocol
Sovereignty Certification consumes that evidence as the federation layer of the
complete v0.1 stack. It does not replace the federated certification; it composes
it with the rest of the certified runtime platform.

## Relationship To EverArcade Runtime Platform v0.1

This is the final v0.1 certification. Its scope is the complete certified
EverArcade Runtime Platform stack and its outcome demonstrates that v0.1 is a
sovereign deterministic runtime protocol: replayable, restorable,
settlement-capable, and federation-ready. Post-v0.1 deployment work such as
HotPocket, Evernode deployment, consensus, networking, GPU authority, or live
XRPL settlement remains outside this certification.
