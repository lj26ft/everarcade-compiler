> **Repository boundary:** This repository does not implement custody, wallet management, live settlement, or legal ownership certification.
>
> XRPL/Xahau material is limited to deterministic anchoring, boundary modeling, or local test scaffolds unless explicitly stated otherwise.
>
# Xahau / XRPL Settlement Boundary

## Rule

EverArcade does not implement live vault custody or live XRPL/Xahau signing inside the HotPocket authority path. The runtime emits deterministic intent and anchor records. An external settlement service performs signing, submission, retry, confirmation, and vault lifecycle operations.

## EverArcade emits

```text
XRPLIntentRecord
ReceiptAnchorRecord
ReplayAnchorRecord
CheckpointAnchorRecord
VaultIntentRecord
```

| Record | Purpose | Authority status |
| --- | --- | --- |
| `XRPLIntentRecord` | Declares that an external XRPL/Xahau action is requested for a canonical payload/root. | Consensus intent; not a signed transaction. |
| `ReceiptAnchorRecord` | Requests or records notarization of execution/package/deployment receipts. | Consensus anchor intent and verified receipt facts. |
| `ReplayAnchorRecord` | Requests or records notarization of replay window roots. | Consensus anchor intent and verified receipt facts. |
| `CheckpointAnchorRecord` | Requests or records notarization of checkpoint and lineage roots. | Consensus anchor intent and verified receipt facts. |
| `VaultIntentRecord` | Declares desired vault lifecycle or settlement action. | Consensus intent only; custody and execution are external. |

## External settlement service performs

```text
signing
submission
retry
settlement
vault lifecycle
```

The external service owns:

- wallet keys, seeds, hardware-signing integrations, and signer authorization;
- XRPL/Xahau transaction construction details;
- submission timing, fee selection, retry/backoff, and network failover;
- transaction confirmation, finality checks, and reorg/failure handling;
- vault creation, funding, policy changes, closure, and recovery;
- operational logs and alerting for settlement workers.

## Data flow

1. Runtime creates a deterministic intent record under HotPocket authority.
2. Replay stores the intent as part of the canonical history.
3. External settlement service reads intents from an approved export/API.
4. External service signs and submits transactions outside HotPocket authority.
5. External service returns a settlement receipt containing transaction id, ledger/index data, result, and anchored root.
6. Runtime validates the receipt facts against the original intent and stores a verified anchor receipt record.

## Prohibited authority writes

The following must never be written into HotPocket authority:

- private keys, mnemonics, seeds, signer configs, or HSM session material;
- unsigned mutable transaction queues driven by wall-clock scheduling;
- fee-bidding state or retry counters that are not deterministic records;
- external service process state, HTTP client caches, or node RPC responses not reduced to verified receipt facts.

## Launch posture

The correct live boundary is intent-in-authority and custody-outside-authority. Implementing live vaults belongs in an external settlement service and should be tested independently before connecting it to production EverArcade deployments.
