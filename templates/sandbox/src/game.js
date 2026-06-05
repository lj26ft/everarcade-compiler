const template = {
  "id": "sandbox",
  "name": "Sandbox",
  "features": [
    "World",
    "Inventory",
    "Marketplace",
    "Economy",
    "Events"
  ],
  "systems": {
    "world": [
      "freeform-zones",
      "entity-spawn",
      "tags"
    ],
    "inventory": [
      "generic-items",
      "collections"
    ],
    "marketplace": [
      "optional-listings",
      "demo-entitlements"
    ],
    "economy": [
      "soft-currency",
      "reward-hooks"
    ],
    "events": [
      "onJoin",
      "onInteract",
      "onReward"
    ],
    "extensionPoints": [
      "systems",
      "assets",
      "logic",
      "content"
    ]
  }
};

export function createGame(overrides = {}) {
  return {
    ...template,
    ...overrides,
    status: 'playable',
    run() {
      return `${template.name} running with ${template.features.length} canonical feature groups`;
    }
  };
}

export const game = createGame();

if (import.meta.url === `file://${process.argv[1]}`) {
  console.log(game.run());
}
