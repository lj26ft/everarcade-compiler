# World Trust, Reputation, and Governance Layer

## Objective

The World Trust, Reputation, and Governance Layer defines the social operating system that lets persistent EverArcade worlds scale beyond a single operator. The deterministic kernel lets worlds execute. Replay, restore, and migration let worlds survive. World Git lets worlds grow. This layer defines how worlds develop trust, authority, reputation, governance, and legitimacy over time.

It answers these operational questions:

```text
Who can contribute?
Who can review?
Who can approve?
Who can govern?
Who can be trusted?
How does a world avoid capture?
```

Without this layer, World Git is vulnerable to spam, governance attacks, contributor disputes, review cartels, treasury abuse, and operator abuse.

## Core Principle

Git repositories survive because maintainers emerge. Worlds survive because institutions emerge.

EverArcade governance therefore treats identity records, reputation records, reviewer decisions, maintainer appointments, votes, treasury decisions, constitutional changes, and anti-capture controls as replayable world history rather than off-platform social convention.

## Layering Requirements

Trust and governance metadata sits above the deterministic runtime and World Git merge model. It must never weaken the proof chain below it.

| Rule | Requirement |
| --- | --- |
| WTRG-001 | Identity, reputation, review, maintainer, role, vote, treasury, and constitution records must be content-addressed, signed, or rooted in world history. |
| WTRG-002 | Governance actions may authorize world mutation only through existing package validation, deterministic execution, replay, restore, and migration rules. |
| WTRG-003 | Restore and migration tooling must reconstruct the active constitution, role set, reviewer set, maintainer set, treasury policy, and reputation roots from history. |
| WTRG-004 | Registry trust signals are advisory; canonical authority remains in world history, governance artifacts, and signed contributor records. |
| WTRG-005 | Governance rules must prefer accountable institutions over implicit operator discretion as worlds mature. |

## Contributor Identity Model

A `ContributorIdentityRecord` is a portable contributor profile that survives operator changes, world migration, world restore, and contributor movement between worlds.

```json
{
  "contributor_id": "creator:ada",
  "display_name": "Ada",
  "joined_worlds": ["frontier.evr"],
  "merged_artifacts": ["questline-001"],
  "governance_roles": ["reviewer:quest-design"],
  "reputation_root": "sha256:..."
}
```

### Identity Requirements

| Field | Requirement |
| --- | --- |
| `contributor_id` | Stable portable identifier or credential subject. |
| `display_name` | Human-readable name that may change without changing identity. |
| `joined_worlds` | Worlds where the contributor has participated, joined, reviewed, governed, or merged artifacts. |
| `merged_artifacts` | Accepted artifact identifiers and merge references. |
| `governance_roles` | Active and historical governance roles with assignment artifacts. |
| `reputation_root` | Commitment to reputation category scores, evidence, attestations, and dispute state. |

Identity records should track contributions, merge history, governance participation, review participation, moderation history, reputation evidence, and revocation or dispute status. A world may require local admission rules, but it should not erase portable history from other worlds.

## World Reputation System

Reputation lets worlds evaluate contributors without reducing trust to a single global score. Reputation is category-specific, evidence-backed, attributable, auditable, and portable.

### Reputation Categories

| Category | Example evidence |
| --- | --- |
| Quest Design | Accepted quests, completion rates, player ratings, review outcomes. |
| Economy Design | Balanced faucets/sinks, marketplace health, inflation impact, treasury reviews. |
| World Building | Region packs, lore artifacts, continuity quality, adoption by players. |
| Governance | Vote participation, proposal quality, constitutional stewardship. |
| Engineering | Runtime modules, deterministic fixtures, migration safety, bug fixes. |
| Art | Asset packs, style consistency, usage by worlds, player adoption. |
| Moderation | Policy enforcement, appeal accuracy, abuse response, safety reports. |
| Community Leadership | Onboarding, conflict resolution, contributor retention, event leadership. |

