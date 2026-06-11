#!/usr/bin/env node
'use strict';

const payload = {
  schema: 'everarcade.hotpocket.creator-sdk-package.v0.1',
  status: 'ok',
  variant: 'C',
  generated_by: 'EverArcade Creator SDK HotPocket template',
  launched: true
};

process.stdout.write(`${JSON.stringify(payload)}\n`);
