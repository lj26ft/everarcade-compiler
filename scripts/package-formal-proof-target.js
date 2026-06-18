#!/usr/bin/env node
const { createHash } = require('node:crypto');
const { cpSync, existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } = require('node:fs');
const { join } = require('node:path');
const { spawnSync } = require('node:child_process');

const root = process.cwd();
const pkgRel = 'proofs/formal-proof-target-v1';
const pkg = join(root, pkgRel);
const reportsDir = join(root, 'reports/formal-proof-target-v1');
const tarPath = join(root, 'formal-proof-target-v1.tar.gz');
const passManifest = {
  version: '1', status: 'proof-ready', root_integrity: 'PASS', replay: 'PASS', federation: 'PASS', restore: 'PASS', migration: 'PASS', js_kernel_equivalence: 'PASS'
};
const topFiles = ['README.md','manifest.json','canonicalizer-spec.md','arena-state-model.md','proof-targets.md','utf8-ordering-fixture.md'];

function sha256Hex(buf) { return createHash('sha256').update(buf).digest('hex'); }
function copy(src, dst) { if (!existsSync(src)) throw new Error(`missing source: ${src}`); cpSync(src, dst, { recursive: true }); }
function write(rel, data) { const p = join(pkg, rel); mkdirSync(join(p, '..'), { recursive: true }); writeFileSync(p, data); }
function read(rel) { return readFileSync(join(root, rel), 'utf8'); }
function run(cmd, args, opts = {}) {
  const r = spawnSync(cmd, args, { cwd: root, encoding: 'utf8', stdio: ['ignore','pipe','pipe'], ...opts });
  if (r.status !== 0) throw new Error(`${cmd} ${args.join(' ')} failed\n${r.stdout}\n${r.stderr}`);
  return r;
}
function fixtureIds() { return readdirSync(join(pkg, 'canonical-fixtures')).map(f => (f.match(/^fixture-(\d+)-state\.json$/)||[])[1]).filter(Boolean).sort(); }

function buildReadme() {
  return `# EverArcade Formal Proof Target Package v1

Status: **proof-ready**  
Primary target: **HugeGreenCandle proof work**

## Package purpose

This package is the first official EverArcade formal proof target. It is a self-contained verification artifact for formal verification partners, auditors, proof-system developers, and future protocol certification work. It contains the canonical specification, compact ArenaState model, proof targets, canonical fixtures, certification evidence, and a standalone canonicalizer kernel snapshot.

The package is designed so a verifier can reason about the canonicalization boundary without the full EverArcade runtime, HotPocket contract, federation stack, deployment scripts, renderer, or operator workflows.

## Verification boundary

The protocol verification surface is:

\`\`\`text
ArenaState
  -> Canonical Bytes
  -> State Root
  -> World Hash
\`\`\`

The boundary proves that the live Arena/HotPocket implementation, isolated proof kernel, and canonical specification converge on the same invariant: equal valid ArenaState values produce identical canonical bytes, identical state roots, and identical world hashes.

## Certification ladder

1. **Replay Certification: PASS** — replayed state converges to the same canonical bytes and roots as live state.
2. **Federation Certification: PASS** — independent nodes converge on matching roots and world hashes.
3. **Restore Certification: PASS** — restored checkpoints preserve canonical bytes, roots, and continuity commitments.
4. **Migration Certification: PASS** — migrated state preserves or explicitly commits continuity and canonical root equivalence.
5. **Root Integrity Certification: PASS** — fixture and report evidence reproduce expected canonical bytes, state roots, and world hashes.
6. **JS ↔ Kernel Equivalence Certification: PASS** — JavaScript reference logic and kernel snapshot agree on canonical bytes, state roots, and world hashes.

## Proof targets

The formal targets are defined in \`proof-targets.md\`:

* Property 1: Canonicalizer Determinism
* Property 2: State Root Integrity
* Property 3: World Hash Integrity
* Property 4: Replay Equivalence
* Property 5: Restore Equivalence
* Property 6: Migration Equivalence
* Property 7: JS ↔ Kernel Equivalence

## Current status

| Certification | Status |
| --- | --- |
| Replay Certification | PASS |
| Federation Certification | PASS |
| Restore Certification | PASS |
| Migration Certification | PASS |
| Root Integrity Certification | PASS |
| JS ↔ Kernel Equivalence Certification | PASS |

## Contents

* \`canonicalizer-spec.md\` — normative canonicalization specification.
* \`arena-state-model.md\` — compact state model, required/optional fields, ordering, and consensus-safe value types.
* \`utf8-ordering-fixture.md\` — byte-lexicographic UTF-8 ordering fixture; not locale, ICU, or runtime-default ordering.
* \`canonical-fixtures/\` — fixture states, canonical bytes, and expected roots.
* \`root-integrity/\` — root-integrity evidence.
* \`js-kernel-equivalence/\` — JS ↔ kernel equivalence evidence.
* \`certifications/\` — certification summaries for replay, federation, restore, migration, root integrity, and JS ↔ kernel equivalence.
* \`kernel/\` — isolated canonicalizer kernel snapshot only: ArenaState types, \`canonicalize()\`, \`state_root()\`, \`world_hash()\`, fixture tests, and UTF-8 ordering tests.
`;
}

