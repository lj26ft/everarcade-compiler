import test from 'node:test';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';

function runWorld(...args) {
  const result = spawnSync('node', ['creator-sdk/cli/everarcade.mjs', 'world', ...args], { encoding: 'utf8' });
  assert.equal(result.status, 0, result.stderr || result.stdout);
  return JSON.parse(result.stdout);
}

test('world registry search discovers worlds by category and tags', () => {
  const output = runWorld('search', 'governance');
  assert.equal(output.schema, 'everarcade.world-registry.search.v1');
  assert.ok(output.categories.includes('Governance'));
  assert.ok(output.worlds.some(world => world.world_id === 'frontier.evr'));
  assert.ok(output.worlds.some(world => world.trust_signals.replay_verified));
});

test('world registry lookup exposes identity metadata and health', () => {
  const output = runWorld('lookup', 'frontier.evr');
  assert.equal(output.world.world_id, 'frontier.evr');
  assert.equal(output.world.world_name, 'Frontier');
  assert.equal(output.world.proof_status.replay, 'Replay Certified');
  assert.ok(output.world.capabilities.includes('Governance Module'));
  assert.equal(output.world.health.verification_status, 'fully-certified');
});

test('world registry contributor API exposes opportunities and reputation', () => {
  const output = runWorld('contributors', 'frontier.evr');
  assert.equal(output.schema, 'everarcade.world-registry.contributors.v1');
  assert.ok(output.contributor_manifest.wanted_roles.includes('Quest Designers Needed'));
  assert.ok(output.contributors.some(contributor => contributor.reputation_score >= 90));
});

test('world registry lineage API exposes forks migrations and restores', () => {
  const output = runWorld('lineage', 'frontier.evr');
  assert.equal(output.lineage.origin_world, 'frontier.evr');
  assert.ok(output.lineage.forks.includes('frontier-classic.evr'));
  assert.ok(output.lineage.migration_history.length > 0);
  assert.ok(output.lineage.restore_events.length > 0);
});
