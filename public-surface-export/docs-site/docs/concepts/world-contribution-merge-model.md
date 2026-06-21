# World Contribution Merge Model

## Objective

The World Contribution Merge Model defines how contributors propose, review, approve, merge, attribute, and reward changes to persistent EverArcade worlds. It is the World Git equivalent of Git pull requests, but for deterministic world artifacts instead of source-code branches.

The model makes this lifecycle repeatable:

```text
Fork World
↓
Create Content
↓
Submit Contribution
↓
Review Contribution
↓
Merge Contribution
↓
Attribute Contributor
↓
Reward Contributor
```

A world can accept outside contributions only when the workflow preserves deterministic execution, replay compatibility, governance authority, provenance, and reward obligations.

## Layering Rules

World contribution metadata is authoritative only when it is bound to signed artifacts and world history. A contribution request can describe intent, but a merge is valid only when the accepted artifact passes world validation and is recorded in canonical world history.

| Rule | Requirement |
| --- | --- |
| WCMG-001 | Contribution, review, approval, merge, attribution, and reward records must be content-addressed or signed. |
| WCMG-002 | Merged artifacts must not bypass world package validation, deterministic runtime rules, or governance execution. |
| WCMG-003 | Replay, restore, and migration tooling must be able to reconstruct the merge and its reward obligations. |
| WCMG-004 | Rejected or abandoned requests may remain discoverable, but they must not mutate canonical world state. |
| WCMG-005 | Attribution for inherited or merged content must survive operator changes and world forks unless a license explicitly allows removal. |

## Contribution Package

A `ContributionPackage` is a self-contained world artifact submitted for review. Examples include `quest-package.evr`, `housing-module.evr`, `npc-pack.evr`, `economy-module.evr`, `region-pack.evr`, and `governance-module.evr`.

```json
{
  "artifact_id": "questline-001",
  "artifact_hash": "sha256:...",
  "world_target": "frontier.evr",
  "contributor_id": "creator:ada",
  "artifact_type": "quest_package",
  "version": "1.0.0",
  "dependencies": [],
  "submission_root": "sha256:..."
}
```

### Required Fields

| Field | Meaning |
| --- | --- |
| `artifact_id` | Stable identifier for the submitted artifact. |
| `artifact_hash` | Hash of the packaged payload. |
| `world_target` | World identity the package targets. |
| `contributor_id` | Contributor identity or credential that authored the package. |
| `artifact_type` | Declared artifact class, such as `quest_package`, `region_package`, `npc_pack`, `asset_pack`, `economy_module`, `housing_module`, or `governance_module`. |
| `version` | Contributor package schema or semantic version. |
| `dependencies` | Required world modules, assets, schemas, fixtures, or prior artifacts. |
| `submission_root` | Root commitment over package payload, metadata, dependencies, license, fixtures, and proposed reward terms. |

### Package Requirements

A valid package should include deterministic fixtures, declared asset roots, dependency manifests, migration notes when relevant, a license, reviewer instructions, and proposed reward terms. Packages that affect economy or governance must declare the affected ledgers, policies, invariants, and replay fixtures needed to test them.

## World Contribution Request

A `WorldContributionRequest` (WCR) is the world equivalent of a Git pull request. It binds a contribution package to review metadata, approval authority, and proposed reward terms.

```json
{
  "request_id": "wcr.frontier.questline-001",
  "world_id": "frontier.evr",
  "artifact_id": "questline-001",
  "artifact_hash": "sha256:...",
  "contributor_id": "creator:ada",
  "summary": "Adds the first northern wolves questline.",
  "review_status": "pending"
}
```

### Required Metadata

| Metadata | Requirement |
| --- | --- |
| Contributor identity | `contributor_id`, signature, optional reputation profile, and license authority. |
| Contribution description | Human-readable summary, motivation, screenshots or fixtures when relevant, and scope boundaries. |
| Affected systems | Content, economy, governance, NPCs, quests, assets, housing, scheduler, settlement, or moderation surfaces touched by the package. |
| Expected dependencies | Existing artifacts, schemas, modules, world version, runtime version, and replay fixtures. |
| Proposed reward allocation | Reward source, allocation type, percentage or basis points, trigger, cap, duration, and settlement cadence. |
| Verification material | Artifact hash, submission root, deterministic fixtures, dependency graph, validation receipts, and contributor signature. |

## Review Lifecycle

A WCR progresses through explicit states:

```text
Draft → Submitted → Review → Changes Requested → Approved → Merged
                              ↘ Rejected
```

| State | Meaning |
| --- | --- |
| `Draft` | Contributor is preparing metadata, fixtures, dependencies, and reward proposal. |
| `Submitted` | Request is signed and available to the target world's review authority. |
| `Review` | Reviewers are evaluating validation results, governance impact, and reward terms. |
| `Changes Requested` | Reviewers require package or metadata updates before approval. |
| `Approved` | Required authority has accepted the contribution for merge subject to final validation. |
| `Merged` | Merge artifact is recorded in world history and attribution/reward records are active. |
| `Rejected` | Request is closed without mutating canonical world state or activating rewards. |

### Review Requirements

Reviewers should evaluate these gates before approval:

| Gate | Review Question |
| --- | --- |
| Determinism | Does the package produce stable outputs from the same state, inputs, tick, randomness seed, and dependency graph? |
| Compatibility | Does it preserve replay, restore, migration, schema, and world package compatibility? |
| Economic impact | Does it alter treasury flows, marketplace fees, inflation, sinks, faucets, asset rights, royalties, or player incentives? |
| Governance impact | Does it alter authority, voting rules, emergency powers, moderation policy, treasury permissions, or merge policy? |
| Security impact | Can it violate declared invariants, privilege boundaries, content safety policies, or settlement boundaries? |

