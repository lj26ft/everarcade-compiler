# Frontier World Template

## Purpose

Persistent survival and exploration worlds.

## Demonstrates

The world remembers.

## Included RustRigs

- `identity.join`
- `position.move`
- `resource.harvest`
- `inventory.transfer`
- `structure.build`
- `structure.repair`
- `structure.decay`
- `continuity.advance`
- `continuity.record_event`

## Continuity

persistent: aging, ruins, settlement history, and event records are advanced on scheduled ticks

## Customize

1. Edit `world-contract.toml` authority rules and receipts.
2. Replace `genesis/state.toml` with project-specific starting state.
3. Update `projection/projection.toml` and assets for the desired player/operator view.
4. Build the World Package and submit it through certification level 3.
