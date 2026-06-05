export function marketplaceSale(assetId, price) { return { kind: 'marketplaceSale', assetId, price, productionPayments: false }; }
export function creatorRoyalty(creator, basisPoints) { return { kind: 'creatorRoyalty', creator, basisPoints }; }
export function premiumAsset(assetId, entitlement) { return { kind: 'premiumAsset', assetId, entitlement }; }
export function subscription(planId, benefits = []) { return { kind: 'subscription', planId, benefits, productionBilling: false }; }
export function tournamentReward(eventId, reward) { return { kind: 'tournamentReward', eventId, reward }; }
