#!/usr/bin/env node
const { createHash } = require('node:crypto');
const { cpSync, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } = require('node:fs');
const { join, basename } = require('node:path');
const { tmpdir } = require('node:os');
const { spawnSync } = require('node:child_process');

const REPORT_DIR = join(process.cwd(), 'reports', 'js-kernel-equivalence');
const FIXTURE_DIR = join(process.cwd(), 'proofs', 'handoff-v1', 'canonical-fixtures');
const LIVE_CANDIDATES = [
  join(process.cwd(), 'reports/hotpocket-live/node-a/work/state/arena-wrapper-state.json'),
  join(process.cwd(), 'deployments/arena-vanguard-consensus/node-a/evernode/hotpocket/arena-wrapper-state.json')
];
const TOP_FIELDS = ['schema_version','world_id','arena_id','tick','players','entities','positions','health','receipts','continuity','metadata'];
const PLAYER_FIELDS = ['player_id','controller_id','join_tick','status','score','metadata'];
const ENTITY_FIELDS = ['entity_id','entity_type','owner_player_id','spawn_tick','despawn_tick','attributes'];
const POSITION_FIELDS = ['entity_id','x','y','z','rotation'];
const HEALTH_FIELDS = ['entity_id','current','maximum'];
const RECEIPT_FIELDS = ['receipt_root','receipt_count','last_receipt_hash'];
const CONTINUITY_FIELDS = ['continuity_root','previous_state_root','replay_root','migration_root','epoch'];
const METADATA_FIELDS = ['ruleset_id','ruleset_version','created_by','labels','extensions'];
const ZERO = '0'.repeat(64);

function sha256(s) { return createHash('sha256').update(s).digest('hex'); }
function utf8Compare(a, b) { return Buffer.compare(Buffer.from(a, 'utf8'), Buffer.from(b, 'utf8')); }
function rootBytes(root, label) { if (!/^[0-9a-f]{64}$/.test(root)) throw new Error(`${label} must be 64 lowercase hex characters`); return Buffer.from(root, 'hex'); }
function sortValue(v) {
  if (v === undefined) return null;
  if (v === null || typeof v !== 'object') return v;
  if (Array.isArray(v)) return v.map(sortValue);
  return Object.fromEntries(Object.keys(v).sort(utf8Compare).map((k) => [k, sortValue(v[k])]));
}
function orderObject(input, fields) {
  const out = {};
  for (const f of fields) out[f] = input[f];
  return out;
}
function normalizeArenaState(input) {
  return orderObject({
    ...input,
    players: (input.players || []).map((p) => orderObject({ ...p, metadata: sortValue(p.metadata || {}) }, PLAYER_FIELDS)).sort((a, b) => utf8Compare(a.player_id, b.player_id)),
    entities: (input.entities || []).map((e) => orderObject({ ...e, attributes: sortValue(e.attributes || {}) }, ENTITY_FIELDS)).sort((a, b) => utf8Compare(a.entity_id, b.entity_id)),
    positions: (input.positions || []).map((p) => orderObject(p, POSITION_FIELDS)).sort((a, b) => utf8Compare(a.entity_id, b.entity_id)),
    health: (input.health || []).map((h) => orderObject(h, HEALTH_FIELDS)).sort((a, b) => utf8Compare(a.entity_id, b.entity_id)),
    receipts: orderObject(input.receipts || {}, RECEIPT_FIELDS),
    continuity: orderObject(input.continuity || {}, CONTINUITY_FIELDS),
    metadata: orderObject({ ...(input.metadata || {}), labels: [...((input.metadata || {}).labels || [])].sort(utf8Compare), extensions: sortValue((input.metadata || {}).extensions || {}) }, METADATA_FIELDS)
  }, TOP_FIELDS);
}
function canonicalizeArenaState(state) { return JSON.stringify(normalizeArenaState(state)); }
function stateRoot(state) { return sha256(canonicalizeArenaState(state)); }
function worldHash(stateRoot, receiptRoot, continuityRoot) { return createHash('sha256').update(Buffer.concat([rootBytes(stateRoot, 'state_root'), rootBytes(receiptRoot, 'receipt_root'), rootBytes(continuityRoot, 'continuity_root')])).digest('hex'); }
let stagedKernelDir;
function kernelManifest() {
  if (!stagedKernelDir) {
    stagedKernelDir = mkdtempSync(join(tmpdir(), 'everarcade-canonicalizer-kernel-'));
    cpSync(join(process.cwd(), 'crates', 'canonicalizer-kernel'), stagedKernelDir, { recursive: true });
  }
  return join(stagedKernelDir, 'Cargo.toml');
}
function kernel(state) {
  const r = spawnSync('cargo', ['run','-q','--manifest-path', kernelManifest(),'--bin','canonicalizer-kernel-cli','--','state'], { input: JSON.stringify(state), encoding: 'utf8', cwd: stagedKernelDir, env: { ...process.env, CARGO_NET_OFFLINE: 'false' } });
  if (r.status !== 0) throw new Error(`kernel failed: ${r.stderr || r.stdout}`);
  return JSON.parse(r.stdout);
}
process.on('exit', () => { if (stagedKernelDir) rmSync(stagedKernelDir, { recursive: true, force: true }); });
function write(name, value) { mkdirSync(REPORT_DIR, { recursive: true }); writeFileSync(join(REPORT_DIR, name), typeof value === 'string' ? value : `${JSON.stringify(value, null, 2)}\n`); }
function fixtureIds() { return [...new Set(require('node:fs').readdirSync(FIXTURE_DIR).map(f => (f.match(/^fixture-(\d+)-state\.json$/)||[])[1]).filter(Boolean))].sort(); }
function livePath() { const p = LIVE_CANDIDATES.find(existsSync); if (!p) throw new Error('missing arena-wrapper-state.json'); return p; }
function projectLiveSnapshot(snapshot, sourcePath) {
  const s = snapshot.state || snapshot;
  const latest = (s.commitments || []).at(-1) || {};
  const previous = (s.commitments || []).at(-2) || {};
  const ids = Object.keys(s.players || {}).sort(utf8Compare);
  return normalizeArenaState({ schema_version:1, world_id:'arena-vanguard', arena_id:'arena-vanguard-live', tick:s.tick || 0,
    players: ids.map(id => ({ player_id:id, controller_id:id, join_tick:0, status:s.players[id].connected ? 'connected' : 'disconnected', score:s.players[id].score || 0, metadata:{ live_player_id:s.players[id].id } })),
    entities: ids.map(id => ({ entity_id:id, entity_type:'player', owner_player_id:id, spawn_tick:0, despawn_tick:null, attributes:{ connected:!!s.players[id].connected } })),
    positions: ids.map(id => ({ entity_id:id, x:s.players[id].x || 0, y:s.players[id].y || 0, z:0, rotation:0 })),
    health: ids.map(id => ({ entity_id:id, current:s.players[id].health || 0, maximum:100 })),
    receipts: { receipt_root: latest.receipt_root || ZERO, receipt_count: (snapshot.receipts || []).length, last_receipt_hash: (snapshot.receipts || []).at(-1)?.receipt_hash || null },
    continuity: { continuity_root: latest.continuity_root || ZERO, previous_state_root: previous.state_root || null, replay_root: sha256(JSON.stringify(snapshot.journal || [])), migration_root:null, epoch:0 },
    metadata: { ruleset_id:'arena-vanguard', ruleset_version:1, created_by:null, labels:['live-persisted','arena-wrapper-state'], extensions:{ source_path:sourcePath, live_tick:s.tick || 0, live_state_root: latest.state_root || null, live_world_hash: latest.world_hash || null } }
  });
}
function compare(label, state) {
  const jsBytes = canonicalizeArenaState(state); const jsRoot = stateRoot(state); const jsWorld = worldHash(jsRoot, state.receipts.receipt_root, state.continuity.continuity_root); const ko = kernel(state);
  return { label, match: { canonical_bytes: Buffer.from(jsBytes).toString('hex') === ko.canonical_hex, state_root: jsRoot === ko.state_root, world_hash: jsWorld === ko.world_hash }, js: { canonical_hex: Buffer.from(jsBytes).toString('hex'), state_root: jsRoot, world_hash: jsWorld }, rust_kernel: ko };
}

