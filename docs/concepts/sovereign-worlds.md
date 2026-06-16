# Sovereign Worlds

EverArcade uses **sovereignty** to mean that a world is not trapped inside one platform owner, one server operator, one storefront, or one unreviewable history database. A sovereign world has portable rules, portable execution expectations, recoverable state, independently checkable history, and external ownership or settlement boundaries where those boundaries matter.

This does not mean every world is fully decentralized on day one. It means EverArcade is designed so developers, operators, players, and verifiers do not have to treat a single platform database as the final source of truth forever.

## Why world ownership matters

Traditional platforms often combine four powers:

1. the server that runs the world,
2. the rules that decide valid state changes,
3. the distribution channel that controls access, and
4. the continuity record that says what happened.

When those powers are bundled, a developer can lose leverage even after building the community. If a platform changes policy, sunsets infrastructure, removes distribution, or rewrites service terms, the world may have no practical escape path.

EverArcade separates those powers:

- **Developers define the world** through world contracts, packages, rules, metadata, and upgrade boundaries.
- **Operators run the world** by hosting the runtime, preserving checkpoints, and serving players and tools.
- **Players inhabit the world** through the continuity created by shared state and history.
- **Verification protects the history** through replayable records and independently checkable execution.
- **Ownership anchors outside the platform** when XRPL/Xahau records are used for settlement, identity, or ownership boundaries.

The result is a design direction for developer-owned worlds, portable execution, independent verification, operator independence, less platform lock-in, and long-lived world continuity.

## How worlds remain portable

Portability starts with **World Contracts** and **Portable World Packages**. A world package should carry the information needed to understand what the world is, what rules it expects, what runtime assumptions apply, what assets or modules it references, and what operational boundaries an operator must respect.

The **Deterministic Runtime** is the execution side of that portability. If accepted inputs, package versions, and state roots are the same, deterministic execution is intended to produce the same transition result. That makes world execution less dependent on one private server implementation.

**Checkpoint Restore** adds recovery portability. Checkpoints provide known recovery material so an operator can resume from a documented continuity point after an incident, migration, or infrastructure change.

Together, package portability, deterministic execution, and checkpoint restore give a world a path to move across operators without pretending that the world has to start over.

## How replay verification protects continuity

Replay verification lets a verifier rerun or inspect a history window and compare the result against recorded roots, receipts, checkpoints, or other continuity evidence. Instead of asking players to trust that an operator's database is honest, EverArcade treats history as something that can be checked.

This protects continuity in three ways:

- invalid transitions can be detected when replay does not match expected roots,
- disputes can reference evidence rather than private operational claims, and
- migrations can carry a verifiable story of how the current state was reached.

Replay verification is especially important for long-lived worlds because continuity becomes part of the value of the world. The longer a world runs, the more important it becomes that its history is legible and not merely asserted.

## Why operators cannot silently rewrite world history

An operator can host infrastructure, admit inputs, serve players, publish checkpoints, and maintain uptime. That does not mean the operator should be able to silently redefine the past.

EverArcade limits silent rewrites by making important state transitions and recovery points auditable. Deterministic execution constrains what valid transitions look like. Replay verification can check whether recorded history produces the claimed state. Checkpoints create restore boundaries that can be compared against continuity evidence. Anchors can commit important ownership or settlement boundaries outside the operator's private database.

An operator may still fail, go offline, publish bad data, or refuse service. Sovereignty is not a promise that infrastructure can never fail. The promise is that the world design should make operator claims inspectable and make continuity portable enough for communities to recover or migrate when required.

## XRPL/Xahau anchoring boundaries

XRPL and Xahau anchoring are used for boundaries that should not live only inside the runtime platform. These boundaries can include ownership references, settlement records, identity-linked claims, or commitments to world continuity data.

Anchoring does not mean every gameplay action belongs on-chain. Most world execution should remain in the deterministic runtime and verification layer. The ledger boundary is for commitments that benefit from external settlement or ownership semantics. In this model, the platform does not have to be the sole authority over who owns a relevant asset or which external settlement record exists.

## Evernode and independent compute

Evernode supports the sovereignty story by giving operators and developers a path toward independent compute. Instead of assuming that one company-hosted backend is the only place a world can run, EverArcade can align packaged worlds, deterministic runtime expectations, and verification evidence with compute that is operated outside a single platform owner.

This matters because operator independence is not only a legal or branding claim. It requires practical infrastructure options. Evernode compute can help make independent world operation more concrete when paired with portable packages, replay verification, and checkpoint restore procedures.

## How this differs from traditional game hosting

Traditional game hosting usually optimizes for one product, one backend, one release pipeline, and one commercial service lifecycle. That can be efficient, but it often means the world ends when the service owner stops running the service.

EverArcade is designed around a different question: what would it take for a world to outlive one host, one storefront, one database, or one operating team?

| Traditional hosting | EverArcade sovereign-world direction |
| --- | --- |
| Server owner controls continuity. | Continuity is recorded, replayable, and recoverable. |
| Rules and hosting are tightly coupled. | World contracts and packages describe rules outside one host. |
| Migration depends on private tooling. | Portable packages and checkpoints support operator transitions. |
| History is accepted on platform trust. | Replay verification can independently check history windows. |
| Ownership often lives inside platform accounts. | XRPL/Xahau anchoring can place ownership or settlement boundaries outside the platform. |
| Compute is tied to the service provider. | Evernode-compatible operation supports independent compute paths. |

Sovereignty is therefore the technical and operational separation of powers around a world. Developers define it, operators run it, players inhabit it, verification protects it, and ownership boundaries can exist outside the platform that serves the current session.
