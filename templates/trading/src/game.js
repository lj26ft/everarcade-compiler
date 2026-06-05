const template = {
  "id": "trading",
  "name": "Trading World",
  "features": [
    "Assets",
    "Inventory",
    "Marketplace",
    "Economy",
    "Vault Integration"
  ],
  "systems": {
    "assets": [
      "demo-sword",
      "demo-pass",
      "resource-pack"
    ],
    "inventory": [
      "wallet-linked-bag",
      "escrow-slots"
    ],
    "marketplace": [
      "fixed-price-listings",
      "creator-royalty"
    ],
    "economy": [
      "gold",
      "gems",
      "fee-preview"
    ],
    "vaultIntegration": [
      "custody-record",
      "transfer-receipt"
    ],
    "creatorMonetization": [
      "royalty-bps",
      "premium-bundle"
    ],
    "assetOwnership": [
      "ownership-proof",
      "transfer-history"
    ],
    "settlementFlow": [
      "quote",
      "commit",
      "receipt"
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
