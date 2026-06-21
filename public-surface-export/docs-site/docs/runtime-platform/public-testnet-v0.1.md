# EverArcade Public Testnet v0.1

## Purpose

Public Testnet v0.1 is the first operational EverArcade testnet intended for external users. It validates that developers, lease operators, GPU providers, players, and governance participants can use the certified runtime stack through real workflows while remaining testnet-only.

The goal is ecosystem validation, not production launch. Failures are expected and useful because the testnet exists to expose operational problems before release-candidate hardening.

## Enrollment

Enrollment is split by participant domain:

- Developer enrollment records developer registration, project registration, project approval, and project status.
- Operator enrollment records lease operators, node operators, federation operators, health, checkpoints, and replays.
- GPU provider enrollment records provider registration, capability advertisement, capacity declaration, and provider status.
- Player records identify test cohorts that interact with deployed projects.

The generated enrollment roots are stored in `public-testnet/records/roots.env`.

## Deployments

The deployment registry represents:

- Project identity.
- Deployment identity.
- Lease assignment.
- Federation membership.
- Deployment status.

Deployments must run through certified runtime surfaces and remain auditable through deployment records, operator checkpoints, and replay IDs.

## Civilizations

The civilization registry represents civilization, world, region, governance state, and economy state. It validates civilization-runtime behavior across deployed worlds while preserving deterministic replay and testnet-only economy state.

## GPU Marketplace

GPU marketplace testing represents providers, jobs, artifacts, verification records, and settlement intents. Providers advertise capabilities and capacity, execute projection jobs, emit deterministic artifacts, and record verification results.

GPU settlement records are intent-only and must never become commercial billing records.

## Settlement Testing

Settlement testing represents:

- Settlement intents.
- XRPL testnet settlement observations.
- Xaman test authorizations.
- Receipt imports.

All settlement flows use XRPL testnet semantics. Production funds, production revenue, production asset value, paid settlement, real billing, and mainnet assets are out of scope.

## Governance

Governance records proposals, votes, policies, and testnet rule changes. Governance is used to change testnet operating parameters such as provider windows, cohort limits, and temporary recovery procedures.

Every governance action must be auditable and replayable.

## Analytics

Analytics are derived observations over:

- Developers.
- Deployments.
- Civilizations.
- GPU jobs.
- Settlement events.
- Operator activity.

Analytics do not create authority. They summarize recorded testnet activity for validation, reporting, and operational review.

## Replay

The replay layer reconstructs enrollment, deployments, civilizations, GPU activity, settlement activity, and governance activity. Public Testnet v0.1 passes replay only when the replay root equals the aggregate public testnet root.

## PASS Criteria

Public Testnet v0.1 passes when:

- Developer enrollment exists and has a valid root.
- Operator enrollment exists and has a valid root.
- GPU provider enrollment exists and has a valid root.
- Deployment registry exists and has a valid root.
- Civilization registry exists and has a valid root.
- Settlement testing exists, uses testnet-only records, and has a valid root.
- GPU marketplace testing exists and has a valid root.
- Governance exists and has a valid root.
- Analytics exists and has a valid root.
- Replay exists and its root equals the public testnet root.
- Validation reports `Public Testnet Validation: PASS`.
- Certification reports `Public Testnet: PASS`.

## FAIL Criteria

Public Testnet v0.1 fails if any of the following occur:

- A required enrollment, registry, settlement, marketplace, governance, analytics, or replay record is missing.
- Any generated root is malformed.
- Replay root does not equal the public testnet root.
- A record requires production XRPL funds, production revenue, paid settlement, real billing, production asset value, or mainnet launch behavior.
- Validation or certification reports anything other than PASS.

## Relationship To Developer Portal

The Developer Portal remains the non-authoritative builder interface for creating projects, preparing deployments, viewing dashboards, and observing marketplace or GPU state. Public Testnet v0.1 consumes those completed platform surfaces as operational workflows for external users.

The testnet does not rewrite the portal. It records public-testnet participation and validates end-to-end lifecycle execution against the existing platform stack.

## Relationship To Release Candidate

Public Testnet v0.1 precedes Release Candidate v0.1. It gathers operational evidence, failure reports, replay confidence, and settlement-flow validation before interfaces are frozen.

Release Candidate v0.1 should use public-testnet findings to validate upgrade paths, certify operational readiness, and prepare commercial deployment without changing the testnet-only guarantees of this milestone.
