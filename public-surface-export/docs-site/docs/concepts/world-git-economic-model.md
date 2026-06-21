# World Git Economic Model

## Objective

The World Git layer defines the economic, governance, and contribution primitives that allow EverArcade worlds to behave like Git repositories for persistent realities. Deterministic execution, replay verification, commitment verification, restore verification, and migration verification prove that a world can continue correctly. World Git defines how a world attracts contributors, attributes work, rewards value creation, includes players as economic participants, and funds its own continuity.

The final objective is not only decentralized game hosting. The final objective is worlds that outlive their original creators.

## Core Principle

Git made software collaboration durable by making repositories the unit of history. EverArcade makes persistent worlds collaborative by making the world the primary unit of ownership, contribution, economic accounting, and governance.

A Git repository accumulates code, commits, contributors, and history. An EverArcade world accumulates content, creators, assets, institutions, economic flows, governance decisions, and continuity proofs.

## Layering Requirements

World Git sits above the runtime proof chain and must not weaken the lower layers. A valid World Git implementation requires the underlying world to preserve:

- deterministic execution;
- replay-verifiable history;
- commitment verification;
- restore verification;
- migration verification;
- stable world identity;
- canonical package, artifact, and state roots.

World Git metadata may advertise, discover, attribute, or reward activity, but it must not mutate canonical state without passing through the world runtime, governance, or package acceptance rules.

## World Contributor Manifest

A world operator or world governance publishes a `WorldContributorManifest` to advertise what the world needs, what contributors can earn, and how contributions are approved.

```json
{
  "world_id": "frontier.evr",
  "manifest_version": "1.0.0",
  "wanted_roles": [
    "quest_designer",
    "environment_artist",
    "combat_designer",
    "economy_designer"
  ],
  "open_projects": [
    {
      "project_id": "frontier.northern-expansion",
      "title": "Northern Expansion Zone",
      "needed_roles": ["zone_designer", "environment_artist"],
      "status": "open",
      "reward_model": "royalty"
    }
  ],
  "reward_model": "royalty",
  "governance_model": "operator_approved",
  "submission_policy": {
    "requires_artifact_hash": true,
    "requires_replay_fixture": true,
    "requires_license": true,
    "review_authority": "operator"
  }
}
```

### Manifest Fields

| Field | Meaning |
| --- | --- |
| `world_id` | Stable world identity receiving contributions. |
| `manifest_version` | Schema version of the contributor manifest. |
| `wanted_roles` | Skills the world currently needs. |
| `open_projects` | Bounded opportunities contributors can claim or propose against. |
| `reward_model` | Default earning model: `revenue_share`, `royalty`, `governance_allocation`, or `hybrid`. |
| `governance_model` | Approval model: `operator_controlled`, `council_controlled`, `tokenless_governance`, or `hybrid_governance`. |
| `submission_policy` | Requirements for accepted contribution artifacts. |

### Contributor Manifest Invariants

| Invariant | Requirement |
| --- | --- |
| WCM-001 | A manifest must reference exactly one stable `world_id`. |
| WCM-002 | Reward and governance models must be explicit before contribution acceptance. |
| WCM-003 | Open projects must declare a review authority and artifact requirements. |
| WCM-004 | Manifest updates must be versioned and retained in world history or registry history. |
| WCM-005 | Manifest publication must not imply acceptance of any artifact. |

## Contributor Discovery Through the World Registry

The World Registry becomes the discovery layer for worlds seeking contributors. Registry entries should surface contributor opportunities without becoming the authority that approves or pays contributors.

A registry entry may include these World Git discovery fields:

```rust
pub struct WorldGitDiscovery {
    pub world_id: String,
    pub contributor_manifest_hash: String,
    pub wanted_roles: Vec<String>,
    pub open_project_count: u32,
    pub governance_model: String,
    pub reward_models: Vec<String>,
    pub treasury_status: String,
    pub marketplace_enabled: bool,
}
```

Contributor search must support:

- active worlds;
- wanted roles;
- open projects;
- governance structures;
- reward structures;
- treasury status;
- marketplace availability;
- certification level.

The registry remains informational. The contributor manifest, world package, governance record, and treasury ledger remain authoritative for acceptance and rewards.

