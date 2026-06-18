const HotPocket = require('hotpocket-nodejs-contract');
const { ArenaVanguard } = require('./arena_vanguard');

async function contract(ctx) {
  const app = new ArenaVanguard();

  for (const user of ctx.users.list()) {
    for (const input of user.inputs) {
      try {
        const buf = await ctx.users.read(input);
        const message = JSON.parse(buf);
        const output = await app.handleInput(user.publicKey, message, {
          readonly: ctx.readonly,
          lclSeqNo: ctx.lclSeqNo,
          npl: ctx.npl
        });

        await user.send(JSON.stringify(output));
      } catch (error) {
        const output = {
          status: 'rejected',
          accepted: false,
          tick: Number(ctx.lclSeqNo),
          lclSeqNo: Number(ctx.lclSeqNo),
          error: error.message
        };
        await user.send(JSON.stringify(output));
      }
    }
  }
}

const hpc = new HotPocket.Contract();
hpc.init(contract);

module.exports = { contract };
