#!/usr/bin/env node
'use strict';

const fs = require('fs');
const path = require('path');
const adapter = require('../src');

function usage() {
  console.error(`Usage: everarcade-hotpocket-adapter <command> [json]
Commands:
  ping
  join-player <player-id>
  execute <json-input>
  replay-proof
  validate`);
}

function print(value) {
  process.stdout.write(`${JSON.stringify(value, null, 2)}\n`);
}

const command = process.argv[2];
try {
  if (command === 'ping') {
    print(adapter.executeHotPocketInput({ action: 'ping', nonce: 'proof-ping' }, { stateDir: process.env.EVERARCADE_HOTPOCKET_STATE_DIR }));
  } else if (command === 'join-player') {
    const player = process.argv[3] || 'player-1';
    print(adapter.executeHotPocketInput({ action: 'join_player', player_id: player, nonce: `join:${player}` }, { stateDir: process.env.EVERARCADE_HOTPOCKET_STATE_DIR }));
  } else if (command === 'execute') {
    print(adapter.executeHotPocketInput(JSON.parse(process.argv[3] || '{}'), { stateDir: process.env.EVERARCADE_HOTPOCKET_STATE_DIR }));
  } else if (command === 'replay-proof' || command === 'validate') {
    const sequence = [
      { action: 'ping', nonce: 'ping-1' },
      { action: 'join_player', player_id: 'player-1', nonce: 'join-1' },
      { action: 'join_player', player_id: 'player-2', nonce: 'join-2' }
    ];
    const a = adapter.runSequence(sequence);
    const b = adapter.runSequence(sequence);
    print({ status: JSON.stringify(a.roots) === JSON.stringify(b.roots) ? 'PASS' : 'FAIL', run_a: a.roots, run_b: b.roots });
  } else {
    usage();
    process.exit(2);
  }
} catch (error) {
  print({ status: 'FAIL', error: error.message });
  process.exit(1);
}
