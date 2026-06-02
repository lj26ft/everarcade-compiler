import type { WorldStateFeed } from "./types";

export function renderEnemies(feed: WorldStateFeed) {
  return feed.enemies.map(enemy => ({ kind: "Enemy", id: enemy.enemyId, health: enemy.health, status: enemy.status, zone: enemy.position.zone }));
}