const fixtures = fixtureIds().map(id => compare(`fixture-${id}`, JSON.parse(readFileSync(join(FIXTURE_DIR, `fixture-${id}-state.json`), 'utf8'))));
const lp = livePath();
const liveState = projectLiveSnapshot(JSON.parse(readFileSync(lp, 'utf8')), lp);
const live = compare(`live:${basename(lp)}`, liveState);
const all = [...fixtures, live];
const ok = all.every(r => r.match.canonical_bytes && r.match.state_root && r.match.world_hash);
write('fixture-equivalence.json', { fixtures });
write('live-state-equivalence.json', { source: lp, projected_arena_state: liveState, comparison: live });
write('canonical-byte-comparison.json', all.map(r => ({ label:r.label, match:r.match.canonical_bytes, js_canonical_hex:r.js.canonical_hex, rust_canonical_hex:r.rust_kernel.canonical_hex })));
write('state-root-comparison.json', all.map(r => ({ label:r.label, match:r.match.state_root, js_state_root:r.js.state_root, rust_state_root:r.rust_kernel.state_root })));
write('world-hash-comparison.json', all.map(r => ({ label:r.label, match:r.match.world_hash, js_world_hash:r.js.world_hash, rust_world_hash:r.rust_kernel.world_hash, receipt_root:r.rust_kernel.receipt_root, continuity_root:r.rust_kernel.continuity_root })));
write('equivalence-certification-report.txt', ['EverArcade JS ↔ Kernel Equivalence Certification','',`Fixtures checked: ${fixtures.length}`,`Live state: ${lp}`,'','Checks:',`Canonical bytes match: ${all.every(r=>r.match.canonical_bytes) ? 'PASS' : 'FAIL'}`,`State roots match: ${all.every(r=>r.match.state_root) ? 'PASS' : 'FAIL'}`,`World hashes match: ${all.every(r=>r.match.world_hash) ? 'PASS' : 'FAIL'}`,'','Proof Boundary: ArenaState -> canonical bytes -> state_root -> world_hash','',`JS KERNEL EQUIVALENCE CERTIFICATION: ${ok ? 'PASS' : 'FAIL'}`,''].join('\n'));
if (!ok) process.exitCode = 1;
console.log(`JS KERNEL EQUIVALENCE CERTIFICATION: ${ok ? 'PASS' : 'FAIL'}`);
