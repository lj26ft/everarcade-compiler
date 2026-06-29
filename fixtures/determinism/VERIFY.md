# Determinism Fixture Verification

These files are intentionally small so independent verifiers can rederive every root without repository-specific build products.

## Files

- `world-spec.json`: genesis world state.
- `inputs.json`: ordered fixture inputs.
- `expected-roots.json`: expected `state_root`, `receipt_root`, `continuity_root`, `world_hash`, and `checkpoint_hash`.

## Verification command

Run from the repository root:

```sh
node <<'NODE'
const { createHash } = require('node:crypto');
const { readFileSync } = require('node:fs');
const zero = '0'.repeat(64);
function canon(v) {
  if (v === null || typeof v !== 'object') return JSON.stringify(v);
  if (Array.isArray(v)) return '[' + v.map(canon).join(',') + ']';
  return '{' + Object.keys(v).sort().map((k) => JSON.stringify(k) + ':' + canon(v[k])).join(',') + '}';
}
function hashBytes(parts) {
  const h = createHash('sha256');
  for (const part of parts) h.update(part);
  return h.digest('hex');
}
function hashTaggedJson(tag, obj) {
  return hashBytes([Buffer.from(tag, 'utf8'), Buffer.from([0]), Buffer.from(canon(obj), 'utf8')]);
}
function hashTaggedHex(tag, hexes) {
  return hashBytes([Buffer.from(tag, 'utf8'), Buffer.from([0]), ...hexes.map((hex) => Buffer.from(hex, 'hex'))]);
}
const genesis = JSON.parse(readFileSync('fixtures/determinism/world-spec.json', 'utf8'));
const { inputs } = JSON.parse(readFileSync('fixtures/determinism/inputs.json', 'utf8'));
const expected = JSON.parse(readFileSync('fixtures/determinism/expected-roots.json', 'utf8'));
let state = JSON.parse(JSON.stringify(genesis));
let receiptHashes = [];
let previousStateRoot = hashTaggedJson('everarcade.state_root.v1', state);
for (const input of inputs) {
  const pre = previousStateRoot;
  state.tick = input.tick;
  if (input.action === 'score') {
    state.players.find((p) => p.player_id === input.actor).score += input.points;
  } else if (input.action === 'move') {
    let pos = state.positions.find((p) => p.entity_id === input.actor);
    if (!pos) {
      pos = { entity_id: input.actor, x: 0, y: 0, z: 0, rotation: 0 };
      state.positions.push(pos);
    }
    pos.x += input.dx;
    pos.y += input.dy;
    state.positions.sort((a, b) => a.entity_id.localeCompare(b.entity_id));
  } else {
    throw new Error(`unknown action: ${input.action}`);
  }
  const postNoReceipts = hashTaggedJson('everarcade.state_root.v1', { ...state, receipts: genesis.receipts, continuity: genesis.continuity });
  const receipt = { schema_version: 1, world_id: state.world_id, tick: input.tick, input, pre_state_root: pre, post_state_root: postNoReceipts, status: 'ok' };
  receiptHashes.push(hashTaggedJson('everarcade.receipt.v1', receipt));
  previousStateRoot = postNoReceipts;
}
const receipt_root = receiptHashes.length ? hashTaggedHex('everarcade.receipt_root.v1', receiptHashes) : zero;
const replay_root = hashTaggedHex('everarcade.replay_root.v1', receiptHashes);
const continuityInput = { schema_version: 1, world_id: state.world_id, epoch: 0, previous_state_root: hashTaggedJson('everarcade.state_root.v1', genesis), receipt_root, replay_root, migration_root: null, terminal_tick: state.tick };
const continuity_root = hashTaggedJson('everarcade.continuity_root.v1', continuityInput);
state.receipts = { receipt_root, receipt_count: receiptHashes.length, last_receipt_hash: receiptHashes.at(-1) };
state.continuity = { continuity_root, previous_state_root: continuityInput.previous_state_root, replay_root, migration_root: null, epoch: 0 };
const state_root = hashTaggedJson('everarcade.state_root.v1', state);
const world_hash = hashTaggedHex('everarcade.world_hash.v1', [state_root, receipt_root, continuity_root]);
const checkpoint_hash = hashTaggedHex('everarcade.checkpoint_hash.v1', [world_hash, state_root, receipt_root, continuity_root]);
const actual = { state_root, receipt_root, continuity_root, world_hash, checkpoint_hash };
console.log(JSON.stringify(actual, null, 2));
for (const [key, value] of Object.entries(actual)) {
  if (value !== expected[key]) throw new Error(`${key} mismatch: ${value} !== ${expected[key]}`);
}
console.log('byte-identical');
NODE
```

## Cross-build and cross-process checks

For this fixture, the verifier is independent of optimized build artifacts. Cross-build determinism is recorded by running the same root derivation in debug and release validation modes and comparing identical root bytes. Cross-process determinism is recorded by invoking the command in independent processes and requiring `byte-identical` output every time.
