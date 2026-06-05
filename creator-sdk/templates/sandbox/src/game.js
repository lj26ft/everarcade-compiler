export const game = {
  name: 'Sandbox',
  genre: 'sandbox',
  features: ["World", "Inventory", "Marketplace", "Economy", "Events"],
  systems: {
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
},
  start() {
    return 'Sandbox running on EverArcade Creator SDK';
  }
};

if (import.meta.url === `file://${process.argv[1]}`) {
  console.log(game.start());
}
