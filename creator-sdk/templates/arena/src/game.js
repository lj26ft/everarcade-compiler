export const game = {
  name: 'Arena',
  genre: 'competitive-arena',
  features: ["Players", "Combat", "Inventory", "Match Flow", "Score Tracking"],
  systems: {
  "players": [
    "player-spawn",
    "player-health",
    "player-input"
  ],
  "combat": [
    "hit-resolution",
    "cooldowns",
    "arena-bounds"
  ],
  "inventory": [
    "starter-weapon",
    "health-pack",
    "reward-crate"
  ],
  "matchFlow": [
    "lobby",
    "countdown",
    "active",
    "complete"
  ],
  "scoreTracking": [
    "eliminations",
    "assists",
    "survival-time"
  ],
  "renderer": [
    "topdown-camera",
    "hud-scoreboard"
  ],
  "physics": [
    "deterministic-collisions",
    "fixed-timestep"
  ],
  "economyRewards": [
    "match-win-coins",
    "participation-xp"
  ]
},
  start() {
    return 'Arena running on EverArcade Creator SDK';
  }
};

if (import.meta.url === `file://${process.argv[1]}`) {
  console.log(game.start());
}
