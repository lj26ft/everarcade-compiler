#!/usr/bin/env node
'use strict';

const fs = require('fs');
const path = require('path');
const childProcess = require('child_process');
const { buildAnchorPayload } = require('../payload_builder');
const publisher = require('../publisher');
const retrieval = require('../retrieval');
const verifier = require('../verifier');
const certification = require('../certification');

const ROOT = path.resolve(__dirname, '..');
const REPO_ROOT = path.resolve(ROOT, '../..');
const REPORT_DIR = process.env.EVERARCADE_XRPL_ANCHOR_REPORT_DIR || path.join(ROOT, 'reports');
const ROOT_REPORT_DIR = path.join(REPO_ROOT, 'reports');

function ensureDir(dir) { fs.mkdirSync(dir, { recursive: true }); }
function status(ok) { return ok ? 'PASS' : 'FAIL'; }
function writeTextBoth(name, content) {
  ensureDir(REPORT_DIR); ensureDir(ROOT_REPORT_DIR);
  fs.writeFileSync(path.join(REPORT_DIR, name), content);
  fs.writeFileSync(path.join(ROOT_REPORT_DIR, name), content);
}
function writeJsonBoth(name, value) { writeTextBoth(name, `${JSON.stringify(value, null, 2)}\n`); }
function runNode(script) {
  const result = childProcess.spawnSync('node', [script, 'validate'], {
    cwd: REPO_ROOT,
    env: { ...process.env, CARGO_BUILD_JOBS: process.env.CARGO_BUILD_JOBS || '1' },
    encoding: 'utf8',
    stdio: ['ignore', 'pipe', 'pipe'],
    timeout: Number(process.env.EVERARCADE_XRPL_ANCHOR_DEPENDENCY_TIMEOUT_MS || 300000)
  });
  return { ok: result.status === 0 && !result.error && /PASS/.test(result.stdout), stdout: result.stdout || '', stderr: result.error ? `${result.stderr || ''}${result.error.message}` : result.stderr || '' };
}

function dependencyProofs() {
  const scripts = {
    gameplay: path.join(REPO_ROOT, 'runtime', 'hotpocket-gameplay-proof', 'validation', 'hotpocket-gameplay-proof.js'),
    runtime: path.join(REPO_ROOT, 'runtime', 'hotpocket-runtime-proof', 'validation', 'hotpocket-runtime-proof.js'),
    migration: path.join(REPO_ROOT, 'runtime', 'hotpocket-migration-proof', 'validation', 'hotpocket-migration-proof.js'),
    continuity: path.join(REPO_ROOT, 'runtime', 'continuity-anchor-proof', 'validation', 'continuity-anchor-proof.js')
  };
  const results = Object.fromEntries(Object.entries(scripts).map(([name, script]) => [name, runNode(script)]));
  writeJsonBoth('anchor_publication_dependency_report.json', {
    schema: 'everarcade.xrpl-xahau.anchor-publication.dependencies.v0.1',
    hotpocket_gameplay_proof: status(results.gameplay.ok),
    runtime_integration_proof: status(results.runtime.ok),
    migration_proof: status(results.migration.ok),
    continuity_anchor_proof: status(results.continuity.ok),
    status: status(Object.values(results).every((result) => result.ok))
  });
  return results;
}

function submissionReport(network, payloadBundle, ledgerEntry, verification) {
  return {
    schema: `everarcade.${network}.anchor-submission-report.v0.1`,
    network,
    payload_preparation: 'PASS',
    transaction_construction: 'PASS',
    submission: ledgerEntry.accepted ? 'PASS' : 'FAIL',
    acceptance: ledgerEntry.accepted && ledgerEntry.validated && ledgerEntry.engine_result === 'tesSUCCESS' ? 'PASS' : 'FAIL',
    retrieval: verification.checks.retrieval ? 'PASS' : 'FAIL',
    hash_verification: verification.status,
    payload: payloadBundle.payload,
    payload_hash: payloadBundle.payload_hash,
    transaction_hash: ledgerEntry.transaction_hash,
    engine_result: ledgerEntry.engine_result,
    ledger_index: ledgerEntry.ledger_index,
    status: verification.status
  };
}

