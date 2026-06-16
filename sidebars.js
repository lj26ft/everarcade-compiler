module.exports = {
  docs: [
    'index',
    { type: 'category', label: 'Choose Your Path', items: ['getting-started/index', 'developers/index', 'operators/index', 'players/index', 'contributor-guide/index'] },
    { type: 'category', label: 'Concepts', items: ['concepts/index', 'concepts/what-is-a-world', 'concepts/world-contract', 'concepts/continuity-engine', 'concepts/rustrigs'] },
    { type: 'category', label: 'Developers', items: ['GAME_DEVELOPER_START', 'creator-sdk/quick-start', 'creator-sdk/create-first-game', 'world-contracts/index', 'rustrigs/index', 'sdk/getting-started', 'game-templates/README'] },
    { type: 'category', label: 'Operators', items: ['operators/index', 'runtime-operations-manual', 'architecture/federation/federation-runtime', 'runtime/replay_verification', 'checkpoint-recovery', 'operator-recovery'] },
    { type: 'category', label: 'Architecture', link: { type: 'doc', id: 'architecture/index' }, items: ['architecture/system-overview', 'architecture/runtime-lifecycle', 'architecture/replay-verification', 'architecture/federation', 'architecture/world-packaging', 'architecture/checkpointing', 'architecture/restoration', 'architecture/xrpl-anchoring', 'architecture/evernode-integration', 'architecture/diagrams/README'] },
    { type: 'category', label: 'Archive', items: ['archive/whitepaper/index'] },
  ],
};
