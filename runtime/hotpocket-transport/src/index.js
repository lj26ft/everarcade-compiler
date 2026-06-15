'use strict';

const crypto = require('crypto');

const PROTOCOL = 'everarcade.transport.hotpocket.v0.1';
const RUNTIME_PROTOCOL = 'everarcade.runtime.receipt.v0.1';

function sha256(bytes) {
  return crypto.createHash('sha256').update(bytes).digest('hex');
}

function canonicalize(value) {
  if (Buffer.isBuffer(value)) return JSON.stringify(value.toString('base64'));
  if (value === null || typeof value !== 'object') return JSON.stringify(value);
  if (Array.isArray(value)) return `[${value.map(canonicalize).join(',')}]`;
  return `{${Object.keys(value).sort().map((key) => `${JSON.stringify(key)}:${canonicalize(value[key])}`).join(',')}}`;
}

function canonicalHash(value) { return sha256(canonicalize(value)); }
function bytesFromPayload(payload) { return Buffer.isBuffer(payload) ? Buffer.from(payload) : Buffer.from(typeof payload === 'string' ? payload : canonicalize(payload)); }
function parsePayload(rawPayload) {
  const text = Buffer.isBuffer(rawPayload) ? rawPayload.toString('utf8') : String(rawPayload);
  const parsed = JSON.parse(text);
  if (!parsed || typeof parsed !== 'object' || Array.isArray(parsed)) throw new Error('payload must be a JSON object');
  for (const key of ['world_id', 'player_id', 'mutation_type']) {
    if (typeof parsed[key] !== 'string' || parsed[key].length === 0) throw new Error(`payload missing ${key}`);
  }
  return parsed;
}
function normalizeSubmissionStatus(status) {
  if (!status) return { accepted: false, status: 'missing', reason: 'submissionStatus was not provided' };
  const value = typeof status === 'string' ? { status } : status;
  const statusText = String(value.status || value.result || value.code || '').toLowerCase();
  const accepted = value.accepted === true || value.ok === true || ['accepted', 'success', 'validated', 'tesSUCCESS'.toLowerCase()].includes(statusText);
  return { accepted, status: value.status || value.result || value.code || (accepted ? 'accepted' : 'rejected'), reason: value.reason || value.error || null, server: value.server || value.node || null, ledger_seq_no: value.ledger_seq_no ?? value.ledgerSeqNo ?? value.ledger_index ?? value.ledgerIndex ?? null };
}

async function awaitAcceptedSubmission(submitResult) {
  const result = Array.isArray(submitResult) ? submitResult[0] : submitResult;
  if (!result) return { accepted: false, quarantine: true, reason: 'empty submit result' };
  const status = await Promise.resolve(result.submissionStatus);
  const normalized = normalizeSubmissionStatus(status);
  const hash = result.hash || result.input_hash || result.submission_hash || normalized.hash;
  return { ...normalized, hash, raw: result, quarantine: !normalized.accepted };
}

function createTransportSubmission({ lease_id, contract_id, user_public_key, nonce = 0, raw_payload, ledger_seq_no = null, input_hash = null }) {
  const payloadBytes = bytesFromPayload(raw_payload);
  return {
    transport: PROTOCOL,
    lease_id: String(lease_id || ''),
    contract_id: String(contract_id || ''),
    user_public_key: String(user_public_key || ''),
    input_hash: input_hash || canonicalHash({ payload: payloadBytes.toString('base64'), nonce: Number(nonce) || 0 }),
    nonce: Number(nonce) || 0,
    ledger_seq_no: ledger_seq_no == null ? null : Number(ledger_seq_no),
    raw_payload: payloadBytes
  };
}

function submissionToEnvelope(submission) {
  const payload = parsePayload(submission.raw_payload);
  return {
    world_id: payload.world_id,
    player_id: payload.player_id,
    mutation_type: payload.mutation_type,
    payload: bytesFromPayload(payload.payload ?? {}),
    submission_hash: submission.input_hash,
    source_transport: submission.transport
  };
}

function executeDeterministicMutation(state, envelope) {
  if (!envelope || envelope.source_transport !== PROTOCOL) throw new Error('runtime only accepts canonical HotPocket envelopes');
  const current = state || { mutations: [], sequence: 0 };
  const mutationRecord = { sequence: current.sequence + 1, envelope: { ...envelope, payload: envelope.payload.toString('base64') } };
  const mutations = current.mutations.concat([mutationRecord]);
  return { sequence: mutationRecord.sequence, mutations, last_mutation_hash: canonicalHash(mutationRecord) };
}

function createReceipt({ world_id, input_hash, state, ledger_seq_no = null, ok = true }) {
  const state_root = canonicalHash({ label: 'state', state });
  const replay_root = canonicalHash({ label: 'replay', mutations: state.mutations });
  const receipt_root = canonicalHash({ label: 'receipt', world_id, input_hash, state_root, replay_root, ok });
  const continuity_root = canonicalHash({ label: 'continuity', receipt_root, sequence: state.sequence });
  return { protocol: RUNTIME_PROTOCOL, world_id, input_hash, state_root, replay_root, receipt_root, continuity_root, ledger_seq_no, ok };
}

function processAcceptedSubmission(submission, state) {
  const envelope = submissionToEnvelope(submission);
  const nextState = executeDeterministicMutation(state, envelope);
  const receipt = createReceipt({ world_id: envelope.world_id, input_hash: envelope.submission_hash, state: nextState, ledger_seq_no: submission.ledger_seq_no, ok: true });
  return { envelope, state: nextState, receipt };
}

module.exports = { PROTOCOL, RUNTIME_PROTOCOL, awaitAcceptedSubmission, canonicalHash, canonicalize, createReceipt, createTransportSubmission, executeDeterministicMutation, normalizeSubmissionStatus, processAcceptedSubmission, sha256, submissionToEnvelope };
