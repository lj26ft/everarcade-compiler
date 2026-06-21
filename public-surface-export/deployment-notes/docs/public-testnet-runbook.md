# EverArcade Public Testnet v0.1 Runbook

This runbook describes how external participants operate the EverArcade Public Testnet. The network is testnet-only and must not use production XRPL funds, production revenue, or production asset value.

## Joining Testnet

1. Read `docs/runtime-platform/public-testnet-v0.1.md`.
2. Select a participant role: developer, lease operator, node operator, federation operator, GPU provider, player, or governance participant.
3. Submit the role-specific registration record to the testnet coordinator.
4. Wait for the next deterministic enrollment snapshot.
5. Confirm the generated enrollment root appears in `public-testnet/records/roots.env`.

## Deploying Games

1. Register the developer identity in `public-testnet/developers/developer_enrollment.records`.
2. Register a project with a project ID, name, certified runtime version, and expected surfaces.
3. Obtain project approval for public-testnet execution.
4. Bind the project to a lease in `public-testnet/deployments/deployment_registry.records`.
5. Confirm deployment status transitions to `running` or `validating`.
6. Preserve deployment records for replay and certification.

## Running A Lease

1. Register as a lease operator, node operator, or federation operator.
2. Publish node identity, lease ID, federation membership, checkpoint policy, and replay location.
3. Run only the certified public-testnet runtime stack.
4. Emit operator records for health, checkpoint, replay, and federation membership.
5. Report degraded health immediately through governance or failure reporting.

## Registering A GPU

1. Register the GPU provider identity.
2. Advertise deterministic capability class, certified driver stack, and supported job types.
3. Declare capacity and maximum parallel jobs.
4. Accept only testnet projection jobs.
5. Emit artifact, verification, replay, and settlement-intent records.
6. Never require production funds or commercial billing.

## Participating In Governance

1. Create proposals for testnet-only policy or rule changes.
2. Cast votes using testnet participant identities.
3. Record proposal, vote, policy, and rule-change entries.
4. Ensure all changes are auditable and replayable.
5. Use governance to adjust windows, capacity limits, test cohorts, and operational policies.

## Reporting Failures

1. Capture the failing participant role, project, deployment, lease, GPU job, settlement intent, or governance proposal.
2. Include checkpoint, replay ID, observed status, and expected status.
3. Classify the issue as enrollment, deployment, civilization, GPU marketplace, settlement, governance, analytics, or replay.
4. Attach logs and deterministic record snippets where possible.
5. Keep reports testnet-only and omit private keys or production credentials.

## Recovery Procedures

1. Stop the affected testnet component.
2. Preserve the latest checkpoint and replay artifact.
3. Re-run `bash scripts/validate_public_testnet.sh` to determine the failing domain.
4. Restore from the last passing checkpoint.
5. Re-emit records and verify root changes are expected.
6. Re-run `bash scripts/certify_public_testnet.sh` before returning the component to active testnet use.
7. Escalate unresolved divergence through governance.
