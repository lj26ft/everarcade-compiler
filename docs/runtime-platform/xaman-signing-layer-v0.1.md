# Xaman Signing Layer v0.1

## Purpose

Xaman Signing Layer v0.1 adds a human authorization boundary between the EverArcade Runtime settlement model and XRPL settlement evidence.

The layer is deterministic and representational. It creates Xaman-compatible payload metadata, deep-link metadata, QR metadata, tracking state, signed receipt imports, and settlement continuity roots. It does not store private keys, custody wallets, sign transactions, submit production funds, use multi-sign, deploy Hooks, or call live Xaman APIs.

EverArcade remains authoritative for protocol state. Xaman authorizes transactions.

## Payload Model

A Xaman payload binds:

- payload id;
- wallet;
- transaction root;
- settlement root;
- reference data;
- timestamp;
- signing authority marker;
- runtime authority marker;
- custody marker.

The payload root is a SHA-256 digest over canonical field order. The payload id is derived from the intent root, transaction root, settlement root, wallet, reference data, and timestamp before the payload root is generated.

## Deep Link Model

The deep-link model represents:

- payload identifier;
- request identifier;
- transaction reference;
- transaction root;
- deterministic request URL metadata;
- request timestamp.

The request URL is metadata only. No network call is made. The deep link root is a SHA-256 digest of the canonical deep-link transcript.

## QR Model

The QR metadata model contains:

- payload id;
- reference data;
- request hash;
- encoding marker;
- network-call marker.

The QR root is a SHA-256 digest over canonical QR metadata. QR metadata is intentionally separate from live QR rendering so replay does not depend on image encoders or API availability.

## Tracking Model

Tracking state supports:

```text
Created
Pending
Approved
Rejected
Expired
```

The v0.1 fixture records an approved path while still declaring the full allowed state set. The tracking root commits to the supported states, state timestamps, current status, and payload id.

## Receipt Model

The signed receipt import captures:

- payload id;
- transaction hash;
- wallet;
- outcome;
- timestamp;
- payload root;
- transaction root;
- settlement root.

The signed receipt root is deterministic settlement evidence that a user approval was imported. It is not a private-key signature and does not make Xaman authoritative for runtime state.

## Continuity Model

The continuity update integrates:

```text
Intent + Transaction + Payload + Receipt
```

It validates:

- receipt matches payload;
- payload matches transaction;
- transaction matches intent.

The settlement continuity root is a SHA-256 digest over the intent root, transaction root, payload root, and signed receipt root. Replay must reproduce that same root.

## PASS Criteria

The layer passes when:

- payload construction succeeds;
- deep-link metadata construction succeeds;
- QR metadata construction succeeds;
- tracking state supports all required states;
- signed receipt import succeeds;
- continuity update succeeds;
- replay root equals settlement continuity root;
- validation report says `Xaman Signing Validation: PASS`;
- certification report says `Xaman Signing Layer: PASS`.

## FAIL Criteria

The layer fails if:

- canonical field order changes unexpectedly;
- payload id, wallet, transaction root, settlement root, reference data, or timestamp is missing;
- deep-link request data omits payload, request, or transaction references;
- QR metadata omits payload id, reference data, or request hash;
- tracking omits any required state;
- receipt payload id, transaction hash, wallet, outcome, or timestamp is invalid;
- receipt import cannot match the payload;
- payload cannot match the transaction;
- transaction cannot match the intent;
- replay root differs from settlement continuity root.

## Relationship To XRPL Live Settlement

XRPL Live Settlement Layer v0.1 creates the deterministic settlement intent, unsigned XRPL transaction representation, transaction hash, receipt evidence, and settlement root.

Xaman Signing Layer v0.1 consumes that unsigned transaction representation and creates the authorization artifacts around it. The XRPL transaction remains deterministic, while Xaman provides human approval metadata before signed receipt import and continuity update.

## Relationship To Future Public Testnet

This v0.1 layer is ready to be replaced at the boundary by public-testnet API calls because the deterministic transcript already records payload ids, request ids, request hashes, receipt fields, and continuity roots.

Future public-testnet work may add live payload creation, mobile approval, and transaction submission. Those additions must preserve the same replay contract: given intent, transaction, payload, and receipt, replay reproduces the settlement continuity root.
