#!/usr/bin/env node
const assert = require('node:assert/strict');
const { existsSync, readFileSync, readdirSync, statSync, mkdirSync, writeFileSync, copyFileSync } = require('node:fs');
const { join, relative } = require('node:path');
const { ArenaVanguard, replayJournal } = require('../src/arena_vanguard');

const STATE_FILE = 'arena-wrapper-state.json';
const JOURNAL_FILE = 'arena-hotpocket-journal.json';
const ROOT_KEYS = ['state_root', 'receipt_root', 'world_hash', 'continuity_root'];

function walk(dir, matches = []) {
  for (const name of readdirSync(dir)) {
    if (name === 'node_modules' || name === '.git') continue;
    const path = join(dir, name);
    const stats = statSync(path);
    if (stats.isDirectory()) walk(path, matches);
    else if (name === STATE_FILE || name === JOURNAL_FILE) matches.push(path);
  }
  return matches;
}

function findFile(fileName) {
  const explicit = process.env[fileName === STATE_FILE ? 'ARENA_STATE_PATH' : 'ARENA_JOURNAL_PATH'];
  if (explicit) return explicit;
  const local = join(process.cwd(), 'state', fileName);
  if (existsSync(local)) return local;
  return walk(process.cwd()).find((path) => path.endsWith(fileName));
}

function readJson(path) {
  assert.ok(path, `missing ${path}`);
  return JSON.parse(readFileSync(path, 'utf8'));
}

function latestCommitments(snapshot, replayed) {
  if (snapshot.state && Array.isArray(snapshot.state.commitments) && snapshot.state.commitments.length > 0) return snapshot.state.commitments.at(-1);
  return replayed.commitments;
}

function writeReport(statePath, journalPath, snapshot, journal, comparison, replayVerification) {
  const reportDir = join(__dirname, '..', '..', '..', 'reports', 'hotpocket-live-persistence');
  mkdirSync(reportDir, { recursive: true });
  copyFileSync(statePath, join(reportDir, 'node-1-state.json'));
  copyFileSync(journalPath, join(reportDir, 'node-1-journal.json'));
  writeFileSync(join(reportDir, 'root-comparison.json'), `${JSON.stringify(comparison, null, 2)}\n`);
  writeFileSync(join(reportDir, 'replay-verification.json'), `${JSON.stringify(replayVerification, null, 2)}\n`);
  writeFileSync(join(reportDir, 'persistence-report.md'), `# HotPocket Live Persistence Report\n\n- State path: ${statePath}\n- Journal path: ${journalPath}\n- Journal entries: ${journal.length}\n- Replay status: ${replayVerification.ok ? 'PASS' : 'FAIL'}\n\n## Roots\n\n\`\`\`json\n${JSON.stringify(comparison, null, 2)}\n\`\`\`\n`);
}

const statePath = findFile(STATE_FILE);
const journalPath = findFile(JOURNAL_FILE);
assert.ok(statePath, `could not locate ${STATE_FILE} under ${process.cwd()}`);
assert.ok(journalPath, `could not locate ${JOURNAL_FILE} under ${process.cwd()}`);

const snapshot = readJson(statePath);
const journal = readJson(journalPath);
assert.ok(Array.isArray(journal), 'journal must be an array');
const replayed = replayJournal(journal);
const live = latestCommitments(snapshot, replayed);
const comparison = Object.fromEntries(ROOT_KEYS.map((key) => [key, { live: live[key], replayed: replayed.commitments[key], match: live[key] === replayed.commitments[key] }]));
const ok = ROOT_KEYS.every((key) => comparison[key].match);
const app = new ArenaVanguard({ statePath, journalPath });
const appVerification = app.verify();
assert.equal(appVerification.ok, true);
assert.equal(ok, true);
const replayVerification = { ok, statePath: relative(process.cwd(), statePath), journalPath: relative(process.cwd(), journalPath), live, replayed: replayed.commitments };
writeReport(statePath, journalPath, snapshot, journal, comparison, replayVerification);
console.log('LIVE HOTPOCKET PERSISTENCE: PASS');
console.log(JSON.stringify(replayVerification, null, 2));
