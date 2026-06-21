# HotPocket Integration Layer v0.1

## Purpose

HotPocket Integration Layer v0.1 establishes the runtime boundary between the EverArcade Protocol Node Appliance and a HotPocket contract container. It lets a node run under a HotPocket execution environment while preserving the protocol node as the only authority over deterministic state.

This milestone is an adapter milestone only. It does not add consensus, federation networking, Evernode deployment, XRPL settlement, Xaman signing, GPU execution, renderer execution, or protocol logic.

## Runtime Boundary

```text
EverArcade Protocol Node
  owns deterministic protocol state
  owns mutation paths
  owns checkpoint, replay, continuity, and settlement roots

HotPocket Adapter
  reads HotPocket-facing inputs
  forwards and exports deterministic envelopes
  publishes roots and reports for the HotPocket container

HotPocket Runtime
  owns the execution environment
  does not own protocol authority
```

HotPocket is not authoritative. The adapter writes only HotPocket integration artifacts under `hotpocket/` and reports under `reports/`. Protocol mutations remain inside existing node lifecycle scripts and the Protocol Node Appliance.

## Canonical Runtime Layout

```text
hotpocket/
  adapter/      HotPocket-to-EverArcade boundary scripts; no protocol logic.
  input/        Deterministic input envelopes and input_root.
  output/       Deterministic output envelopes and output_root.
  checkpoint/   Checkpoint export envelopes and checkpoint_export_root.
  replay/       Replay export envelopes and replay_export_root.
  settlement/   Settlement export envelopes and settlement_export_root.
  status/       Runtime status envelopes and status_root.
```

The canonical adapter entrypoint is:

```bash
bash hotpocket/adapter/hotpocket_adapter.sh <command>
```

## Adapter Model

The adapter is a bridge from HotPocket runtime inputs to EverArcade node artifacts. Its responsibilities are:

- Input ingestion.
- Output export.
- Checkpoint export.
- Replay export.
- Settlement export.
- Status export.

The adapter may not modify protocol state, replay roots, continuity roots, settlement roots, or authority metadata. It publishes deterministic envelopes so HotPocket can observe and transport integration data without becoming a protocol state owner.

## Input Model

Input envelopes are written under `hotpocket/input/` with this canonical field order:

```text
Envelope Type
Input Identifier
Timestamp
Origin
Payload
Payload Hash
Serialization
Ordering
Replay Safe
Hash
```

The envelope hash is computed from the canonical fields before the `Hash` line is appended. `input_root` is computed from sorted input-envelope hashes. Ordering is lexicographic by input identifier, making replay deterministic and stable.

## Output Model

Output envelopes are written under `hotpocket/output/` with this canonical field order:

```text
Envelope Type
Output Identifier
Result
Status
Root
Serialization
Ordering
Hash
```

The output root is computed from sorted output-envelope hashes. Outputs are integration exports only and do not mutate node state.

## Checkpoint Export Model

Checkpoint export envelopes are written under `hotpocket/checkpoint/` and expose:

- Checkpoint identifier.
- Checkpoint path.
- Continuity root.
- Checkpoint hash.
- EverArcade Protocol Node authority marker.
- HotPocket mutation prohibition marker.

`checkpoint_export_root` is computed from sorted checkpoint export hashes. The checkpoint hash is derived from the existing protocol-node checkpoint when present, otherwise from a deterministic genesis export transcript.

## Replay Export Model

Replay export envelopes are written under `hotpocket/replay/` and expose:

- Replay identifier.
- Replay root.
- Replay hash.
- Replay-safe marker.
- HotPocket replay-root mutation prohibition marker.

`replay_export_root` is computed from sorted replay export hashes.

## Settlement Export Model

Settlement export envelopes are written under `hotpocket/settlement/` and expose:

- Settlement identifier.
- Settlement root.
- Settlement hash.
- XRPL execution disabled marker.
- Signing disabled marker.
- Networking disabled marker.
- HotPocket settlement-root mutation prohibition marker.

This layer exports settlement artifacts only. It does not execute XRPL RPC, Hooks, signing, Xaman flows, networking, or live settlement.

## Status Export Model

Status export envelopes are written under `hotpocket/status/` and expose:

- Node status.
- Checkpoint status.
- Replay status.
- World status.
- Adapter status.
- HotPocket authority scope.

`status_root` is computed from sorted status export hashes.

## PASS Criteria

HotPocket Integration Layer v0.1 passes when:

- The canonical `hotpocket/` layout exists.
- The adapter script exists and runs.
- Input envelopes are deterministic, ordered, replay-safe, and root-producing.
- Output envelopes are deterministic, ordered, and root-producing.
- Checkpoint exports include checkpoint identifier, continuity root, checkpoint hash, and export root.
- Replay exports include replay identifier, replay root, replay hash, and export root.
- Settlement exports include settlement identifier, settlement root, settlement hash, and no live XRPL/signing/networking behavior.
- Status exports include node, checkpoint, replay, and world status plus a status root.
- Validation produces `reports/hotpocket_integration_validation_report.txt` with all integration checks passing.
- Certification produces `reports/hotpocket_integration_certification_report.txt` with `HotPocket Integration: PASS`.

## FAIL Criteria

The integration fails if:

- HotPocket adapter artifacts mutate protocol state directly.
- HotPocket becomes authoritative for protocol roots.
- Input, output, checkpoint, replay, settlement, or status envelopes lack deterministic hashes or roots.
- Settlement export performs XRPL RPC, signing, networking, Hooks, or live settlement.
- Validation or certification reports any required layer as failing.

## Relationship To Protocol Node Appliance

The Protocol Node Appliance remains the owner of node lifecycle, deterministic state, checkpoints, replay roots, continuity roots, and settlement roots. The HotPocket adapter observes and exports those artifacts without changing their authority model.

Validation may prepare node artifacts by calling existing Protocol Node Appliance scripts. Those mutations flow through the protocol node, not through HotPocket.

## Relationship To Future Evernode Deployment Layer

This milestone is a prerequisite for an Evernode deployment layer. Future work may package this adapter inside a deployed HotPocket contract container, add Evernode-specific deployment metadata, and connect live operator workflows.

Those future layers must preserve this boundary: HotPocket and Evernode may provide execution and deployment surfaces, but deterministic protocol state remains sovereign to EverArcade.
