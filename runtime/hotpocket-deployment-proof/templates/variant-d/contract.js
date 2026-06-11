#!/usr/bin/env node
'use strict';

const payload = {
  schema: 'everarcade.hotpocket.adapter-package.v0.1',
  status: 'ok',
  variant: 'D',
  generated_by: 'EverArcade HotPocket adapter',
  launched: true
};

process.stdout.write(`${JSON.stringify(payload)}\n`);
