# World Factory MVP Phase 4 — Live Host Deployment Proof

Phase 4 extends the World Factory proof chain from local generation and remote verification to a live host deployment proof. It demonstrates that the generated `frontier-settlement` world package can be bundled for an existing operator host, served by the small deterministic runtime wrapper, and independently checked from another machine.

```text
Prompt
↓
Blueprint
↓
Contract Plan
↓
world.evr
↓
Runtime
↓
Replay
↓
Remote Verification
↓
Attestation
↓
Live Host Deployment
↓
Deployment Proof
```

## Scope

Phase 4 uses the existing generated world:

```text
examples/world-factory/frontier-settlement/out/world.evr
```

It does not introduce a new world type. The runtime remains the scaffold-level `small` deterministic World Factory runtime used by Phases 2 and 3.

## Deployment bundle

`everarcade world factory deploy` prepares `out/deploy/` with:

- `world.evr/` — generated world package.
- `runtime-config.json` — endpoint, host, and port configuration.
- `deployment-report.json` — local deployment readiness and commitment roots.
- `deployment-manifest.json` — shareable deployment manifest.
- `world-release-attestation.json` — copied from the release output when available.
- `release-report.json` — copied from the release output when available.
- `evidence/` — public evidence bundle containing the manifest, proof, attestation, and release report when available.

The deployment manifest includes:

```json
{
  "world_id": "...",
  "package_hash": "...",
  "world_hash": "...",
  "continuity_root": "...",
  "deployment_target": "...",
  "attestation_hash": "...",
  "verification_status": "PASS"
}
```

## Publish command

```bash
everarcade world factory publish --host <host>
```

`publish` prepares the same deployment package, writes the deployment manifest, and emits manual upload/run/verify instructions for an existing host. It intentionally does not provision infrastructure or leases.

The upload instructions are written to:

```text
out/deploy/upload-instructions.json
```

## Live runtime command

```bash
everarcade world factory serve --host 0.0.0.0 --port 8787
```

The server exposes:

- `/health`
- `/state`
- `/journal`
- `/verify`

`/verify` includes state, receipts, and computed roots so a verifier can recompute the commitments locally instead of trusting server status strings.

## Proof command

From an independent machine with the public evidence bundle and generated package available:

```bash
everarcade world factory proof --url http://<host>:8787
```

The proof command fetches `/health`, `/state`, `/journal`, and `/verify`, then recomputes:

- `state_root`
- `receipt_root`
- `world_hash`
- `continuity_root`

It verifies:

- package verification
- replay verification
- remote verification
- attestation verification

The generated proof report is:

```text
out/deploy/live-deployment-proof.json
```

with the shape:

```json
{
  "world_id": "...",
  "host": "http://<host>:8787",
  "deployment_status": "RUNNING",
  "package_verification": "PASS",
  "replay_verification": "PASS",
  "remote_verification": "PASS",
  "attestation_verification": "PASS",
  "proof_timestamp": "..."
}
```

## End-to-end demonstration

```bash
everarcade world factory generate
everarcade world factory run --ticks 100
everarcade world factory replay
everarcade world factory deploy
everarcade world attest create
everarcade world factory serve --host 0.0.0.0 --port 8787
```

From a separate machine:

```bash
everarcade world factory proof --url http://<host>:8787
```

Expected output:

```text
PASS
```

## Proof boundaries

Phase 4 truthfully demonstrates remote host deployment, remote runtime observation, independent commitment verification, deployment proof generation, and release attestation verification.

It does **not** claim:

- EverNode consensus
- multi-host federation
- automatic lease provisioning
- production MMO scale

## Future hooks

These are documented but not implemented in Phase 4:

- EverNode lease provisioning
- one-click deployment
- Operator Registry integration
- XRPL/Xahau attestation anchoring
- federated world clusters
