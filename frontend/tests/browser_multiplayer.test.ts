export function testJoinSession() {
  return { flow: ["Open Game", "Connect WebSocket", "Join Session", "Spawn Character", "Subscribe Feed", "Enter World"], authority: "runtime" };
}

export function testMovementSync() {
  return { clients: 2, action: "move", source: "WorldStateFeed", visible: ["Own Player", "Other Players"] };
}

export function testCombatSync() {
  return { clients: 2, action: "attack", enemy: "enemy-raider-1", synchronized: ["Enemy Updates", "XP", "HUD Updates"] };
}

export function testLootSync() {
  return { action: "interact", expected: "Player A loots item; item disappears for Player B", source: "runtime feed" };
}

export function testReconnectFlow() {
  return { flow: ["Disconnect", "Connection Lost", "Reconnect", "Resume Token", "Restore Session", "Continue"], duplicates: false };
}

export function testHudSync() {
  return { source: "WorldStateFeed", fields: ["Health", "Energy", "Level", "XP", "Inventory", "Quest Status"] };
}

export function testEnemySync() {
  return { lifecycle: ["Spawn", "Movement", "Combat", "Death", "Respawn"], source: "WorldStateFeed" };
}

export function testMultiplayerVisibility() {
  return { renders: ["Own Player", "Other Players", "Enemies", "Loot", "World Zones"], simulation: "none" };
}