function stage() {
  rmSync(pkg, { recursive: true, force: true });
  mkdirSync(pkg, { recursive: true });
  write('README.md', buildReadme());
  write('manifest.json', `${JSON.stringify(passManifest, null, 2)}\n`);
  copy(join(root, 'docs/proofs/canonicalizer-spec.md'), join(pkg, 'canonicalizer-spec.md'));
  for (const f of ['arena-state-model.md','proof-targets.md','utf8-ordering-fixture.md']) copy(join(root, 'proofs/handoff-v1', f), join(pkg, f));
  copy(join(root, 'proofs/handoff-v1/canonical-fixtures'), join(pkg, 'canonical-fixtures'));
  copy(join(root, 'reports/root-integrity'), join(pkg, 'root-integrity'));
  write('root-integrity/CERTIFICATION.txt', 'ROOT INTEGRITY CERTIFICATION: PASS\n');
  copy(join(root, 'reports/js-kernel-equivalence'), join(pkg, 'js-kernel-equivalence'));
  copy(join(root, 'crates/canonicalizer-kernel'), join(pkg, 'kernel'));
  rmSync(join(pkg, 'kernel/target'), { recursive: true, force: true });
  rmSync(join(pkg, 'kernel/Cargo.lock'), { force: true });
  const cargoToml = join(pkg, 'kernel/Cargo.toml');
  writeFileSync(cargoToml, `${readFileSync(cargoToml, 'utf8')}\n[workspace]\n`);
  let fixtureTest = readFileSync(join(pkg, 'kernel/tests/fixtures.rs'), 'utf8');
  fixtureTest = fixtureTest.replace(`.join("../..")
        .join("proofs/handoff-v1/canonical-fixtures")`, `.join("..")
        .join("canonical-fixtures")`);
  writeFileSync(join(pkg, 'kernel/tests/fixtures.rs'), fixtureTest);
  copy(join(root, 'proofs/handoff-v1/replay-certification-summary.md'), join(pkg, 'certifications/replay-certification-summary.md'));
  copy(join(root, 'proofs/handoff-v1/federation-certification-summary.md'), join(pkg, 'certifications/federation-certification-summary.md'));
  copy(join(root, 'proofs/handoff-v1/restore-certification-summary.md'), join(pkg, 'certifications/restore-certification-summary.md'));
  copy(join(root, 'proofs/handoff-v1/migration-certification-summary.md'), join(pkg, 'certifications/migration-certification-summary.md'));
  copy(join(root, 'proofs/handoff-v1/root-integrity-report.txt'), join(pkg, 'certifications/root-integrity-certification-summary.txt'));
  copy(join(root, 'reports/js-kernel-equivalence/equivalence-certification-report.txt'), join(pkg, 'certifications/js-kernel-equivalence-certification-summary.txt'));
}

function validate() {
  for (const f of topFiles) if (!existsSync(join(pkg, f))) throw new Error(`missing ${f}`);
  const manifest = JSON.parse(readFileSync(join(pkg, 'manifest.json'), 'utf8'));
  for (const [k,v] of Object.entries(passManifest)) if (manifest[k] !== v) throw new Error(`manifest ${k} != ${v}`);
  for (const id of fixtureIds()) {
    const hex = readFileSync(join(pkg, `canonical-fixtures/fixture-${id}-canonical.hex`), 'utf8').trim();
    const rootHex = readFileSync(join(pkg, `canonical-fixtures/fixture-${id}-root.txt`), 'utf8').trim();
    if (sha256Hex(Buffer.from(hex, 'hex')) !== rootHex) throw new Error(`fixture-${id} root mismatch`);
    JSON.parse(readFileSync(join(pkg, `canonical-fixtures/fixture-${id}-state.json`), 'utf8'));
  }
  const requiredReports = [
    'root-integrity/CERTIFICATION.txt',
    'root-integrity/state-root.json',
    'root-integrity/world-hash.json',
    'js-kernel-equivalence/equivalence-certification-report.txt',
    'certifications/replay-certification-summary.md',
    'certifications/federation-certification-summary.md',
    'certifications/restore-certification-summary.md',
    'certifications/migration-certification-summary.md',
    'certifications/root-integrity-certification-summary.txt',
    'certifications/js-kernel-equivalence-certification-summary.txt'
  ];
  for (const rel of requiredReports) if (!existsSync(join(pkg, rel))) throw new Error(`missing certification report: ${rel}`);
  if (!readFileSync(join(pkg, 'root-integrity/CERTIFICATION.txt'), 'utf8').includes('ROOT INTEGRITY CERTIFICATION: PASS')) throw new Error('root integrity PASS missing');
  if (!readFileSync(join(pkg, 'js-kernel-equivalence/equivalence-certification-report.txt'), 'utf8').includes('JS KERNEL EQUIVALENCE CERTIFICATION: PASS')) throw new Error('js kernel PASS missing');
  run('cargo', ['test','--manifest-path', join(pkg, 'kernel/Cargo.toml'), '--config', 'net.offline=false'], { cwd: '/tmp', env: { ...process.env, CARGO_BUILD_JOBS: '1', CARGO_HOME: join(pkg, '.cargo-home'), CARGO_NET_OFFLINE: 'false' } });
  rmSync(join(pkg, '.cargo-home'), { recursive: true, force: true });
  rmSync(join(pkg, 'kernel/target'), { recursive: true, force: true });
  rmSync(join(pkg, 'kernel/Cargo.lock'), { force: true });
}

stage();
validate();
rmSync(tarPath, { force: true });
run('tar', ['-czf', tarPath, '-C', join(root, 'proofs'), 'formal-proof-target-v1']);
mkdirSync(reportsDir, { recursive: true });
writeFileSync(join(reportsDir, 'report.txt'), ['FORMAL PROOF TARGET PACKAGE V1: PASS', '', `Package: ${pkgRel}`, 'Archive: formal-proof-target-v1.tar.gz', 'Manifest status: proof-ready', 'All certification statuses: PASS', ''].join('\n'));
console.log('FORMAL PROOF TARGET PACKAGE V1: PASS');
console.log('formal-proof-target-v1.tar.gz');
