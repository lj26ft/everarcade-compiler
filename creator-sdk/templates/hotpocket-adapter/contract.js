'use strict';

const { handleHotPocketInput } = require('../../../runtime/hotpocket-adapter/contract/contract');

const input = JSON.parse(process.argv[2] || '{"action":"ping"}');
handleHotPocketInput(input).then((output) => {
  process.stdout.write(`${JSON.stringify(output)}\n`);
}).catch((error) => {
  process.stderr.write(`${error.stack || error.message}\n`);
  process.exit(1);
});
