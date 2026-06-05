export const CREATOR_MARKETPLACE_VERSION = '0.1';

export function marketplaceSale(assetId, price) {
  return { kind: 'marketplaceSale', assetId, price, productionPayments: false };
}

export function publication(project, channel = 'creator-testnet') {
  return { kind: 'publication', project, channel };
}

export function creatorPublication({ publicationId, creatorId, type, version, license = 'free-starter' }) {
  return {
    kind: 'creatorPublication',
    publicationId,
    creatorId,
    type,
    version,
    license,
    status: 'published',
    versioned: true,
    nonCustodial: true,
  };
}

export function creatorInstall({ publicationId, consumerCreatorId, version }) {
  return {
    kind: 'creatorMarketplaceUsage',
    action: 'Install',
    publicationId,
    consumerCreatorId,
    version,
    replayable: true,
  };
}

export function royaltyIntent({ creatorId, publicationId, usageEventId, royaltyRateBps }) {
  return {
    kind: 'creatorRoyaltyIntent',
    creatorId,
    publicationId,
    usageEventId,
    royaltyRateBps,
    productionPayment: false,
    billing: false,
  };
}
