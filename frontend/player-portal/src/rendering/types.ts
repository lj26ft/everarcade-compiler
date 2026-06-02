export type RuntimePosition = { x: number; y: number; zone: string };
export type RuntimePlayer = { playerId: string; characterId: string; position: RuntimePosition; health: number; energy: number; xp: number; level: number; inventory: string[]; connected: boolean };
export type RuntimeEnemy = { enemyId: string; position: RuntimePosition; health: number; status: string };
export type RuntimeLoot = { lootId: string; itemId: string; position: RuntimePosition; available: boolean; claimedBy?: string | null };
export type WorldStateFeed = { tick: number; sessionId?: string; session_id?: string; players: RuntimePlayer[]; enemies: RuntimeEnemy[]; zones?: string[]; world_zones?: string[]; loot?: RuntimeLoot[]; checkpointAge?: number; checkpoint_age?: number; replaySize?: number; replay_size?: number };
