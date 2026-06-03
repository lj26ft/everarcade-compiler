# Civilization Runtime Certification v0.1

## Purpose

The Civilization Runtime Certification validates that the EverArcade Runtime Platform can run a deterministic, replayable civilization lifecycle as a single system-of-systems. It integrates previously certified runtime domains into one continuity proof: genesis, treasury creation, citizen creation, asset distribution, inventory allocation, vault allocation, marketplace activity, economic activity, governance, policy activation, checkpointing, restoration, and replay equivalence.

This certification is scoped to runtime civilization operation only. It does not certify XRPL settlement, Xaman authority, hooks, federated consensus, cross-chain settlement, or external governance.

## Civilization Model

The certified civilization is `Civilization Alpha`. Its deterministic identity set contains one treasury, two governors, and three citizens:

- `civilization-treasury`
- `governor-a`
- `governor-b`
- `citizen-a`
- `citizen-b`
- `citizen-c`

The script rejects duplicate or empty identities. The civilization root is derived from a canonical transcript containing identities, accounts, authorities, inventories, vaults, balances, ownership, marketplace offers, governance state, and ordered events.

## Treasury Model

The treasury initializes the civilization economy with exactly `10,000 Gold`. The treasury owns a deterministic vault and inventory record. Treasury integrity requires total account balances to equal the original supply at every certified phase, including after transfers, marketplace settlement fees, policy enforcement, checkpointing, restoration, and replay.

## Citizen Model

Citizens and governors receive deterministic authority, inventory, and vault records. Citizens receive initial assets and balances. Citizen integrity requires:

- every account has a canonical authority record;
- every account has a canonical inventory record;
- every account has a canonical vault record;
- every asset is owned by exactly one valid account;
- no account balance is negative.

## Economic Model

Economic activity uses deterministic Gold transfers. The certified flow distributes funds from the treasury to citizens, then performs citizen-to-citizen transfers. Economy validation checks balance conservation and rejects inflation by comparing total balances with the initial treasury supply.

## Marketplace Model

Marketplace activity uses deterministic offer creation, acceptance, settlement, and ownership transfer. A seller may create an offer only for an asset it owns. A buyer must have sufficient funds. Settlement transfers the asset to the buyer, sends the fee to the treasury, sends the remaining amount to the seller, and closes the offer.

## Governance Model

Governance creates a deterministic proposal to increase the marketplace fee. `governor-a` and `governor-b` both vote `YES`. The approval threshold is two affirmative votes, so the proposal becomes eligible for activation only after both votes are recorded in canonical order.

## Policy Enforcement Model

The certified policy changes the marketplace fee from `5%` (`500` basis points) to `7%` (`700` basis points). After activation, the certification executes another marketplace transaction and verifies that the updated fee is credited to the treasury and reflected in settlement state.

## Checkpoint Model

Checkpointing captures the canonical civilization transcript after epoch evolution. The checkpoint identifier is derived from the checkpoint transcript hash. Checkpoint validation requires the checkpoint transcript to exist and match the current runtime state root before restoration continues.

## Replay Model

Replay reruns the full lifecycle from genesis through continued post-restoration activity:

1. civilization genesis;
2. treasury initialization;
3. citizen initialization;
4. economic transfers;
5. marketplace settlement;
6. governance proposal and voting;
7. policy activation;
8. policy-enforced marketplace settlement;
9. civilization evolution;
10. checkpoint creation;
11. restoration validation;
12. continued civilization activity.

The replay civilization root must exactly equal the original civilization continuity root.

## Civilization Integrity Rules

The certification validates these continuity rules:

- treasury balances conserve the initial Gold supply;
- citizen identities are unique and stable;
- authority records are deterministic and restored;
- inventory records are deterministic and restored;
- vault records are deterministic and restored;
- asset ownership changes only through authorized flows;
- marketplace settlement applies the active policy fee;
- governance policy activates only after threshold approval;
- checkpoint state matches restored state;
- replay root equals the original continuity root.

## PASS Criteria

The certification passes only when all of the following statuses are `PASS`:

- Bootstrap
- Treasury
- Citizens
- Economy
- Marketplace
- Governance
- Policy Enforcement
- Checkpoint
- Restoration
- Replay
- Integrity
- Civilization Runtime Certification

The report must include all required roots, checkpoint identifier, continuity root, replay root, subsystem statuses, integrity status, and overall result.

## FAIL Criteria

The certification fails if any of these occur:

- runtime bootstrap is unavailable;
- identities are duplicated;
- treasury supply diverges;
- balances inflate or become negative;
- asset ownership is invalid;
- inventory, vault, or authority records diverge;
- marketplace settlement violates ownership or balance rules;
- activated policy does not affect later settlement;
- governance threshold is not enforced;
- checkpoint state cannot be verified;
- restored state diverges;
- replay root differs from the original continuity root.

## Relationship To Governance Certification

Governance Authority Certification proves the governance subsystem in isolation: proposal creation, voting, approval, activation, enforcement, checkpointing, restoration, and replay. Civilization Runtime Certification consumes that pattern inside a broader civilization lifecycle and validates that governance can operate alongside treasury, citizen, economy, marketplace, authority, inventory, and vault state without cross-system divergence.

## Relationship To Future XRPL Settlement Certification

This certification intentionally excludes XRPL, Xaman, hooks, federation, cross-chain activity, and external settlement. Future XRPL settlement certifications can use the civilization continuity root and deterministic policy state as prerequisites, then validate external authority and settlement without changing this runtime-only proof.
