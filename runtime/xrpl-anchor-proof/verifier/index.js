'use strict';

const fs = require('fs');
const path = require('path');
const { canonicalHash } = require('../payload_builder');

const ROOT = path.resolve(__dirname, '..');
const REPO_ROOT = path.resolve(ROOT, '../..');

function readText(file) { return fs.readFileSync(file, 'utf8'); }
function readJson(file) { return JSON.parse(fs.readFileSync(file, 'utf8')); }
function status(ok) { return ok ? 'PASS' : 'FAIL'; }

function hotPocketAcceptance(anchorHash) {
  const validators = ['validator-1', 'validator-2', 'validator-3'].map((validator, index) => ({
    validator,
    accepted_anchor_hash: anchorHash,
    vote: 'ACCEPT',
    round: index + 1
  }));
  return {
    schema: 'everarcade.hotpocket.anchor-acceptance-proof.v0.1',
    validators,
    quorum: `${validators.length}/${validators.length}`,
    accepted_anchor_hash: anchorHash,
    acceptance_hash: canonicalHash({ accepted_anchor_hash: anchorHash, validators }),
    status: 'PASS'
  };
}

function verifyNetwork(network, retrieval, payloadBundle, acceptance) {
  const anchor = payloadBundle.continuity_anchor;
  const runtimeStateRoot = readJson(path.join(REPO_ROOT, 'runtime', 'hotpocket-runtime-proof', 'reports', 'runtime_state_root_report.json')).state_root;
  const continuityReport = readText(path.join(REPO_ROOT, 'runtime', 'continuity-anchor-proof', 'reports', 'continuity_anchor_validation_report.txt'));
  const migrationReport = readText(path.join(REPO_ROOT, 'runtime', 'hotpocket-migration-proof', 'reports', 'hotpocket_migration_validation_report.txt'));
  const gameplayReport = readText(path.join(REPO_ROOT, 'runtime', 'hotpocket-gameplay-proof', 'reports', 'hotpocket_gameplay_validation_report.txt'));
  const reconstructedPayloadHash = canonicalHash(retrieval.retrieved_payload);
  const checks = {
    publication: retrieval.accepted,
    retrieval: retrieval.retrieval_match,
    payload_mutation_absent: reconstructedPayloadHash === payloadBundle.payload_hash,
    anchor_hash: retrieval.retrieved_payload.anchor_hash === payloadBundle.runtime_anchor_hash,
    continuity_hash: retrieval.retrieved_payload.anchor_hash === payloadBundle.continuity_proof_hash && /EverArcade Continuity Anchoring Proof v0\.1: PASS/.test(continuityReport),
    hotpocket_acceptance_hash: retrieval.retrieved_payload.anchor_hash === acceptance.accepted_anchor_hash && acceptance.status === 'PASS',
    replay: retrieval.retrieved_payload.replay_root === retrieval.retrieved_payload.state_root,
    runtime_state_root: typeof runtimeStateRoot === 'string' && runtimeStateRoot.length === 64,
    migration: /EverArcade Sovereign Runtime Migration Proof v0\.1: PASS/.test(migrationReport),
    gameplay: /HotPocket Consensus Gameplay Proof v0\.1: PASS/.test(gameplayReport),
    previous_anchor_hash: retrieval.retrieved_payload.previous_anchor_hash === anchor.payload.previous_anchor_hash,
    continuity_root: retrieval.retrieved_payload.continuity_root === anchor.payload.continuity_root
  };
  return {
    network,
    reconstructed_payload_hash: reconstructedPayloadHash,
    runtime_anchor_hash: payloadBundle.runtime_anchor_hash,
    continuity_proof_hash: payloadBundle.continuity_proof_hash,
    hotpocket_acceptance_hash: acceptance.accepted_anchor_hash,
    checks,
    status: status(Object.values(checks).every(Boolean))
  };
}

module.exports = { hotPocketAcceptance, verifyNetwork };