## Contribution Artifact

Every contribution must be packaged, hashed, attributable, and traceable to the world that accepted or rejected it.

```json
{
  "contribution_id": "contrib.frontier.quest.wolves-v1",
  "contributor_id": "creator:ada",
  "world_id": "frontier.evr",
  "artifact_type": "quest_package",
  "artifact_hash": "sha256:...",
  "contribution_root": "sha256:...",
  "license": "world-deployable-v1",
  "dependencies": ["asset.frontier.wolf-pack-v2"],
  "reward_terms": {
    "model": "royalty",
    "basis_points": 250,
    "trigger": "quest_completion_fee"
  },
  "governance_status": "accepted",
  "accepted_in_world_commit": "worldcommit:frontier:000042"
}
```

Supported artifact classes include:

- quest packages;
- zone packages;
- NPC packages;
- asset packages;
- economy modules;
- governance modules;
- event packages;
- marketplace templates;
- moderation policies;
- lore or history records.

### Contribution Artifact Invariants

| Invariant | Requirement |
| --- | --- |
| CA-001 | Every artifact must include `contributor_id`, `artifact_hash`, `world_id`, and `contribution_root`. |
| CA-002 | `artifact_hash` identifies the submitted payload. |
| CA-003 | `contribution_root` commits to payload, metadata, dependency declarations, license, and reward terms. |
| CA-004 | Accepted artifacts must reference the world commit or governance decision that accepted them. |
| CA-005 | Reward terms must be immutable after acceptance unless superseded by explicit governance action. |
| CA-006 | Rejected artifacts may remain attributable but must not earn deployment rewards. |

## Reward Models

World Git supports contributor rewards without requiring employment by a centralized studio.

### Revenue Share

A contributor receives a percentage of world-level income streams such as marketplace fees, subscriptions, creator marketplace revenue, or world service revenue. Revenue share terms should define:

- covered income streams;
- percentage or basis points;
- start and end conditions;
- caps, if any;
- payment cadence;
- dispute process.

### Royalty

A contributor receives ongoing payments when a specific artifact creates value. Royalty triggers may include:

- asset use;
- module deployment;
- content consumption;
- quest completion;
- zone access;
- marketplace resale;
- license rental.

### Governance Allocation

World governance allocates treasury funds to contributors through proposals, budgets, grants, bounties, retroactive rewards, or maintenance contracts. Governance allocations should cite the contribution roots or contributor identities receiving funds.

### Reward Ledger

A world should maintain a deterministic `RewardLedger`:

```rust
pub struct RewardLedgerEntry {
    pub world_id: String,
    pub contributor_id: String,
    pub contribution_root: String,
    pub reward_model: String,
    pub income_source: String,
    pub amount: u64,
    pub currency: String,
    pub settlement_status: String,
    pub epoch: u64,
}
```

The reward ledger makes attribution, accrual, settlement, and auditability replayable.

## Player Participation Layer

Players are economic participants, not only consumers. A world may expose player roles such as:

- merchant;
- guild leader;
- land owner;
- event organizer;
- market maker;
- world historian;
- content curator;
- craft specialist;
- logistics operator;
- landlord;
- service provider;
- governance delegate.

Player participation capabilities include:

- earn;
- trade;
- rent;
- license;
- build;
- govern;
- curate;
- organize events;
- maintain institutions.

### World-Owned Assets

World-owned and player-controlled assets may include houses, land, ships, guild halls, market stalls, collectibles, licenses, leases, crafting stations, and service rights.

Asset ownership must survive:

- operator migration;
- world migration;
- restore events;
- checkpoint recovery;
- governance transition;
- studio or operator failure.

A portable asset record should include:

```json
{
  "asset_id": "land.frontier.north-plot-17",
  "world_id": "frontier.evr",
  "owner_id": "player:mira",
  "asset_type": "land",
  "asset_hash": "sha256:...",
  "rights": ["use", "build", "rent", "transfer"],
  "encumbrances": ["guild_tax:2pct"],
  "last_continuity_root": "sha256:..."
}
```

## World Treasury Model

A self-sustaining world maintains a treasury that receives income, pays expenses, stores reserves, and exposes audited balances.

