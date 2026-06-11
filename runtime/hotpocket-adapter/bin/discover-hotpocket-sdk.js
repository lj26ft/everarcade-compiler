#!/usr/bin/env node
'use strict';

const fs = require('fs');
const path = require('path');

function safeRequire(name) {
  try {
    const resolved = require.resolve(name, { paths: [process.cwd(), __dirname] });
    return { name, resolved, module: require(resolved), packageJson: readPackageJson(resolved) };
  } catch (error) {
    return { name, available: false, error: error.message };
  }
}

function readPackageJson(resolved) {
  let dir = path.dirname(resolved);
  while (dir !== path.dirname(dir)) {
    const pkg = path.join(dir, 'package.json');
    if (fs.existsSync(pkg)) return JSON.parse(fs.readFileSync(pkg, 'utf8'));
    dir = path.dirname(dir);
  }
  return null;
}

function surface(value, depth = 0) {
  if (!value || depth > 1) return [];
  const keys = new Set([...Object.keys(value)]);
  if (typeof value === 'function' && value.prototype) Object.getOwnPropertyNames(value.prototype).forEach((key) => keys.add(`prototype.${key}`));
  return [...keys].sort().map((key) => ({ key, type: typeof key.split('.').reduce((acc, part) => (part === 'prototype' ? acc.prototype : acc && acc[part]), value) }));
}

const candidates = [
  'hotpocket-js-client',
  'hotpocket-nodejs-contract',
  'hotpocket-contract-js',
  'hotpocket'
];
const packages = candidates.map((name) => {
  const found = safeRequire(name);
  if (!found.module) return found;
  return {
    name,
    available: true,
    version: found.packageJson && found.packageJson.version,
    resolved: found.resolved,
    exports: surface(found.module),
    package: found.packageJson ? { name: found.packageJson.name, version: found.packageJson.version, main: found.packageJson.main } : null
  };
});

const report = {
  schema: 'everarcade.hotpocket.sdk-discovery.v0.1',
  generated_at: new Date(0).toISOString(),
  node: process.version,
  platform: process.platform,
  arch: process.arch,
  cwd: process.cwd(),
  packages,
  callback_signature_hypotheses: [
    'contract.init(ctx) / contract.execute(ctx) lifecycle style',
    'server.on(\'input\', (ctx, input) => ...) event style',
    'HotPocketContract(...) callback registration style'
  ],
  client_event_surface_hypotheses: ['connect', 'disconnect', 'ledger', 'contractOutput', 'submissionStatus'],
  consensus_configuration_probe: {
    env: Object.fromEntries(Object.keys(process.env).filter((key) => key.startsWith('HP_') || key.startsWith('HOTPOCKET_')).sort().map((key) => [key, process.env[key]])),
    required_local_nodes: 3
  },
  runtime_environment_metadata: {
    deterministic_clock: '1970-01-01T00:00:00.000Z',
    adapter_state_dir: process.env.EVERARCADE_HOTPOCKET_STATE_DIR || 'runtime/hotpocket-adapter/.state'
  }
};

const out = path.resolve(process.argv[2] || 'reports/hotpocket_sdk_discovery_report.json');
fs.mkdirSync(path.dirname(out), { recursive: true });
fs.writeFileSync(out, `${JSON.stringify(report, null, 2)}\n`);
process.stdout.write(`${out}\n`);