function failureGates(xrpl, xahau) {
  const both = [xrpl, xahau];
  const gates = {
    anchor_hash_mismatch: both.some((item) => !item.checks.anchor_hash || !item.checks.hotpocket_acceptance_hash),
    payload_mutation: both.some((item) => !item.checks.payload_mutation_absent),
    retrieval_mismatch: both.some((item) => !item.checks.retrieval),
    publication_failure: both.some((item) => !item.checks.publication),
    replay_mismatch: both.some((item) => !item.checks.replay),
    continuity_mismatch: both.some((item) => !item.checks.continuity_hash || !item.checks.continuity_root)
  };
  return { schema: 'everarcade.xrpl-xahau.anchor-publication.failure-gates.v0.1', gates, status: status(!Object.values(gates).some(Boolean)) };
}

function validate() {
  ensureDir(REPORT_DIR); ensureDir(ROOT_REPORT_DIR);
  const dependencies = dependencyProofs();
  const dependenciesOk = Object.values(dependencies).every((result) => result.ok);
  const payloadBundle = buildAnchorPayload();
  const acceptance = verifier.hotPocketAcceptance(payloadBundle.runtime_anchor_hash);
  writeJsonBoth('hotpocket_anchor_acceptance_report.json', acceptance);

  const xrplLedger = publisher.submit('xrpl', payloadBundle);
  const xahauLedger = publisher.submit('xahau', payloadBundle);
  const xrplRetrieval = retrieval.retrieve('xrpl');
  const xahauRetrieval = retrieval.retrieve('xahau');
  const xrplVerification = verifier.verifyNetwork('xrpl', xrplRetrieval, payloadBundle, acceptance);
  const xahauVerification = verifier.verifyNetwork('xahau', xahauRetrieval, payloadBundle, acceptance);
  const gates = failureGates(xrplVerification, xahauVerification);
  const overall = dependenciesOk && xrplVerification.status === 'PASS' && xahauVerification.status === 'PASS' && gates.status === 'PASS';

  writeJsonBoth('xrpl_anchor_submission_report.json', submissionReport('xrpl', payloadBundle, xrplLedger, xrplVerification));
  writeJsonBoth('xahau_anchor_submission_report.json', submissionReport('xahau', payloadBundle, xahauLedger, xahauVerification));
  writeJsonBoth('anchor_retrieval_report.json', {
    schema: 'everarcade.xrpl-xahau.anchor-retrieval-report.v0.1',
    xrpl: xrplRetrieval,
    xahau: xahauRetrieval,
    status: status(xrplRetrieval.retrieval_match && xahauRetrieval.retrieval_match)
  });
  writeJsonBoth('anchor_hash_verification_report.json', {
    schema: 'everarcade.xrpl-xahau.anchor-hash-verification-report.v0.1',
    runtime_anchor_hash: payloadBundle.runtime_anchor_hash,
    continuity_proof_hash: payloadBundle.continuity_proof_hash,
    hotpocket_acceptance_proof_hash: acceptance.accepted_anchor_hash,
    payload_hash: payloadBundle.payload_hash,
    xrpl: xrplVerification,
    xahau: xahauVerification,
    failure_gates: gates,
    status: status(overall)
  });
  writeTextBoth('anchor_publication_certification_report.txt', certification.certify(xrplVerification, xahauVerification, gates).replace(/XRPL \/ Xahau Anchor Publication Proof v0\.1: (PASS|FAIL)/, `XRPL / Xahau Anchor Publication Proof v0.1: ${status(overall)}`));
  process.stdout.write(`${status(overall)}\n`);
  return overall;
}

function main() {
  const command = process.argv[2] || 'validate';
  if (command !== 'validate') throw new Error(`unknown command: ${command}`);
  process.exit(validate() ? 0 : 1);
}

if (require.main === module) main();
module.exports = { validate };
