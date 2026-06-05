export const game = {
  name: 'RPG',
  genre: 'adventure-rpg',
  features: ["Characters", "Equipment", "Inventory", "NPCs", "Quest Framework", "Regions"],
  systems: {
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
},
  start() {
    return 'RPG running on EverArcade Creator SDK';
  }
};

if (import.meta.url === `file://${process.argv[1]}`) {
  console.log(game.start());
}
