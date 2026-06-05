const template = {
  "id": "civilization",
  "name": "Civilization",
  "features": [
    "Settlements",
    "Citizens",
    "Governance",
    "Economy",
    "Regions"
  ],
  "systems": {
    "settlements": [
      "capital",
      "outpost",
      "resource-node"
    ],
    "citizens": [
      "worker",
      "builder",
      "delegate"
    ],
    "governance": [
      "proposal",
      "vote",
      "policy"
    ],
    "economy": [
      "food",
      "ore",
      "civic-credit"
    ],
    "regions": [
      "plains",
      "forest",
      "coast"
    ],
    "civilizationRuntime": [
      "turn-scheduler",
      "region-state"
    ],
    "governanceRuntime": [
      "proposal-lifecycle",
      "quorum-check"
    ],
    "federationRuntime": [
      "observer-sync",
      "regional-receipts"
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
