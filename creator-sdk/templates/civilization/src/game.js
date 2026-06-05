export const game = {
  name: 'Civilization',
  genre: 'civilization',
  features: ["Settlements", "Citizens", "Governance", "Economy", "Regions"],
  systems: {
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
},
  start() {
    return 'Civilization running on EverArcade Creator SDK';
  }
};

if (import.meta.url === `file://${process.argv[1]}`) {
  console.log(game.start());
}
