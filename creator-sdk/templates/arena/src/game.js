export const game = {
  name: 'Arena',
  genre: 'competitive-arena',
  features: ["Players", "Combat", "Inventory", "Match Flow", "Score Tracking"],
  canonicalGameplayActions: [
    {
      type: 'PlayerJoin',
      input: { player_id: 'player-1', action: 'join' },
      mutates: ['players', 'positions', 'health', 'scores', 'tick']
    },
    {
      type: 'PlayerMove',
      input: { player_id: 'player-1', action: 'move', direction: 'north' },
      mutates: ['positions', 'tick']
    },
    {
      type: 'PlayerAttack',
      input: { player_id: 'player-1', action: 'attack', target: 'dummy' },
      mutates: ['health', 'scores', 'tick']
    },
    {
      type: 'ScoreUpdate',
      input: { player_id: 'player-1', action: 'score_update', score_delta: 5 },
      mutates: ['scores', 'tick']
    }
  ],
  runtimeState: ['players', 'positions', 'health', 'scores', 'tick'],
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
