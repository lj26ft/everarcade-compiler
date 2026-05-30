# EverArcade Vertical Slice Report

## First Official Game

**Arena Vanguard** is the first official EverArcade vertical-slice game. It is scoped as a one-world arena battler with one gameplay loop, one progression loop, multiplayer enabled, persistent save enabled, and deployment enabled.

## Gameplay Loop

```text
join match -> defeat waves -> collect shards -> extract or fall
```

## Progression Loop

```text
bank shards -> upgrade loadout -> unlock arena modifiers
```

## End-to-End Workflow

The title is certified against the creator workflow surfaces required for a complete production pass:

1. Studio
2. Gameplay Framework
3. World Authoring
4. Publishing Pipeline
5. Deployment Pipeline

Manual runtime hacks are not allowed in the certification model.

## Package Outputs

The vertical slice validates deterministic generation of the required package set:

- `arena-vanguard.game.pkg`
- `arena-vanguard.world.pkg`
- `arena-vanguard.deployment.pkg`
- `arena-vanguard.runtime.pkg`

The package set is reproducible because the package root is derived from canonical game metadata and ordered package identities.

## Deployment Trial

The deployment trial covers:

- EverNode
- local federation
- standalone runtime

The required lifecycle checks are startup, recovery, multiplayer, operations, shutdown, and restart.

## Multiplayer Certification

The multiplayer gate uses a four-player session and requires:

- multiple players join
- persistent state survives
- world recovery succeeds
- replay continuity is preserved

## Open Protocol Certification

The vertical slice verifies deterministic execution, replay safety, runtime recovery, package reproducibility, and deployment reproducibility.
