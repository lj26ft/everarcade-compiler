> **Repository boundary:** Economic and treasury systems are outside the scope of this open-source reference implementation.
>
> Implementations may attach non-authoritative economic metadata, but live payments, custody, accounting, treasury services, and commercial settlement are not implemented here.
>
# World Operator Runtime and Treasury Execution Layer

This specification defines how a world acts as a self-sustaining economic institution. The treasury belongs to the world identity and is replayed as world history; it does not belong to any temporary operator, host, council, or marketplace.

## Treasury model

A world treasury record contains the world identifier, treasury identifier, treasury root, balances by asset, income streams, expense streams, and active governance model. Treasury identity and history must survive operator rotation, restore, migration, and governance transitions.

## Revenue sources

Supported revenue classes are marketplace fees, capability royalties, subscriptions, transaction fees, governance fees, licensing, events, and donations. Each revenue event records payer, amount, asset, attribution, and timestamp so income can be audited and replayed.

## Expense model

Supported expense classes are contributor payments, operator compensation, infrastructure costs, moderation costs, governance operations, world events, and treasury grants. Every expenditure references a recipient, artifact, and governance action so spending remains an artifact of world history.

## Contributor compensation

The compensation engine supports fixed payments, revenue share, royalties, and treasury grants. Compensation artifacts identify the contributor, artifact, payment type, asset, amount, and treasury source. Because these artifacts are replay data, contributor payment history survives restore, migration, and operator turnover.

## Capability royalties

Capability royalty events connect usage in a world to a capability, creator, usage units, amount, and attribution root. This allows reusable capabilities such as `housing.evr` to receive deterministic, auditable rewards across many worlds.

## Treasury governance

Treasury governance actions cover budget approval, grant approval, compensation approval, treasury allocation, and emergency expenditure. Governance may be operator-controlled, council-controlled, community-controlled, or hybrid. Executed actions are replayable governance history.

## Budgets and grants

Budget artifacts version allocations for development, infrastructure, moderation, events, reserve, education, and growth. Grant artifacts track contributor, capability, infrastructure, research, and community grants through proposal, review, vote, approval, disbursement, and reporting.

## Verification, replay, restore, and migration

Independent implementations must reproduce treasury state, revenue events, expense events, grant events, and compensation events from replay data. Restore checks compare treasury root, balance, and compensation history. Migration checks preserve treasury identity, treasury state, and governance allocations.

## Operator runtime model

Operators are accountable participants responsible for runtime hosting, availability, upgrades, security operations, governance execution, and treasury execution. Operator actions become auditable artifacts. Operator compensation may come from treasury allocations, hosting fees, governance allocations, or marketplace revenue share, and should align operator incentives with world sustainability.

## Treasury health

Treasury health tracks revenue, expenses, reserve ratio, contributor payouts, grant performance, revenue growth, treasury runway, contributor retention, and dependency concentration. These signals help worlds detect economic collapse risk before reserves are exhausted.

## Machine-readable API surface

Tooling should expose at least the following commands:

```bash
world treasury status
world treasury revenue
world treasury expenses
world treasury grants
world treasury payouts
```

Each command returns replay-derived, machine-readable treasury data.

## Economic lifecycle

The target lifecycle is:

```text
Player Activity
↓
World Revenue
↓
Treasury Accumulation
↓
Governance Allocation
↓
Contributor Compensation
↓
Capability Rewards
↓
Infrastructure Funding
↓
World Growth
```

The long-term goal is a world that can fund contributors, operators, infrastructure, and growth while surviving governance changes, operator turnover, restore, and migration without a publisher or central platform.
