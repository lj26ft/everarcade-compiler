# Network Transport & Session Synchronization Proof v0.1

## Summary

This proof demonstrates that two independent local client identities can exchange deterministic transport messages with an authoritative runtime and converge on the same session state. The final classification is:

```text
Network Transport & Session Synchronization Proven
```

## Transport model

The canonical local transport emits ordered messages with these message types:

- `JoinSession`
- `MovePlayer`
- `AttackPlayer`
- `SessionState`
- `Receipt`
- `Heartbeat`

Every transport message includes:

- `message_id`
- `session_id`
- `sequence`
- `sender`
- `payload`
- `hash`

Messages are serialized as deterministic JSON from runtime-owned structures. The message hash binds the message identity, type, session, sequence, sender, and payload.

## Authority model

The runtime is authoritative. Clients may submit transport intents, but clients do not directly mutate state and do not generate authoritative receipts, checkpoints, or replay roots.

Runtime-owned artifacts include:

- authoritative arena state
- journal entries
- receipts
- checkpoint evidence
- replay proof
- session transcript

Client artifacts are observations of the runtime state after synchronization.

## Client lifecycle

The local proof starts two client identities:

- Client A (`client-a`, `player-a`)
- Client B (`client-b`, `player-b`)

Client A joins, moves, attacks Client B, receives state updates, receives receipts, and acknowledges receipts. Client B joins, moves, observes the attack interaction, receives state updates, receives receipts, and acknowledges receipts.

## Synchronization model

Synchronization succeeds when the following fields match across Client A, Client B, and the runtime:

- `tick`
- `state_root`
- `player_count`

The proof writes runtime evidence under `network/`:

- `client-a-state.json`
- `client-b-state.json`
- `runtime-state.json`
- `transport-log.json`
- `session-transcript.json`
- `receipt-delivery.json`

The ordered session transcript is also written to `sessions/network-session-transcript.json`.

## Receipt delivery

Receipts are generated only by the runtime. The proof records delivery and acknowledgement for each receipt in `network/receipt-delivery.json`, proving:

```text
Receipt generated
Receipt delivered
Receipt acknowledged
```

for both Client A and Client B.

## Journal observation

The runtime journal remains authoritative. Clients record the observed journal length after synchronization, proving that clients observe journal progression without modifying journal entries.

## Multi-tick progression

The proof advances through at least ticks 1 through 5 with joins, movement, and interaction. Both clients remain synchronized after multi-tick progression.

## Replay model

Replay is generated from authoritative journal events. The replay proof is written to:

```text
replay/network-session-replay-proof.json
```

Replay verification passes only when:

```text
Replay root = Session root
```

Expected replay status:

```text
Network Replay Verification: PASS
```

## CLI command

Run the proof with:

```bash
node creator-sdk/cli/everarcade.mjs play-network-local --template arena
```

Expected output:

```text
Network Transport Session: PASS
```

## Validation and certification

Validation:

```bash
bash scripts/validate_network_transport_session.sh
```

Certification:

```bash
bash scripts/certify_network_transport_session.sh
```

Expected final certification line:

```text
Network Transport & Session Synchronization Proof v0.1: PASS
```

## Limitations

This proves local network-style transport and synchronization.

It does not prove:

- federation,
- Evernode deployment,
- WAN networking,
- multi-lease coordination,
- or production transport reliability.
