export function region(id, biome) { return { kind: 'region', id, biome }; }
export function world(id, regions = []) { return { kind: 'world', id, regions }; }
export function settlement(id, regionId, population = 0) { return { kind: 'settlement', id, regionId, population }; }
export function governance(id, rules = []) { return { kind: 'governance', id, rules }; }
export function localEconomy(id, currencies = []) { return { kind: 'economy', id, currencies }; }
