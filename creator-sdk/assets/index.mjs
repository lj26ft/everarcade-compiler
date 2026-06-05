export function sprite(id, file, options = {}) { return { kind: 'sprite', id, file, ...options }; }
export function texture(id, file, options = {}) { return { kind: 'texture', id, file, ...options }; }
export function audio(id, file, options = {}) { return { kind: 'audio', id, file, ...options }; }
export function itemAsset(id, file, options = {}) { return { kind: 'item', id, file, ...options }; }
export function npc(id, profile) { return { kind: 'npc', id, ...profile }; }
export function structure(id, profile) { return { kind: 'structure', id, ...profile }; }
export const exampleAssets = [
  sprite('hero', 'assets/sprites/hero.txt', { frames: 4 }),
  texture('arena-floor', 'assets/textures/arena-floor.txt'),
  audio('coin', 'assets/audio/coin.txt'),
  itemAsset('iron-sword', 'assets/items/iron-sword.json'),
  npc('trainer', { role: 'onboarding', dialogue: 'Welcome to EverArcade.' }),
  structure('market-stall', { footprint: [2, 1], category: 'marketplace' })
];
