# Civilization World Template

## Purpose

Full Continuity Engine worlds.

## Demonstrates

Worlds as digital civilizations.

## Included RustRigs

- `identity.join`
- `position.move`
- `resource.harvest`
- `inventory.transfer`
- `structure.build`
- `structure.repair`
- `structure.decay`
- `market.list`
- `market.trade`
- `governance.create_proposal`
- `governance.vote`
- `faction.create`
- `faction.join`
- `guild.create`
- `quest.start`
- `quest.complete`
- `continuity.advance`
- `continuity.record_event`
- `migration.prepare`
- `migration.apply`

## Continuity

full: economy, governance, history, factions, settlements, continuity, and migration are linked into a durable timeline

## Customize

1. Edit `world-contract.toml` authority rules and receipts.
2. Replace `genesis/state.toml` with project-specific starting state.
3. Update `projection/projection.toml` and assets for the desired player/operator view.
4. Build the World Package and submit it through certification level 4.
