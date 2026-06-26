> **Repository boundary:** This repository includes local verification and certification primitives.
>
> Hosted verification services, public badge programs, reviewer marketplaces, application workflows, and paid certification products are outside the scope of this open-source reference implementation.
>
# World Contract Certification Framework v1

Status: v1 protocol framework  
Scope: deterministic gameplay mutation surface for EverArcade worlds  
Primary output: certifiable World Contract declarations and certified RustRig mutation libraries

## Purpose

The runtime certification ladder proves:

```text
ArenaState -> Canonical Bytes -> State Root -> World Hash
```

The World Contract Certification Framework proves:

```text
Input -> Deterministic Mutation -> State Transition -> Canonical State
```

Together these define the complete gameplay verification surface: every accepted input must resolve through a declared deterministic mutation interface before its effects enter canonical state, root calculation, replay, federation comparison, and world hash publication.

## 1. World Contract Model

A **World Contract** is a formally declared deterministic mutation interface for a world.

A World Contract declares:

- **State Schema**: the canonical state domains, field types, key spaces, and serialization rules the world may contain.
- **Mutation Surface**: the complete set of named emitters allowed to propose state transitions.
- **Authority Rules**: the identities, roles, capabilities, signatures, or system contexts allowed to call each emitter.
- **Validation Rules**: deterministic preconditions that must hold before a mutation may be applied.
- **Transition Rules**: deterministic state-write rules performed after validation succeeds.
- **Certification Targets**: replay, root, receipt, authority, federation, and upgrade properties that must be checked.

The World Contract model is:

```text
World Contract
  -> Allowed Emitters
  -> Allowed Mutations
  -> State Transition
  -> Canonical State
```

World Contracts are **protocol artifacts**, not hidden server code. A runtime may implement scheduling, networking, storage, and execution mechanics, but the gameplay mutation API itself must be explicit, versioned, auditable, replayable, and certifiable.

A mutation is outside the protocol if it is not declared by the active World Contract. Such a mutation must not alter canonical gameplay state.

## 2. Deterministic Emitters

An **Emitter** is a deterministic state-transition function. It is the smallest contract-declared unit that may validate input and produce canonical state changes.

Examples include:

- `combat.attack()`
- `inventory.transfer()`
- `quest.complete()`
- `market.trade()`
- `governance.vote()`
- `world.spawn()`
- `world.despawn()`
- `economy.mint()`
- `economy.burn()`

Each emitter must:

1. Accept deterministic inputs.
2. Produce deterministic outputs.
3. Produce deterministic state transitions.
4. Produce deterministic receipts.

Emitter behavior must not depend on wall-clock time, host randomness, thread scheduling, network arrival order outside the canonical input order, floating-point nondeterminism, filesystem state, process environment, or any other non-contract source of entropy.

An emitter declaration must include:

- stable emitter name and version;
- input schema;
- authority requirement;
- state read set;
- state write set;
- validation preconditions;
- transition effects;
- receipt schema;
- failure behavior;
- certification class;
- upgrade compatibility constraints.

## 3. Certification Classes

World Contract emitters are grouped into certification classes so certification tooling can apply the correct proof obligations.

### CLASS-A: State Mutation Emitters

General gameplay mutations that update ordinary world state.

Examples: `combat.attack()`, `inventory.transfer()`, `quest.complete()`, `position.move()`.

Required focus: deterministic validation, write-set integrity, replay equivalence, receipt integrity, canonical output.

### CLASS-B: Economic Emitters

Mutations that create, destroy, escrow, price, trade, or transfer economically significant assets.

Examples: `market.trade()`, `asset.transfer()`, `economy.mint()`, `economy.burn()`.

Required focus: conservation rules, supply rules, balance non-negativity, authorization, deterministic settlement ordering, audit receipts.

### CLASS-C: Governance Emitters

Mutations that affect voting, permissions, policy, upgrades, councils, guilds, or collective authority.

Examples: `governance.vote()`, `governance.propose()`, `guild.invite()`, `guild.kick()`.

Required focus: eligibility, quorum, deterministic tallying, replayable proposal state, upgrade safety, authority continuity.

### CLASS-D: World Management Emitters

Mutations that add, remove, migrate, or configure world entities and world-level lifecycle state.

Examples: `world.spawn()`, `world.despawn()`, `world.configure_zone()`, `world.rotate_epoch()`.

Required focus: namespace safety, lifecycle integrity, canonical ordering, migration boundaries, federation equivalence.

### CLASS-E: System Emitters

Runtime-mediated protocol emitters required for continuity, checkpointing, anchoring, receipt publication, or system maintenance.

Examples: `system.checkpoint()`, `system.anchor_root()`, `system.rotate_contract()`, `system.emit_continuity()`.

Required focus: protocol authority, root equivalence, receipt linkage, upgrade safety, host/runtime separation.

## 4. Mutation Certification Requirements

For every emitter, certification must verify the following properties:

| Requirement | Meaning |
| --- | --- |
| Determinism | The same canonical input and prior canonical state always produce the same result. |
| Replay Equivalence | Re-executing the same canonical input log produces byte-equivalent canonical state and receipts. |
| Root Equivalence | The post-mutation canonical state produces the expected state root across conforming implementations. |
| Receipt Integrity | The emitted receipt correctly identifies the input, emitter, authority, pre-state, post-state, and transition result. |
| State Transition Integrity | Only declared state may be read or written, and writes must match the declared transition rules. |
| Canonical Output | All outputs, receipts, state bytes, roots, and failure records use canonical serialization and ordering. |

