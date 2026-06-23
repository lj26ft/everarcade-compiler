export type WorldRecord = {
  id: string; name: string; description: string; category: string; proofStatus: string;
  population: string; governance: string; operator: string; lineage: string;
  capabilities: string[]; treasury: string; opportunities: string[]; worldEvr: string;
};

export const worlds: WorldRecord[] = [
  { id: 'arena-vanguard', name: 'Arena Vanguard', description: 'A live arena world proving replay, restore, and migration workflows.', category: 'Arena', proofStatus: 'Replay verified; restore/migration fixture-witnessed', population: 'Early operators and test players', governance: 'Operator-led with World Git contributions', operator: 'EverArcade reference operator', lineage: 'Founding candidate → verified world', capabilities: ['Replay verifier', 'Restore bundle', 'Migration link'], treasury: 'Execution helpers implemented; settlement tests blocked by vendored dependency issue', opportunities: ['Combat balance', 'Observer UI', 'Verifier review'], worldEvr: 'world.evr://arena-vanguard' },
  { id: 'frontier-settlement', name: 'Frontier Settlement RC1', description: 'The first persistent Founding World reference implementation: World Factory generated, replay verified, remotely verified, release-attested, and ready for new users to verify and join.', category: 'Civilization', proofStatus: 'Package PASS; replay PASS; remote PASS; attestation PASS; deployment RUNNING', population: 'Founding operators, creators, and verifier reviewers', governance: 'Council proposal loop using governance.vote', operator: 'frontier-settlement-operator-rc1', lineage: 'World Factory Frontier Settlement template → RC1 release attestation', capabilities: ['inventory.transfer', 'market.trade', 'governance.vote', 'Public proof bundle', 'Operator workspace'], treasury: 'Settlement treasury fixture with deterministic market receipts', opportunities: ['Economy design', 'Governance scenarios', 'Settlement content', 'Verification review', 'Operator onboarding'], worldEvr: 'world.evr://frontier-settlement-demo' },
  { id: 'marketplace-demo', name: 'Marketplace Demo World', description: 'A creator SDK fixture showing how capabilities appear in world discovery.', category: 'Marketplace', proofStatus: 'Implemented; capability tests blocked by vendored dependency issue', population: 'SDK maintainers', governance: 'Maintainer review', operator: 'Creator SDK fixture', lineage: 'Template fixture', capabilities: ['Capability marketplace', 'Reward model'], treasury: 'Demo rewards only', opportunities: ['Capability authors', 'Docs examples'], worldEvr: 'world.evr://marketplace-demo' }
];

export const capabilities = [
  { name: 'Replay Verifier', author: 'EverArcade', version: '0.1', worldsUsing: 1, rewardModel: 'Public proof infrastructure', verification: 'Independently reproduced over production journals' },
  { name: 'Restore Bundle', author: 'EverArcade', version: '0.1', worldsUsing: 1, rewardModel: 'Operator reliability', verification: 'Fixture-witnessed with receipt accumulator' },
  { name: 'Treasury Helper', author: 'EverArcade', version: '0.1', worldsUsing: 2, rewardModel: 'World policy execution', verification: 'Implemented; test blocked by vendored dependency issue' }
];
