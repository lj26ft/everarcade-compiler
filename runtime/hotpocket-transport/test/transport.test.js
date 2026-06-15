'use strict';

const assert = require('node:assert/strict');
const test = require('node:test');
const { awaitAcceptedSubmission, canonicalHash, createTransportSubmission, processAcceptedSubmission, PROTOCOL, RUNTIME_PROTOCOL, submissionToEnvelope } = require('../src');

const payload = { world_id: 'world-1', player_id: 'player-1', mutation_type: 'join_player', payload: { x: 1 } };

function acceptedSubmission(extra = {}) {
  return createTransportSubmission({ lease_id: 'lease-1', contract_id: 'contract-1', user_public_key: 'user-pub', nonce: 7, ledger_seq_no: 42, raw_payload: JSON.stringify(payload), ...extra });
}

test('accepted submission becomes MutationEnvelope', () => {
  const envelope = submissionToEnvelope(acceptedSubmission());
  assert.equal(envelope.world_id, 'world-1');
  assert.equal(envelope.player_id, 'player-1');
  assert.equal(envelope.mutation_type, 'join_player');
  assert.equal(envelope.source_transport, PROTOCOL);
});

test('rejected submission never reaches runtime', async () => {
  const status = await awaitAcceptedSubmission({ hash: 'abc', submissionStatus: Promise.resolve({ status: 'rejected', reason: 'max_ledger_expired', server: 's1' }) });
  assert.equal(status.accepted, false);
  assert.equal(status.quarantine, true);
});

test('empty HotPocket round produces no mutation', () => {
  const users = [];
  let state = { mutations: [], sequence: 0 };
  for (const user of users) for (const input of user.inputs) state = processAcceptedSubmission(input, state).state;
  assert.equal(state.sequence, 0);
  assert.deepEqual(state.mutations, []);
});

test('malformed payload is rejected deterministically', () => {
  const bad = acceptedSubmission({ raw_payload: JSON.stringify({ world_id: 'w' }) });
  assert.throws(() => submissionToEnvelope(bad), /payload missing player_id/);
});

test('same payload produces same input hash', () => {
  assert.equal(acceptedSubmission().input_hash, acceptedSubmission().input_hash);
});

test('same mutation produces same receipt root', () => {
  const a = processAcceptedSubmission(acceptedSubmission(), { mutations: [], sequence: 0 });
  const b = processAcceptedSubmission(acceptedSubmission(), { mutations: [], sequence: 0 });
  assert.equal(a.receipt.receipt_root, b.receipt.receipt_root);
});

test('receipt output is parseable by client', () => {
  const result = processAcceptedSubmission(acceptedSubmission(), { mutations: [], sequence: 0 });
  const parsed = JSON.parse(JSON.stringify({ protocol: RUNTIME_PROTOCOL, receipt: result.receipt }));
  assert.equal(parsed.receipt.protocol, RUNTIME_PROTOCOL);
  assert.equal(typeof parsed.receipt.state_root, 'string');
});

test('transport does not leak HotPocket objects into runtime', () => {
  const result = processAcceptedSubmission(acceptedSubmission(), { mutations: [], sequence: 0 });
  assert.equal(JSON.stringify(result).includes('ctx'), false);
  assert.equal(JSON.stringify(result).includes('users'), false);
});

test('ledger sequence is captured when available', () => {
  const result = processAcceptedSubmission(acceptedSubmission(), { mutations: [], sequence: 0 });
  assert.equal(result.receipt.ledger_seq_no, 42);
});

test('replay from canonical envelopes produces equivalent roots', () => {
  const sub = acceptedSubmission();
  const first = processAcceptedSubmission(sub, { mutations: [], sequence: 0 });
  const replaySub = createTransportSubmission({ lease_id: sub.lease_id, contract_id: sub.contract_id, user_public_key: sub.user_public_key, nonce: sub.nonce, ledger_seq_no: sub.ledger_seq_no, raw_payload: sub.raw_payload, input_hash: sub.input_hash });
  const second = processAcceptedSubmission(replaySub, { mutations: [], sequence: 0 });
  assert.equal(canonicalHash(first.state), canonicalHash(second.state));
  assert.equal(first.receipt.replay_root, second.receipt.replay_root);
});
