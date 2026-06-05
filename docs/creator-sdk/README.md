# EverArcade Creator SDK v0.1

The Creator SDK is the opinionated path from idea to running EverArcade game without requiring deep knowledge of federation, settlement, XRPL, Xaman, GPU marketplace, replay, or checkpoints.

## Quick Start

```bash
node creator-sdk/cli/everarcade.mjs new --template arena --name my-arena
node creator-sdk/cli/everarcade.mjs build --project my-arena
node creator-sdk/cli/everarcade.mjs test --project my-arena
node creator-sdk/cli/everarcade.mjs deploy --project my-arena --target local
node creator-sdk/cli/everarcade.mjs publish --project my-arena --channel creator-testnet
```

## Create First Game

Start from a template instead of a blank folder. Available templates are `blank-game`, `rpg`, `arena`, `civilization`, and `marketplace-demo`.

## Build First Game

`everarcade build` reads `everarcade.game.json`, validates creator-facing defaults, and writes deterministic package artifacts into `dist/`.

## Deploy First Game

`everarcade deploy` creates a local, lease, or federation deployment descriptor. The SDK describes deployment intent and leaves authority, replay, and settlement responsibilities to protocol interfaces.

## Monetize First Game

The monetization SDK includes demo-only marketplace sales, creator royalties, premium assets, subscriptions, and tournament rewards. It intentionally does not implement production billing or production payments.

## Publish First Game

`everarcade publish` creates a publication descriptor for the creator testnet channel. Production marketplace launch is reserved for later ecosystem milestones.
