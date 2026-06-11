#!/usr/bin/env node
'use strict';

const payload = {
  schema: 'everarcade.hotpocket.deployment-variant-a.v0.1',
  status: 'ok',
  variant: 'A',
  launched: true
};

process.stdout.write(`${JSON.stringify(payload)}\n`);
