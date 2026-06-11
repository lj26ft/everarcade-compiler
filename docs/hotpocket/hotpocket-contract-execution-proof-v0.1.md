# HotPocket Contract Execution Proof v0.1

## Scope

This milestone is limited to real HotPocket contract execution for the EverArcade adapter path:

`Client → HotPocket consensus → contract invocation → EverArcade adapter → deterministic receipt/journal/checkpoint → client output`.

It does **not** claim Evernode lease hosting, WAN federation, XRPL settlement, Xahau settlement, multiplayer at scale, production economics, or civilization hosting.

## Repository Layout

- `runtime/hotpocket-contract-proof/contract/` contains the HotPocket smart contract entrypoint.
- `runtime/hotpocket-contract-proof/client/` contains the canonical websocket round-trip client.
- `runtime/hotpocket-contract-proof/src/` contains deterministic artifact generation shared by the contract and validators.
- `runtime/hotpocket-contract-proof/validation/` contains the proof/report generator.
- `runtime/hotpocket-contract-proof/reports/` receives proof-local reports; scripts also mirror reports to repository-level `reports/`.
- `scripts/validate_hotpocket_execution.sh` is the single validation command.
- `scripts/certify_hotpocket_execution.sh` writes the final certification report.

## SDK Compatibility Findings

The discovery validator records the actual installed SDK exports in `reports/hotpocket_execution_sdk_report.json`. The proof targets these observed APIs:

- Contract SDK package: `hotpocket-nodejs-contract`.
- Contract registration: `new HotPocket.Contract().init(async (ctx) => { ... }, HotPocket.clientProtocols.json)`.
- Client SDK package: `hotpocket-js-client`.
- Client creation: `HotPocket.createClient(servers, keys, options)`.
- Client input: `client.submitContractInput(JSON.stringify(payload), nonce, maxLedger, isOffset)`.
- Client output: `HotPocket.events.contractOutput`.

The contract also writes `hotpocket_live_context_report.json` during real invocation so callback shape and context object details come from the live runtime rather than from assumptions.

## Deployment

Run the proof through one command:

```bash
bash scripts/validate_hotpocket_execution.sh
```

The command installs the proof package dependencies with `npm install --prefix runtime/hotpocket-contract-proof` when needed. It does not require manual container editing, `docker exec`, manual `bin_path` edits, or manual `node_modules` fixes.

A live cluster must provide websocket endpoints through either:

```bash
export HOTPOCKET_SERVERS="ws://127.0.0.1:8081,ws://127.0.0.1:8082,ws://127.0.0.1:8083"
```

or:

```bash
export HP_SERVERS="ws://127.0.0.1:8081,ws://127.0.0.1:8082,ws://127.0.0.1:8083"
```

The contract entrypoint for cluster deployment is:

```text
runtime/hotpocket-contract-proof/contract/index.js
```

## Consensus Flow

The proof requires a three-node HotPocket cluster. The consensus proof report explicitly records:

- configured server count,
- proposal creation status,
- vote participation status,
- consensus finalization status,
- output delivery status,
- absence of `max_ledger_expired`, and
- absence of `Not enough peers proposing`.

Report: `reports/hotpocket_consensus_execution_report.txt`.

## Execution Flow

The canonical ping input is:

```json
{ "action": "ping" }
```

The required output is:

```json
{ "status": "ok" }
```

Report: `reports/hotpocket_ping_execution_report.txt`.

## Client Interaction

The canonical client:

1. loads the HotPocket websocket endpoints,
2. generates or reads client keys,
3. connects to the cluster,
4. submits canonical JSON input,
5. waits for `contractOutput`,
6. verifies the returned payload, and
7. records submission hash, submission status, output payload, and completion time.

Report: `reports/hotpocket_client_roundtrip_report.txt`.

## Receipts

Every successful execution produces a canonical receipt containing:

- input hash,
- state root,
- receipt root,
- timestamp,
- execution id,
- status, and
- output.

Report: `reports/hotpocket_receipt_execution_report.txt`.

## Journals

The journal is append-only and records sequence number, input hash, mutation hash, before root, after root, and deterministic content hash.

Report: `reports/hotpocket_journal_execution_report.txt`.

## Replay

Replay proof executes the identical deterministic sequence twice and verifies:

- `state_root_a == state_root_b`,
- `receipt_root_a == receipt_root_b`, and
- `journal_root_a == journal_root_b`.

Report: `reports/hotpocket_replay_execution_report.txt`.

## Deterministic Mutation

The canonical mutation input is:

```json
{ "action": "join_player", "player_id": "player-1" }
```

The mutation is:

```text
player_count += 1
```

Report: `reports/hotpocket_mutation_execution_report.txt`.

## Failure Modes

The validator fails rather than downgrading to a script-only proof when:

- no HotPocket websocket server list is configured,
- the client cannot connect,
- input submission fails,
- output is not delivered,
- output payload does not verify,
- consensus error text contains `max_ledger_expired`,
- consensus error text contains `Not enough peers proposing`, or
- SDK packages cannot be inspected.

A deterministic self-test exists for local syntax and artifact checks, but it is explicitly not a live execution proof and is not used for certification.

## Certification

Run:

```bash
bash scripts/certify_hotpocket_execution.sh
```

The final certification line is written only from live validation state:

```text
HotPocket Contract Execution Proof v0.1: PASS
```
