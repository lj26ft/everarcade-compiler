export function marketplaceSale(assetId, price) { return { kind: 'marketplaceSale', assetId, price, productionPayments: false }; }
export function publication(project, channel = 'creator-testnet') { return { kind: 'publication', project, channel }; }