### Reputation Signals

Reputation may be derived from these auditable signals:

- merged contributions;
- contribution usage;
- player adoption;
- economic impact;
- review accuracy;
- governance participation;
- appeal or dispute outcomes;
- long-term artifact health after restore, migration, and runtime upgrades.

```json
{
  "reputation_event_id": "rep.frontier.000144",
  "contributor_id": "creator:ada",
  "world_id": "frontier.evr",
  "category": "Quest Design",
  "signal": "Merged Contribution",
  "subject_artifact": "questline-001",
  "score_delta": 12,
  "evidence_root": "sha256:...",
  "issuer": "governance:frontier-council",
  "event_tick": 420000
}
```

### Reputation Requirements

| Requirement | Meaning |
| --- | --- |
| Portable | Contributors can carry evidence and reputation history between worlds. |
| Auditable | Reputation events reference evidence roots, issuers, dates or ticks, and affected artifacts. |
| Attributable | Positive and negative events identify the contributor, issuer, world, category, and signal. |
| Disputable | Worlds may attach challenges, appeals, reversals, or superseding reputation events. |
| Localizable | A world can weight outside reputation differently from local reputation without hiding the evidence. |

## Reviewer System

Reviewers are accountable participants who evaluate contribution packages before approval or merge. Reviewer eligibility may come from reputation thresholds, governance appointment, operator appointment, council election, or hybrid rules.

```json
{
  "reviewer_id": "creator:ada",
  "approved_contributions": 37,
  "rejected_contributions": 8,
  "review_accuracy": 0.92
}
```

### Reviewer Eligibility Models

| Model | Requirement |
| --- | --- |
| Reputation Threshold | Contributor qualifies after meeting category-specific score and evidence requirements. |
| Governance Appointment | Active governance body grants reviewer status through a governance artifact. |
| Operator Appointment | Operator appoints reviewer under an operator-controlled or hybrid constitution. |
| Council Election | Eligible voters or council members elect reviewer for a term. |
| Emergency Appointment | Temporary reviewer status is granted during incident response and expires automatically. |

### Reviewer Accountability

A review record should include reviewer identity, contribution hash, affected categories, checklist results, validation receipts, decision, rationale hash, conflicts of interest, signature, and later accuracy signals. Review accuracy can be updated when accepted content causes defects, rejected content is later accepted, security issues are found, or player/economic outcomes diverge from the review rationale.

## Maintainer Model

Maintainers are the World Git equivalent of repository maintainers. They hold explicit rights to approve contributions, merge contributions, reject contributions, delegate reviews, and trigger governance escalation when a contribution affects protected systems.

```json
{
  "maintainer_id": "creator:lin",
  "world_id": "frontier.evr",
  "scopes": ["quest_package", "region_package"],
  "rights": ["approve_contributions", "merge_contributions", "delegate_reviews"],
  "assigned_by": "vote.frontier.000021",
  "term_start_tick": 400000,
  "term_end_tick": 900000
}
```

### Promotion Paths

| Path | Description |
| --- | --- |
| Operator Appointment | Operator grants maintainer rights where the constitution allows centralized authority. |
| Council Election | Council or community vote appoints a maintainer for a scope and term. |
| Reputation Promotion | Contributor crosses reputation and review-accuracy thresholds, triggering promotion eligibility. |
| Hybrid Promotion | Reputation threshold creates eligibility, council vote confirms authority, and operator or multi-sig executes the assignment. |

Maintainers must be identifiable and auditable. Every maintainer right should have a scope, source of authority, term, revocation rule, and conflict-of-interest policy.

## Governance Roles

World institutions are expressed through versioned governance roles. Common roles include operator, maintainer, reviewer, council member, treasurer, moderator, historian, security steward, release steward, and registry delegate.

A role assignment is a governance artifact stored in world history:

