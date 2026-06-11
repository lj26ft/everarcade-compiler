#!/usr/bin/env node
'use strict';

const payload = {
  schema: 'everarcade.hotpocket.deployment-variant-b.v0.1',
  status: 'ok',
  variant: 'B',
  launched: true,
  args: process.argv.slice(2)
};

process.stdout.write(`${JSON.stringify(payload)}\n`);
