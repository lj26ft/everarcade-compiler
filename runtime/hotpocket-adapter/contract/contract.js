'use strict';

const { executeHotPocketInput } = require('../src');

async function handleHotPocketInput(rawInput, context = {}) {
  const result = executeHotPocketInput(rawInput, {
    stateDir: process.env.EVERARCADE_HOTPOCKET_STATE_DIR,
    metadata: {
      ledger_seq: context.ledger_seq || context.ledger || 0,
      user: context.user || context.user_id || 'hotpocket-user'
    }
  });
  return result.receipt.output;
}

module.exports = { handleHotPocketInput };

if (require.main === module) {
  const payload = process.argv[2] || '{"action":"ping"}';
  handleHotPocketInput(JSON.parse(payload)).then((output) => {
    process.stdout.write(`${JSON.stringify(output)}\n`);
  }).catch((error) => {
    process.stderr.write(`${error.stack || error.message}\n`);
    process.exit(1);
  });
}
