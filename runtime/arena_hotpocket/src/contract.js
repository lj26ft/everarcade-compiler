const HotPocket = require('hotpocket-nodejs-contract');
const { ArenaVanguard } = require('./arena_vanguard');

async function contract(ctx) {
  console.log('[ARENA] contract cwd', process.cwd());
  console.log('[ARENA] contract round', String(ctx.lclSeqNo));
  const app = new ArenaVanguard();
  const users = ctx.users.list();
  const usersWithInputs = users.filter((user) => user.inputs && user.inputs.length > 0);
  console.log('[ARENA] users with inputs', usersWithInputs.length);

  for (const user of users) {
    console.log('[ARENA] user input count', user.inputs ? user.inputs.length : 0);
    for (const input of user.inputs) {
      try {
        console.log('[ARENA] reading input');
        const buf = await ctx.users.read(input);
        const raw = Buffer.isBuffer(buf) ? buf.toString('utf8') : String(buf);
        console.log('[ARENA] raw input', raw);
        const message = JSON.parse(raw);
        console.log('[ARENA] parsed input', JSON.stringify(message));
        console.log('[ARENA] calling ArenaVanguard.handleInput');
        const output = await app.handleInput(user.publicKey, message, {
          readonly: ctx.readonly,
          lclSeqNo: ctx.lclSeqNo,
          npl: ctx.npl
        });
        console.log('[ARENA] output', JSON.stringify(output));
        console.log('[ARENA] sending output');
        await user.send(JSON.stringify(output));
      } catch (error) {
        const output = {
          status: 'rejected',
          accepted: false,
          tick: Number(ctx.lclSeqNo),
          lclSeqNo: Number(ctx.lclSeqNo),
          error: error.message
        };
        console.log('[ARENA] output', JSON.stringify(output));
        console.log('[ARENA] sending output');
        await user.send(JSON.stringify(output));
      }
    }
  }
}

const hpc = new HotPocket.Contract();
hpc.init(contract);

module.exports = { contract };
