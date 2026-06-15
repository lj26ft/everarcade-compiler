# Developers

Build worlds, not just matches. EverArcade gives developers a path from local experiments to portable world packages.

## What can I build?

You can build persistent worlds with inventories, economies, quests, governance, and shared history. The runtime keeps world state authoritative while clients, tools, and renderers stay replaceable.

## How do I create a world?

1. Start with the [Creator SDK](/docs/creator-sdk/quick-start).
2. Pick or adapt a [template](/docs/game-templates/).
3. Define world rules in a [World Contract](/docs/world-contracts/).
4. Use [RustRigs](/docs/rustrigs/) for common gameplay mutations.
5. Run locally with the [developer quickstart](/docs/developer/quickstart).
6. Package and deploy with the [deployment guide](/docs/creator-sdk/deploy-first-game).

## Key developer concepts

- **World Contracts** are the rulebooks for sovereign worlds.
- **RustRigs** are canonical gameplay mutation libraries such as `combat.attack()`, `inventory.transfer()`, `market.trade()`, and `quest.complete()`.
- **Runtime packages** bundle the world, contract, assets, and metadata needed to run elsewhere.

## Required links

- [Creator SDK](/docs/creator-sdk/quick-start)
- [Templates](/docs/game-templates/)
- [Runtime Packages](/docs/canonical-package-format)
- [World Contracts](/docs/world-contracts/)
- [RustRigs](/docs/rustrigs/)
