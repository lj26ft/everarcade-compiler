# XRPL Live Settlement Layer v0.1

## Purpose

XRPL Live Settlement Layer v0.1 establishes the runtime boundary between EverArcade settlement intent, unsigned XRPL transaction representation, XRPL receipt evidence, and protocol continuity anchoring.

The layer is intentionally representational. It does not call Xaman, sign with production wallets, submit transactions, configure federation networking, use multi-sign, deploy Hooks, or change XRPL consensus. XRPL validates external settlement; EverArcade continuity remains authoritative.

## Intent Model

A settlement intent is a deterministic runtime record with:

- intent id;
- asset;
- amount in drops;
- sender account;
- recipient account;
- timestamp;
- runtime reference.

The intent root is a SHA-256 digest over the canonical field order. Stable ordering makes the intent replay safe and allows the same root to be regenerated on any node.

## Transaction Model

The transaction builder converts the intent into an unsigned XRPL `Payment` representation with:

- `TransactionType=Payment`;
- `Account` from the sender;
- `Destination` from the recipient;
- `Amount` from the intent amount;
- deterministic fee, sequence, and last-ledger fields for representation;
- memo type, data, and format;
- reference fields for intent id, intent root, and runtime reference.

The memo data carries the intent root as hexadecimal bytes. The transaction root is a SHA-256 digest over the canonical unsigned transaction payload.

## Verification Model

Verification checks that:

- the transaction structure is a payment representation;
- account and destination fields match the intent;
- amount fields match the intent;
- memo type, memo data, and memo format preserve intent integrity;
- reference fields preserve intent id, intent root, and runtime reference.

The verification root records the verification transcript and transaction root.

## Receipt Model

The receipt model captures deterministic XRPL settlement evidence:

- transaction hash;
- ledger index;
- account;
- destination;
- amount;
- outcome;
- timestamp;
- transaction root;
- intent root.

The receipt root is a SHA-256 digest over the canonical receipt payload.

## Import Model

Receipt import converts the receipt into EverArcade settlement evidence. Import passes only when:

- receipt integrity is valid;
- the receipt transaction hash matches the represented transaction;
- account, destination, and amount match the transaction;
- transaction references match the original intent.

The settlement evidence root binds the intent root, transaction root, receipt root, and settlement root.

## Anchor Model

The continuity anchor binds:

- EverArcade continuity root;
- settlement root;
- receipt root;
- settlement evidence root.

The anchor root is deterministic evidence that a continuity state observed a specific XRPL settlement receipt without making XRPL authoritative for protocol state.

## Replay Model

Replay combines:

```text
Intent + Transaction + Receipt
```

and regenerates:

```text
Settlement Root
```

Replay passes only when:

```text
Replay Root == Settlement Root
```

This preserves protocol continuity under deterministic replay.

## PASS Criteria

The layer passes when:

- intent construction succeeds;
- transaction construction succeeds;
- transaction verification succeeds;
- receipt representation succeeds;
- receipt import succeeds;
- continuity anchor generation succeeds;
- replay root equals settlement root;
- validation report says `XRPL Live Settlement Validation: PASS`;
- certification report says `XRPL Live Settlement Layer: PASS`.

## FAIL Criteria

The layer fails if:

- serialization order is unstable;
- intent fields are missing or mismatched;
- transaction account, destination, amount, memo, or reference fields diverge from the intent;
- receipt hash, ledger, account, destination, amount, outcome, or timestamp is invalid;
- receipt import cannot match the transaction and intent;
- anchor generation omits continuity, settlement, receipt, or evidence roots;
- replay root differs from settlement root.

## Relationship To XRPL Settlement Certification

XRPL Settlement Certification proves deterministic settlement modeling, authority linkage, receipts, checkpoint restoration, and replay equivalence without live XRPL semantics.

XRPL Live Settlement Layer v0.1 builds on that certification by adding XRPL-aware transaction and receipt representations. It keeps the same sovereignty rule: XRPL provides settlement evidence, while EverArcade continuity owns protocol state.

## Relationship To Future Xaman Signing Layer

Future Xaman Signing Layer v0.1 can consume the unsigned transaction representation created here. Signing and submission can be added after this layer without changing intent roots, transaction roots, receipt roots, settlement roots, or replay rules.