A mutation is certifiable only if **all** of these properties hold. A failure in any property makes the emitter uncertified for the affected contract version.

## 5. World Contract Invariants

### WC-001: Emitter Determinism

Every declared emitter must be a deterministic function of canonical prior state, canonical input, and declared contract constants.

### WC-002: Mutation Integrity

Every state write must be authorized by the emitter declaration, validation result, and transition rule.

### WC-003: Receipt Integrity

Every accepted or rejected mutation must produce a deterministic receipt containing enough information to audit the emitter, input, authority, result, and state transition boundary.

### WC-004: Replay Equivalence

Given the same contract version, genesis state, and canonical input sequence, all conforming runtimes must produce equivalent receipts, canonical states, state roots, and world hashes.

### WC-005: State Root Equivalence

Each accepted mutation must enter the canonical state pipeline such that equivalent states produce equivalent state roots across conforming implementations.

### WC-006: Authority Enforcement

No emitter may execute unless its authority rules are satisfied by deterministic identity, role, capability, signature, governance, or system-context checks.

### WC-007: Contract Upgrade Safety

A contract upgrade must be versioned, authorized, replayable, migration-bounded, and able to prove continuity from the previous canonical state root to the upgraded canonical state root.

### WC-008: Federation Equivalence

Federated nodes executing the same contract, inputs, and canonical prior state must agree on mutation results, receipts, canonical state, and roots.

## 6. RustRig Model

A **RustRig** is a certified deterministic mutation library.

RustRigs implement emitter logic and are composed by World Contracts. A World Contract declares which RustRigs are active, which emitter functions they expose, which state domains they may access, and which certification class applies to each emitter.

Example RustRig emitters:

- `combat.attack()`
- `inventory.transfer()`
- `market.trade()`
- `quest.complete()`
- `governance.vote()`
- `guild.invite()`
- `asset.transfer()`
- `economy.mint()`
- `economy.burn()`

RustRig certification requires:

- no hidden mutation surface;
- no nondeterministic host dependencies;
- explicit input, output, receipt, read-set, and write-set schemas;
- deterministic failure handling;
- compatibility with canonical serialization;
- proof that the RustRig behavior matches the World Contract declaration.

World Contracts compose RustRigs; RustRigs do not implicitly define the protocol surface. Only contract-declared RustRig functions are active protocol emitters.

## 7. Certification Ladder

| Level | Name | Certification Meaning |
| --- | --- | --- |
| Level 0 | Experimental | Emitter exists but is not certified for deterministic protocol use. |
| Level 1 | Deterministic | Emitter passes deterministic single-run checks. |
| Level 2 | Replay Certified | Emitter passes replay equivalence against canonical input logs. |
| Level 3 | Federation Certified | Emitter produces equivalent results across federated conforming runtimes. |
| Level 4 | Root Certified | Emitter output enters canonical state and produces equivalent state roots. |
| Level 5 | World Contract Certified | Emitter is fully declared by a World Contract and satisfies all WC invariants. |
| Level 6 | Protocol Certified | Contract, emitters, receipts, roots, upgrades, and federation behavior are certified protocol artifacts. |

## 8. Reference World Contract

The reference declaration lives at `contracts/reference-world-contract-v1/` and defines the baseline domains and certification targets for an EverArcade deterministic world.

Reference state domains:

- `players`
- `inventory`
- `position`
- `combat`
- `economy`
- `quests`
- `receipts`
- `continuity`
- `governance`

The reference contract provides:

- mutation declarations;
- authority declarations;
- state declarations;
- certification targets.

A conforming implementation may extend the reference contract by adding versioned domains and emitters, but extensions must declare their state, authority, transition, receipt, and certification targets before they may affect canonical state.

## 9. Developer Experience

The canonical EverArcade development workflow is:

```text
Developer writes RustRig
  -> World Contract declares RustRig
  -> Certification validates RustRig
  -> Runtime executes certified mutation
  -> State enters canonical pipeline
  -> Replay verifies
  -> Federation verifies
  -> Roots verify
```

This workflow separates gameplay design from hidden runtime authority. Developers author deterministic mutation libraries, declare the allowed protocol surface in a World Contract, run certification, and then allow certified runtimes to execute the declared emitters against canonical state.

A developer should be able to answer the following without reading runtime implementation code:

- What state may exist?
- Which emitters may mutate that state?
- Who may call each emitter?
- Which validation rules apply?
- What receipts are produced?
- Which roots and replay artifacts certify the transition?

## 10. Future Proof Targets

World Contract certification maps directly to future proof obligations:

| Future Proof Target | Framework Mapping | Related Artifact |
| --- | --- | --- |
| World Contract Determinism | WC-001, deterministic emitter schemas, replay equivalence | Proof Mapping Framework V1 |
| Mutation Integrity | WC-002, declared read/write sets, transition integrity checks | Formal Proof Target Package V1 |
| Receipt Integrity | WC-003, canonical receipt schema, mutation boundary receipts | Canonicalizer Kernel |
| Authority Enforcement | WC-006, authority declarations, deterministic identity and role checks | Formal Proof Target Package V1 |
| Upgrade Safety | WC-007, versioned contracts, bounded migrations, continuity roots | Proof Mapping Framework V1 |

The framework is designed so certification outputs can become proof inputs. Emitter declarations, canonical input logs, deterministic receipts, and state roots define the proof boundary for future formal verification.

## Certification Output

A successful v1 documentation certification emits:

```text
WORLD CONTRACT CERTIFICATION FRAMEWORK V1: PASS
```
