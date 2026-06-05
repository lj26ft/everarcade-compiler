export function currency(code, decimals = 0) { return { kind: 'currency', code, decimals }; }
export function reward(id, currencyCode, amount) { return { kind: 'reward', id, currencyCode, amount }; }
export function listing(id, assetId, price) { return { kind: 'marketplaceListing', id, assetId, price }; }
export function royalty(creator, basisPoints) { return { kind: 'royalty', creator, basisPoints }; }
export function settlementEvent(id, status = 'simulated') { return { kind: 'settlementEvent', id, status, authoritative: false }; }
