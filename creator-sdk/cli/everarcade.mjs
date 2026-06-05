#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const sdkRoot = path.resolve(__dirname, '..');

const command = process.argv[2];
const args = process.argv.slice(3);

function value(flag, fallback) {
  const index = args.indexOf(flag);
  return index >= 0 && args[index + 1] ? args[index + 1] : fallback;
}

function readManifest(projectDir) {
  const manifestPath = path.join(projectDir, 'everarcade.game.json');
  if (!fs.existsSync(manifestPath)) throw new Error(`Missing ${manifestPath}`);
  return JSON.parse(fs.readFileSync(manifestPath, 'utf8'));
}

function copyDir(source, target) {
  fs.mkdirSync(target, { recursive: true });
  for (const entry of fs.readdirSync(source, { withFileTypes: true })) {
    const from = path.join(source, entry.name);
    const to = path.join(target, entry.name);
    if (entry.isDirectory()) copyDir(from, to);
    else fs.copyFileSync(from, to);
  }
}

function writeJson(file, data) {
  fs.mkdirSync(path.dirname(file), { recursive: true });
  fs.writeFileSync(file, `${JSON.stringify(data, null, 2)}\n`);
}

function build(projectDir) {
  const manifest = readManifest(projectDir);
  const dist = path.join(projectDir, 'dist');
  fs.rmSync(dist, { recursive: true, force: true });
  fs.mkdirSync(dist, { recursive: true });
  writeJson(path.join(dist, 'build.json'), {
    type: 'Build',
    project: manifest.name,
    template: manifest.template,
    runtime: manifest.runtime,
    authoritative: false,
    generatedAt: new Date().toISOString()
  });
  writeJson(path.join(dist, 'package.json'), {
    type: 'Package',
    project: manifest.name,
    assets: manifest.assets ?? [],
    entry: manifest.entry ?? 'src/game.js'
  });
  console.log(`Build: PASS (${manifest.name})`);
}

function test(projectDir) {
  const manifest = readManifest(projectDir);
  const required = ['name', 'template', 'entry', 'runtime'];
  const missing = required.filter((key) => !manifest[key]);
  if (missing.length) throw new Error(`Project missing fields: ${missing.join(', ')}`);
  if (!fs.existsSync(path.join(projectDir, manifest.entry))) throw new Error(`Missing entry ${manifest.entry}`);
  console.log(`Test: PASS (${manifest.name})`);
}

function deploy(projectDir) {
  const manifest = readManifest(projectDir);
  const dist = path.join(projectDir, 'dist');
  if (!fs.existsSync(path.join(dist, 'build.json'))) build(projectDir);
  writeJson(path.join(dist, 'deployment.json'), {
    type: 'Deployment',
    project: manifest.name,
    target: value('--target', 'local'),
    authority: 'protocol-interface',
    settlement: 'testnet-simulated',
    replay: 'enabled'
  });
  console.log(`Deploy: PASS (${manifest.name})`);
}

function publish(projectDir) {
  const manifest = readManifest(projectDir);
  const dist = path.join(projectDir, 'dist');
  if (!fs.existsSync(path.join(dist, 'deployment.json'))) deploy(projectDir);
  writeJson(path.join(dist, 'publication.json'), {
    type: 'Publication',
    project: manifest.name,
    channel: value('--channel', 'creator-testnet'),
    monetization: manifest.monetization ?? 'demo-only',
    productionBilling: false,
    productionPayments: false
  });
  console.log(`Publish: PASS (${manifest.name})`);
}

try {
  const projectDir = path.resolve(value('--project', process.cwd()));
  if (command === 'new') {
    const name = value('--name', args[0] ?? 'everarcade-game');
    const template = value('--template', 'blank-game');
    const target = path.resolve(value('--dir', name));
    const templateDir = path.join(sdkRoot, 'templates', template);
    if (!fs.existsSync(templateDir)) throw new Error(`Unknown template ${template}`);
    copyDir(templateDir, target);
    const manifest = readManifest(target);
    manifest.name = name;
    writeJson(path.join(target, 'everarcade.game.json'), manifest);
    console.log(`Project: PASS (${name})`);
  } else if (command === 'build') build(projectDir);
  else if (command === 'test') test(projectDir);
  else if (command === 'deploy') deploy(projectDir);
  else if (command === 'publish') publish(projectDir);
  else {
    console.log('everarcade <new|build|test|deploy|publish> [--project DIR]');
    process.exit(command ? 1 : 0);
  }
} catch (error) {
  console.error(`Creator CLI Error: ${error.message}`);
  process.exit(1);
}
