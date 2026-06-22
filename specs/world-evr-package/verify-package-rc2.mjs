#!/usr/bin/env node
import { createHash } from 'node:crypto';
import { readdirSync, readFileSync, existsSync, statSync } from 'node:fs';
import { join, relative, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = dirname(fileURLToPath(import.meta.url));
const hex = (bytes) => createHash('sha256').update(bytes).digest('hex');
const json = (dir, path) => JSON.parse(readFileSync(join(dir, path), 'utf8'));
const fileHex = (dir, path) => hex(readFileSync(join(dir, path)));
const sha = (dir, path) => `sha256:${fileHex(dir, path)}`;

function filesUnder(dir, prefix = '') {
  const out = [];
  for (const name of readdirSync(join(dir, prefix))) {
    const p = prefix ? `${prefix}/${name}` : name;
    const s = statSync(join(dir, p));
    if (s.isDirectory()) out.push(...filesUnder(dir, p));
    else out.push(p);
  }
  return out.sort((a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b)));
}

function assert(c, msg) { if (!c) throw new Error(msg); }
function verify(dir) {
  const hm = json(dir, 'hash-manifest.json');
  assert(hm.hash_alg === 'sha256', 'hash-manifest.hash_alg must be sha256');
  assert(hm.file_order === 'lexicographic-by-path', 'hash-manifest.file_order must be lexicographic-by-path');
  const paths = hm.files.map(f => f.path);
  const sorted = [...paths].sort((a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b)));
  assert(paths.every((p, i) => p === sorted[i]), 'hash-manifest.files must be sorted byte-lex by path');
  for (const e of hm.files) {
    assert(existsSync(join(dir, e.path)), `hash-manifest path missing: ${e.path}`);
    assert(e.sha256 === fileHex(dir, e.path), `hash mismatch for ${e.path}`);
  }
  const listed = new Set(paths);
  for (const p of filesUnder(dir)) {
    if (p === 'hash-manifest.json' || p === 'expected-package-hash.txt' || p === 'expected-failure.txt') continue;
    assert(listed.has(p), `load-bearing file outside hash-manifest: ${p}`);
  }
  const stream = hm.files.map(e => `${e.path}\0${e.sha256}\n`).join('');
  const packageHash = hex(Buffer.from(stream, 'utf8'));
  assert(readFileSync(join(dir, 'expected-package-hash.txt'), 'utf8').trim() === packageHash, 'expected-package-hash mismatch');

  const manifest = json(dir, 'manifest.json');
  const genesis = json(dir, 'genesis/genesis.json');
  const runtime = json(dir, 'runtime/runtime.json');
  const contract = json(dir, 'world-contract/world-contract.json');
  assert(manifest.spec_version === 'RC2', 'manifest.spec_version must be RC2');
  assert(!manifest.must_understand, 'unknown must-understand field present');
  for (const p of manifest.required_files ?? []) assert(existsSync(join(dir, p)), `required file missing: ${p}`);
  assert(manifest.world_id === genesis.world_id, 'manifest.world_id must equal genesis.world_id');
  assert(manifest.runtime.runtime_id === runtime.runtime_id, 'manifest.runtime.runtime_id must equal runtime.runtime_id');
  assert(manifest.runtime.version === runtime.version, 'manifest.runtime.version must equal runtime.version');
  assert(manifest.world_contract.contract_id === contract.contract_id, 'manifest.world_contract.contract_id must equal contract.contract_id');
  assert(manifest.genesis.sha256 === fileHex(dir, 'genesis/genesis.json'), 'manifest.genesis.sha256 mismatch');
  assert(manifest.runtime.sha256 === fileHex(dir, 'runtime/runtime.json'), 'manifest.runtime.sha256 mismatch');
  assert(manifest.world_contract.sha256 === fileHex(dir, 'world-contract/world-contract.json'), 'manifest.world_contract.sha256 mismatch');

  if (existsSync(join(dir, 'restore/checkpoint.json'))) {
    const checkpoint = json(dir, 'restore/checkpoint.json');
    const journal = json(dir, 'restore/journal.json');
    assert(checkpoint.world_id === manifest.world_id, 'checkpoint.world_id must equal manifest.world_id');
    assert(checkpoint.root_package === manifest.package_name, 'checkpoint.root_package must equal manifest.package_name');
    assert(journal.root_package === manifest.package_name, 'journal.root_package must equal manifest.package_name');
    assert(journal.from_checkpoint === checkpoint.checkpoint_root || journal.checkpoint_root === checkpoint.checkpoint_root, 'journal must bind checkpoint root');
    assert(checkpoint.roots?.continuity_root === sha(dir, 'restore/journal.json'), 'checkpoint.roots.continuity_root must recompute from restore/journal.json');
  }
  if (existsSync(join(dir, 'proof/certification.json'))) {
    const cert = json(dir, 'proof/certification.json');
    assert(cert.world_id === manifest.world_id, 'certification.world_id must equal manifest.world_id');
    assert(cert.runtime_id === manifest.runtime.runtime_id, 'certification.runtime_id must equal manifest.runtime.runtime_id');
  }
  return packageHash;
}

const valid = join(root, 'fixtures/world-package-valid-001');
const failures = readdirSync(join(root, 'failure-fixtures')).map(n => join(root, 'failure-fixtures', n));
let ok = true;
try { console.log(`PASS valid world-package-valid-001 ${verify(valid)}`); } catch (e) { ok = false; console.error(`FAIL valid world-package-valid-001: ${e.message}`); }
for (const dir of failures) {
  try { verify(dir); ok = false; console.error(`UNEXPECTED PASS ${relative(root, dir)}`); }
  catch (e) { console.log(`PASS expected failure ${relative(root, dir)}: ${e.message}`); }
}
process.exit(ok ? 0 : 1);
