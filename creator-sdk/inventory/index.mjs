export function item(id, metadata = {}) { return { kind: 'item', id, stackable: false, ...metadata }; }
export function equipment(id, slot, stats = {}) { return { kind: 'equipment', id, slot, stats }; }
export function container(id, capacity, contents = []) { return { kind: 'container', id, capacity, contents }; }
export function vaultAsset(id, custody = 'creator-testnet') { return { kind: 'vaultAsset', id, custody, transferable: true }; }
