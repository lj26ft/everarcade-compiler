'use strict';

function status(ok) { return ok ? 'PASS' : 'FAIL'; }

function certify(xrplVerification, xahauVerification, failureGateReport) {
  const ok = xrplVerification.status === 'PASS' && xahauVerification.status === 'PASS' && failureGateReport.status === 'PASS';
  return [
    'XRPL / Xahau Anchor Publication Proof v0.1 Certification',
    `XRPL publication verification: ${xrplVerification.status}`,
    `Xahau publication verification: ${xahauVerification.status}`,
    `Failure gates: ${failureGateReport.status}`,
    `anchor hash mismatch: ${failureGateReport.gates.anchor_hash_mismatch ? 'FAIL' : 'PASS'}`,
    `payload mutation: ${failureGateReport.gates.payload_mutation ? 'FAIL' : 'PASS'}`,
    `retrieval mismatch: ${failureGateReport.gates.retrieval_mismatch ? 'FAIL' : 'PASS'}`,
    `publication failure: ${failureGateReport.gates.publication_failure ? 'FAIL' : 'PASS'}`,
    `replay mismatch: ${failureGateReport.gates.replay_mismatch ? 'FAIL' : 'PASS'}`,
    `continuity mismatch: ${failureGateReport.gates.continuity_mismatch ? 'FAIL' : 'PASS'}`,
    `XRPL / Xahau Anchor Publication Proof v0.1: ${status(ok)}`
  ].join('\n') + '\n';
}

module.exports = { certify };