```json
{
  "governance_artifact_id": "role.frontier.000077",
  "world_id": "frontier.evr",
  "artifact_type": "role_assignment",
  "role": "Treasurer",
  "subject_id": "creator:mina",
  "scope": "world_treasury",
  "authority": "vote.frontier.000019",
  "term": { "start_tick": 410000, "end_tick": 710000 },
  "revocation_rule": "council_majority_or_emergency_delay"
}
```

Role artifacts make governance replayable. A restored or migrated world can derive who held authority at any tick and whether an action was valid under the active constitution.

## World Constitution Model

A world constitution defines the rules that create legitimacy for reviews, merges, roles, votes, treasury actions, emergency powers, and amendments.

```json
{
  "constitution_id": "constitution.frontier.v3",
  "world_id": "frontier.evr",
  "governance_model": "hybrid",
  "amendment_rules": {
    "proposal_threshold": "2 maintainers or 5 reviewers",
    "quorum": "60% of active council",
    "approval_threshold": "2/3",
    "challenge_period_ticks": 50000
  }
}
```

### Constitution Models

| Model | Description |
| --- | --- |
| Operator Controlled | Operator retains primary authority, with explicit audit trails and optional advisory review. |
| Council Controlled | Elected or appointed council controls protected decisions, treasury, and role assignment. |
| Community Controlled | Eligible members vote directly on core decisions. |
| Hybrid | Operator, maintainers, council, and community each control specific scopes with checks and balances. |

Constitution artifacts must be versioned, replayable, and migratable. Amendments supersede prior constitutions only through the amendment rules active at the proposal tick.

## Governance Voting System

Votes are governance artifacts that become replayable world history. Supported voting types include merge approval, treasury allocation, role assignment, constitution amendment, world direction, emergency action, policy ratification, and dispute resolution.

```json
{
  "vote_id": "vote.frontier.000021",
  "world_id": "frontier.evr",
  "vote_type": "Role Assignment",
  "proposal_root": "sha256:...",
  "eligible_voter_root": "sha256:...",
  "voting_rule": "one_member_one_vote",
  "threshold": "majority",
  "quorum": "50%",
  "result": "passed",
  "execution_artifact": "role.frontier.000077"
}
```

### Voting Requirements

| Mode | Requirement |
| --- | --- |
| Transparent voting | Vote records disclose voter identity, vote value, signature, and eligibility evidence. |
| Private voting | Future mode that commits to encrypted or zero-knowledge vote evidence while preserving verifiability. |
| Weighted voting | Weights are derived from explicit rules such as role, reputation, stake, tenure, or delegated authority. |
| One-member-one-vote | Each eligible identity contributes one counted vote under anti-duplication rules. |

A vote is valid only when it references an eligible voter set, proposal root, voting window, quorum, threshold, tally method, challenge period, and execution artifact.

## Treasury Governance

Treasury governance controls world funds for contributor rewards, development grants, infrastructure funding, events, moderation operations, security audits, and world expansion.

```json
{
  "treasury_action_id": "treasury.frontier.000088",
  "world_id": "frontier.evr",
  "action_type": "Development Grant",
  "recipient_id": "creator:ada",
  "amount": "1000 WORLD_CREDITS",
  "source_account": "world_treasury",
  "authority": "vote.frontier.000030",
  "vesting_rule": "milestone_receipts_required"
}
```

### Treasury Rule Models

| Model | Requirement |
| --- | --- |
| Operator Controlled | Operator may spend within constitution-defined caps and disclosure rules. |
| Council Controlled | Council vote authorizes allocations and emergency reserves. |
| Multi-Signature | Multiple named roles or institutions must sign before execution. |
| Community Controlled | Eligible members approve allocations by vote. |

Treasury actions should include recipient, amount, source, purpose, authority, milestone receipts, clawback or dispute rules, and public accounting roots.

## Anti-Capture Mechanisms

