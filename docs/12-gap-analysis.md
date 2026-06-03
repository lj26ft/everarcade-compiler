# 12. Gap Analysis

Each gap includes description, current state, required work, dependency chain, and priority.

## Required For v0.1

### Runtime release gate closure

- **Description:** Establish a minimal certified release path for single-node deterministic runtime operation.
- **Current state:** Runtime, execution, replay, checkpoints, and operator commands are implemented foundations with partial release gates.
- **Required work:** Define v0.1 scope, run targeted deterministic tests, verify artifacts, document waivers, and archive evidence.
- **Dependency chain:** execution core → state roots → receipts → checkpoints → replay verification → release approval.
- **Priority:** Critical.

### Restore command completion

- **Description:** Convert restore from prepared/scaffold behavior to an operator-certified restore flow.
- **Current state:** Backup and recovery foundations exist; restore is partial.
- **Required work:** Add restore activation policy, validation checks, rollback safety, and operator procedure evidence.
- **Dependency chain:** checkpoint integrity → backup manifest → journal replay → health verification.
- **Priority:** Critical.

### Documentation authority consolidation

- **Description:** Replace historical onboarding with canonical documentation.
- **Current state:** Canonical hierarchy now exists; older docs remain evidence.
- **Required work:** Keep new architecture claims inside numbered docs and redirect historical reports to evidence status.
- **Dependency chain:** governance → repository navigation → architecture references → review rules.
- **Priority:** High.

## Required For Beta

### Operator observability

- **Description:** Operators need stable health, metrics, alerts, and incident views.
- **Current state:** Health and metrics primitives are partial; dashboards are non-authoritative.
- **Required work:** Define metrics contract, alert thresholds, log retention, dashboards, and operator drills.
- **Dependency chain:** runtime health → metrics export → operator console → incident runbook.
- **Priority:** High.

### Upgrade and rollback certification

- **Description:** Beta needs safe runtime upgrades with continuity evidence.
- **Current state:** Upgrade architecture is partial.
- **Required work:** Implement upgrade gate automation, migration evidence, post-upgrade replay window, and rollback command procedure.
- **Dependency chain:** checkpoint → artifact verification → upgrade migration → replay verification → rollback retention.
- **Priority:** High.

## Required For Production

### External audit readiness

- **Description:** Production requires security and determinism review by parties outside daily implementation.
- **Current state:** Threat and security documents exist; external certification is not complete.
- **Required work:** Freeze audit scope, produce architecture package, run adversarial tests, resolve findings.
- **Dependency chain:** canonical docs → release gates → test evidence → audit package.
- **Priority:** Critical.

### Incident response and SLOs

- **Description:** Production operation requires defined response times, escalation, and data-loss objectives.
- **Current state:** Runbooks exist in fragments; no production SLO policy is authoritative.
- **Required work:** Define RTO/RPO, incident severity, escalation, customer communication, and postmortem requirements.
- **Dependency chain:** operations manual → observability → backup/restore → release approval.
- **Priority:** Critical.

## Required For Multi-Host Federation

### Authenticated peer policy

- **Description:** Federation peers require identity, trust scope, admission, revocation, and quarantine policy.
- **Current state:** Peer and crypto identity modules exist; production policy is partial.
- **Required work:** Define admission protocol, signing policy, peer reputation, revocation, and audit trail.
- **Dependency chain:** node identity → signed manifest → peer registry → convergence validation.
- **Priority:** Critical.

### Partition and rejoin hardening

- **Description:** Multi-host operation must survive network partitions and rejoin without divergent authority.
- **Current state:** Tests and reports exist; production gate is missing.
- **Required work:** Run adversarial partitions, long-duration soak, recovery drills, and operator certification.
- **Dependency chain:** receipt exchange → checkpoint sync → root comparison → replay compare → recovery.
- **Priority:** Critical.

## Required For Commercial Hosting

### Hosting control plane

- **Description:** Commercial hosting needs tenant lifecycle, capacity, billing boundaries, and operator controls.
- **Current state:** Control-plane and provider code exist; commercial product policy is planned.
- **Required work:** Define tenant model, quotas, provisioning API, billing handoff, abuse response, and support workflows.
- **Dependency chain:** Evernode provider → artifact verification → observability → operator policy → billing boundary.
- **Priority:** High.

### Capacity and storage policy

- **Description:** Operators need sizing, retention, archive, and cost models.
- **Current state:** Reports exist; production policy is partial.
- **Required work:** Define storage tiers, checkpoint retention, replay archive retention, compression policy, and capacity gates.
- **Dependency chain:** metrics → archive formats → checkpoint policy → cost model.
- **Priority:** High.

## Required For XRPL Integration

### Settlement boundary certification

- **Description:** External ledger settlement must consume runtime evidence without owning runtime state.
- **Current state:** XRPL and hook scaffolds exist; integration is not production certified.
- **Required work:** Define settlement proofs, failure handling, ledger finality assumptions, and dispute workflow.
- **Dependency chain:** receipt roots → checkpoint anchors → proof manifests → hook validation.
- **Priority:** Medium.

## Required For ZK Integration

### Proof circuit scope

- **Description:** ZK integration needs a precise proof target and circuit ownership model.
- **Current state:** Planned.
- **Required work:** Choose proof statements, inputs, public outputs, verification location, and cost model.
- **Dependency chain:** canonical ABI → execution trace → state root → receipt root → verifier contract/service.
- **Priority:** Medium.

## Required For Creator Marketplace

### Package publication and trust

- **Description:** Marketplace publishing needs package identity, creator identity, validation status, and distribution policy.
- **Current state:** Registry and creator tooling are scaffold/partial.
- **Required work:** Define package certification badges, vulnerability response, versioning, takedown, and revenue boundaries.
- **Dependency chain:** package format → release certification → registry → deployment provider → creator dashboard.
- **Priority:** Medium.

### Creator onboarding completeness

- **Description:** Creators need a reliable path from example to certified deployable package.
- **Current state:** SDK docs and examples exist; certification connection is partial.
- **Required work:** Align quickstarts with packaging, local replay, validation, deployment, and marketplace publication gates.
- **Dependency chain:** SDK → examples → package validator → replay verifier → release certification.
- **Priority:** High.
