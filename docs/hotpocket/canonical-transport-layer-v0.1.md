# Canonical HotPocket Transport Layer v0.1

`runtime/hotpocket-transport` is the canonical EverArcade adapter boundary between HotPocket/Evernode and the deterministic EverArcade runtime.

HotPocket provides lease execution, input delivery, ledger context, and contract output transport. EverArcade owns `MutationEnvelope`, deterministic execution, `state_root`, `replay_root`, `receipt_root`, and `continuity_root`.

## Boundary

```text
HotPocket ctx
  -> extract users / inputs inside contract adapter only
  -> TransportSubmission
  -> MutationEnvelope
  -> deterministic EverArcade runtime bridge
  -> TransportReceipt
  -> HotPocket user output
```

HotPocket-specific objects such as `ctx`, `ctx.users`, input references, and user instances must never cross into runtime mutation logic. The runtime bridge accepts only canonical envelopes.

## Canonical Structures

The JavaScript implementation mirrors the protocol structures:

- `TransportSubmission`: transport id, lease id, contract id, user public key, input hash, nonce, optional ledger sequence, and raw payload bytes.
- `MutationEnvelope`: world id, player id, mutation type, canonical payload bytes, submission hash, and source transport.
- `TransportReceipt`: protocol id, world id, input hash, `state_root`, `replay_root`, `receipt_root`, `continuity_root`, optional ledger sequence, and success flag.

## Client Acceptance Rule

Clients must await HotPocket submission acceptance before considering an input live:

```js
const result = await client.submitContractInput(payload);
const status = await result.submissionStatus;
```

Rejected or missing statuses are quarantined and logged with input hash, status, reason, server, and ledger information when available.

## Mainnet Proof Artifact Baseline

The live Evernode validation artifacts remain the baseline evidence for this milestone:

- acquire lease
- bundle deploy
- client connect
- submission accepted
- ledger event received
- contract output received
- adapter investigation logs

Existing proof reports under `reports/` and HotPocket proof workspaces remain historical evidence. New transport validation extends that baseline by proving that accepted submissions normalize into canonical envelopes and deterministic receipts without leaking HotPocket runtime objects into EverArcade runtime logic.