Worlds should survive bad actors. The layer explicitly models threats and mitigations.

| Threat | Mitigations |
| --- | --- |
| Contributor Spam | Reputation requirements, submission fees or bonds, rate limits, scoped manifests, reviewer triage. |
| Review Cartels | Multi-party approval, randomized reviewer assignment, conflict disclosure, review accuracy penalties, appeal paths. |
| Governance Capture | Role separation, quorum floors, time delays, challenge periods, amendment supermajorities, voter eligibility snapshots. |
| Treasury Abuse | Spending caps, multi-signature execution, public accounting roots, grant milestones, delayed execution, emergency veto. |
| Operator Abuse | Constitution constraints, export rights, maintainer/council override paths, fork/migration rights, transparent governance artifacts. |

Anti-capture controls should be configurable by constitution but replayable as explicit rules. Emergency powers should be narrow, time-limited, logged, and challengeable.

## Reputation Portability

Contributors should not be trapped by a world. A contributor who builds reputation in World A can move to World B while retaining reputation history, evidence roots, and attribution records.

```text
Build reputation in World A
↓
Export identity, reputation roots, merged artifacts, review history, and governance roles
↓
Join World B
↓
World B verifies history and applies local weighting rules
```

Portability requires stable contributor identifiers, exportable evidence bundles, verifiable signatures, chain-of-custody for merged artifacts, issuer metadata, and dispute or revocation status. Receiving worlds may choose their own trust weights, but they should expose how local scores were derived from portable evidence.

## World Registry Trust Signals

The World Registry helps contributors discover healthy worlds. Registry trust signals are informational, not authoritative.

Registry entries should display:

- governance model;
- active maintainers;
- active contributors;
- treasury health;
- merge activity;
- reputation activity;
- reviewer count and review latency;
- recent constitution amendments;
- role churn;
- unresolved disputes;
- anti-capture controls enabled.

```json
{
  "world_id": "frontier.evr",
  "governance_model": "hybrid",
  "maintainer_count": 8,
  "active_contributors_30d": 42,
  "treasury_health": "funded_12_months",
  "merge_activity_30d": 19,
  "reputation_activity_30d": 67,
  "anti_capture_controls": ["challenge_periods", "multi_party_approval", "role_separation"]
}
```

These signals allow informed participation while preserving the rule that the authoritative records live in world history and signed governance artifacts.

## Governance Lifecycle

World leadership should emerge through participation:

```text
Contributor
↓
Reviewer
↓
Maintainer
↓
Council Member
↓
World Leader
```

A healthy governance lifecycle should make each promotion legible:

| Stage | Entry signal | Authority gained |
| --- | --- | --- |
| Contributor | Joined world, submitted artifacts, participated in community. | May submit contribution requests and build reputation. |
| Reviewer | Reputation, appointment, election, or scoped expertise. | May review and sign contribution decisions. |
| Maintainer | High review accuracy, trust, election, or appointment. | May approve, merge, reject, and delegate within scope. |
| Council Member | Election, term assignment, or constitutional role. | May govern protected decisions, roles, treasury, and amendments. |
| World Leader | Sustained legitimacy across contribution, review, treasury, and governance. | May coordinate strategic direction under constitutional checks. |

## Success Criteria

A contributor can:

```text
Build Reputation
↓
Earn Trust
↓
Review Contributions
↓
Become Maintainer
↓
Participate in Governance
```

A world can:

```text
Attract Contributors
↓
Evaluate Trust
↓
Promote Leaders
↓
Allocate Resources
↓
Resist Capture
↓
Survive Leadership Change
```

A governance system can outlive operators, studios, and companies when authority, reputation, roles, votes, constitutions, and treasury records are portable, auditable, replayable, and migratable.

## Long-Term Objective

The final goal is not merely world ownership. The final goal is persistent digital institutions: worlds that accumulate contributors, history, governance, culture, and value, then continue existing long after their original creators are gone.