A review record should include reviewer identity, reviewed artifact hash, checklist results, test receipts, comments, approval or rejection decision, and signature.

## Merge Artifact

A `MergeArtifact` is the World Git equivalent of a Git merge commit. It records that a reviewed contribution became part of canonical world history.

```json
{
  "merge_id": "merge.frontier.000042",
  "world_id": "frontier.evr",
  "artifact_id": "questline-001",
  "contributor_id": "creator:ada",
  "merge_tick": 420000,
  "merge_root": "sha256:...",
  "reviewers": ["operator:frontier", "council:quest-review"],
  "approval_hash": "sha256:..."
}
```

### Merge Requirements

A merge artifact must commit to the contribution package, WCR, review records, approval records, reward assignment, attribution entry, dependency lock, merge tick, resulting world root, and validation receipts. Once accepted, it becomes part of:

- world history;
- world lineage;
- world provenance;
- governance records;
- contributor attribution;
- reward obligations.

## Contribution Ledger

The `ContributionLedger` is the authoritative attribution ledger for merged artifacts. Every merged artifact must have an attribution entry.

| Field | Meaning |
| --- | --- |
| `contributor_id` | Identity credited for the accepted artifact. |
| `artifact_id` | Artifact accepted into the world. |
| `artifact_hash` | Hash of the accepted package payload. |
| `merge_id` | Merge artifact that accepted the package. |
| `merge_date` | Wall-clock date or epoch associated with the governance record. |
| `merge_tick` | Deterministic world tick at which the artifact entered history. |
| `world_id` | World receiving the contribution. |
| `reward_allocation_id` | Reward assignment activated by the merge, if any. |

Example contributor view:

```text
Frontier World

Contributor A
- Housing System

Contributor B
- Trade Network

Contributor C
- Region Pack

Contributor D
- Quest Chain
```

## Reward Allocation

Reward bindings connect accepted artifacts to durable compensation rules. Reward sources may include marketplace revenue, world treasury distributions, subscription revenue, governance grants, asset royalties, event fees, or creator marketplace revenue.

```json
{
  "artifact_id": "questline-001",
  "contributor_id": "creator:ada",
  "allocation_type": "royalty",
  "allocation_percent": 2.5
}
```

A production reward assignment should also include `world_id`, `merge_id`, `reward_source`, `trigger`, `cap`, `start_epoch`, `end_epoch`, `settlement_currency`, `settlement_cadence`, and `supersession_policy`.

Reward assignments must survive restore, migration, operator change, governance transition, and world fork when the inherited artifact license requires continuing obligations.

## Governance Approval Models

Worlds may use different merge authorities depending on maturity and governance structure.

| Model | Flow | Requirements |
| --- | --- | --- |
| Operator approval | Operator reviews, approves, and merges. | Operator signature, validation receipts, and merge artifact. |
| Council approval | Council reviews, votes, and authorizes merge. | Council membership record, quorum, threshold, vote set, and execution receipt. |
| Community approval | Stakeholders vote and contribution merges if thresholds pass. | Eligible voter set, proposal period, quorum, threshold, anti-duplication rules, and execution receipt. |
| Hybrid approval | Operator, council, and community share authority. | Explicit policy defining which classes require which authorities and emergency override limits. |

Governance approval must cite the WCR, artifact hash, review record, reward terms, and merge execution result.

## Contribution Verification

A verifier must be able to prove the full lifecycle using signed records and world history:

```text
Contribution Submitted
Contribution Reviewed
Contribution Approved
Contribution Merged
Contribution Rewarded
```

Verification checks should confirm contributor signatures, artifact hashes, submission roots, review signatures, approval thresholds, merge roots, resulting world roots, attribution entries, reward allocation entries, and ledger settlement records.

## World Fork Workflow

```bash
world fork frontier.evr
```

A fork creates an independent world branch with its own operators, governance, treasury policy, manifests, and contribution queue. The fork must preserve inherited provenance and any license-bound reward obligations.

Contribution workflow:

```bash
world fork frontier.evr
build region-pack.evr
world submit region-pack.evr
world review region-pack.evr
world merge region-pack.evr
```

## World Merge Workflow

```bash
world merge contribution.evr
```

A successful merge adds these records to world history:

- accepted artifact and dependency lock;
- attribution ledger entry;
- reward allocation entry;
- governance approval record;
- validation receipts;
- merge artifact;
- updated world root or package root.

The merge command must fail if validation fails, required governance approvals are missing, dependencies are unresolved, reward terms conflict with policy, or replay compatibility cannot be proven.

## Contributor Reputation Layer

The World Reputation Layer aggregates contributor history without replacing world governance. Operators and councils can use it to identify top quest designers, economy designers, governance architects, builders, artists, and maintainers.

Tracked metrics should include:

- merged contributions;
- rejected contributions;
- worlds served;
- player adoption;
- economic impact;
- governance participation;
- review participation;
- maintenance responsiveness;
- dispute outcomes;
- security or determinism incidents.

Reputation signals should cite underlying contribution, merge, review, adoption, and reward records so reputation remains auditable rather than purely social.

## End-to-End Lifecycle

```text
Discover World
↓
Fork World
↓
Build Contribution
↓
Submit WCR
↓
Review
↓
Approve
↓
Merge
↓
Attribute
↓
Reward
↓
Build Reputation
```

The success state is a living world repository: people, content, history, governance, and value creation accumulate in one replayable lineage.
