const template = {
  "id": "rpg",
  "name": "RPG",
  "features": [
    "Characters",
    "Equipment",
    "Inventory",
    "NPCs",
    "Quest Framework",
    "Regions"
  ],
  "systems": {
    "characters": [
      "hero",
      "stats",
      "save-slot"
    ],
    "equipment": [
      "weapon",
      "armor",
      "trinket"
    ],
    "inventory": [
      "quest-items",
      "consumables",
      "materials"
    ],
    "npcs": [
      "quest-giver",
      "merchant",
      "trainer"
    ],
    "questFramework": [
      "accept",
      "progress",
      "complete",
      "reward"
    ],
    "regions": [
      "village",
      "wilds",
      "dungeon"
    ],
    "persistence": [
      "profile-save",
      "quest-state"
    ],
    "progression": [
      "levels",
      "skill-points"
    ],
    "marketplaceIntegration": [
      "cosmetic-listing",
      "player-trade"
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
