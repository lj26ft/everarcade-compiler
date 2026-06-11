'use strict';

const fs = require('fs');
const path = require('path');
const adapter = require('../../hotpocket-runtime-proof/adapter/runtime-adapter');

const ROOT = path.resolve(__dirname, '..');
const REPO_ROOT = path.resolve(ROOT, '../..');
const CONTINUITY_PROOF_DIR = path.join(REPO_ROOT, 'runtime', 'continuity-anchor-proof');
const PROTOCOL_VERSION = 'everarcade-xrpl-xahau-anchor-publication-proof-v0.1';

function canonicalize(value) { return adapter.canonicalize(value); }
function canonicalHash(value) { return adapter.canonicalHash(value); }
function readJson(file) { return JSON.parse(fs.readFileSync(file, 'utf8')); }

function latestContinuityAnchor() {
  const chain = readJson(path.join(CONTINUITY_PROOF_DIR, 'continuity', 'continuity-chain.json'));
  if (!Array.isArray(chain.anchors) || chain.anchors.length === 0) throw new Error('continuity chain has no anchors');
  const anchor = chain.anchors[chain.anchors.length - 1];
  if (!anchor || !anchor.payload || !anchor.anchor_hash) throw new Error('latest continuity anchor is incomplete');
  return { chain, anchor };
}

function restoreRootFromRuntimeReport() {
  const restoreReport = fs.readFileSync(path.join(REPO_ROOT, 'runtime', 'hotpocket-runtime-proof', 'reports', 'runtime_restore_report.txt'), 'utf8');
  const match = restoreReport.match(/^restored_root:\s*([a-f0-9]{64})$/m);
  if (!match) throw new Error('runtime restore proof did not expose restored_root');
  return match[1];
}

function buildAnchorPayload() {
  const { chain, anchor } = latestContinuityAnchor();
  const restoreRoot = restoreRootFromRuntimeReport();
  const payload = {
    protocol_version: PROTOCOL_VERSION,
    world_id: anchor.payload.world_id,
    state_root: anchor.payload.state_root,
    replay_root: anchor.payload.replay_root,
    restore_root: restoreRoot,
    continuity_root: anchor.payload.continuity_root,
    previous_anchor_hash: anchor.payload.previous_anchor_hash,
    anchor_hash: anchor.anchor_hash
  };
  const payloadHash = canonicalHash(payload);
  return {
    payload,
    payload_hash: payloadHash,
    canonical_payload: canonicalize(payload),
    continuity_anchor: anchor,
    continuity_chain_hash: canonicalHash(chain),
    runtime_anchor_hash: anchor.anchor_hash,
    continuity_proof_hash: anchor.anchor_hash
  };
}

module.exports = { PROTOCOL_VERSION, canonicalize, canonicalHash, buildAnchorPayload };