### Income Categories

World treasuries may receive:

- market fees;
- transaction fees;
- subscriptions;
- governance fees;
- world services;
- creator marketplace revenue;
- land or stall rent;
- license fees;
- event fees;
- bridge or settlement fees.

### Expense Categories

World treasuries may pay:

- hosting;
- development;
- moderation;
- events;
- treasury growth or reserves;
- contributor rewards;
- operator compensation;
- audit and certification costs;
- public goods and community grants.

### Sustainability Rules

A world can operate as an independent economic entity when it can:

- pay operators for infrastructure and continuity work;
- pay contributors for accepted artifacts and maintenance;
- maintain replay, restore, migration, and certification infrastructure;
- fund moderation and player safety;
- build reserves for outages or operator turnover;
- expose treasury balances and obligations to governance participants.

A treasury policy should define minimum reserve ratios, spending authorities, reward priority, operator compensation, emergency spending limits, and migration funding.

## Governance Model

World Git supports four governance modes.

### Operator Controlled

The operator approves contributions, deploys artifacts, updates manifests, and manages treasury policy. This mode is simple and useful for early worlds, but survivability depends on migration rights and treasury transparency.

### Council Controlled

A contributor or stakeholder council approves contributions and budget decisions. Council membership should be recorded in governance history and survive operator migration.

### Tokenless Governance

World stakeholders vote through non-token identity, reputation, contribution, participation, or role-based credentials. This model avoids requiring a speculative governance token while still allowing stakeholder control.

### Hybrid Governance

The operator and community share authority. For example, the operator may retain emergency powers while contributor councils approve artifacts and players approve treasury budgets.

Governance records should cite the world commit, proposal, vote set, quorum, threshold, execution result, and affected artifacts or treasury entries.

## World Git Commands

### World Clone

```text
world clone frontier.evr
```

A clone acquires the world package, history, schemas, governance metadata, contributor manifest, treasury policy, and registry references needed to inspect or operate the world without becoming the canonical authority.

### World Fork

```text
world fork frontier.evr
```

A fork creates an independent branch with alternate governance, economy, content direction, manifests, reward policies, and operators. Forks must preserve attribution to inherited artifacts and must not erase original contributor reward obligations unless the original license permits it.

### World Merge

```text
world merge branch-a
```

A merge imports approved assets, modules, content, governance changes, or economic policy into canonical world history. Merges must pass artifact validation, dependency checks, governance approval, reward-term preservation, and replay compatibility checks.

### World Contributors

```text
world contributors frontier.evr
```

Contributor inspection shows contributors, artifacts, contribution roots, accepted world commits, reward allocations, governance rights, and reputation signals.

### World Marketplace

```text
world marketplace frontier.evr
```

Marketplace inspection shows deployable content, contributor artifacts, license terms, royalty information, dependency graphs, certification status, and governance acceptance status.

## World Git Acceptance Criteria

A world can advertise what it needs, what it offers, and how contributors earn value when it publishes a contributor manifest and exposes it through registry discovery.

Every world artifact can be attributed, tracked, and rewarded without a centralized employer when artifacts include contributor identity, hashes, contribution roots, accepted world commits, and reward-ledger entries.

Players have economic roles beyond gameplay progression when the player participation layer supports earning, trading, renting, licensing, building, governing, and durable ownership of world assets.

A world can operate as an independent economic entity when treasury income, expenses, reserves, operator compensation, contributor rewards, and governance approvals are replayable and auditable.

## End-to-End Success Flows

A contributor should be able to follow this path:

```text
Discover World
↓
Contribute Artifact
↓
Receive Attribution
↓
Receive Reward
↓
Build Reputation
↓
Participate in Governance
```

A player should be able to follow this path:

```text
Join World
↓
Create Value
↓
Own Assets
↓
Participate Economically
↓
Help Govern World
```

A world should be able to follow this path:

```text
Attract Contributors
↓
Reward Contributors
↓
Retain Players
↓
Generate Revenue
↓
Fund Operations
↓
Survive Indefinitely
```

## Long-Term Outcome

A world exists. The original studio disappears. The operator migrates. Contributors continue building. Players continue participating. The economy continues functioning. The world survives.
