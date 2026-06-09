#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const sdkRoot = path.resolve(__dirname, '..');
const repoRoot = path.resolve(sdkRoot, '..');

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


function slugify(value) {
  return String(value ?? '')
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '') || 'everarcade-game';
}

function discoverRuntimeVersion(runtimeRoot = path.join(repoRoot, 'runtime', 'everarcade-runtime')) {
  const metadataPath = path.join(runtimeRoot, 'runtime.metadata.json');
  if (fs.existsSync(metadataPath)) {
    const metadata = JSON.parse(fs.readFileSync(metadataPath, 'utf8'));
    if (metadata.RUNTIME_VERSION) return metadata.RUNTIME_VERSION;
    if (metadata.runtime_version) return metadata.runtime_version;
  }

  const srcRoot = path.join(runtimeRoot, 'src');
  const candidates = [
    path.join(srcRoot, 'runtime', 'configuration.rs'),
    path.join(srcRoot, 'lib.rs'),
    path.join(srcRoot, 'main.rs')
  ];
  for (const candidate of candidates) {
    if (!fs.existsSync(candidate)) continue;
    const source = fs.readFileSync(candidate, 'utf8');
    const match = source.match(/RUNTIME_VERSION\s*:\s*&str\s*=\s*"([^"]+)"/);
    if (match) return match[1];
  }
  throw new Error(`Unable to discover RUNTIME_VERSION from ${srcRoot}`);
}

function runtimePackageDir(projectDir) {
  return path.join(projectDir, 'dist', 'runtime-package');
}

function runtimePackageMetadata(manifest) {
  const gameId = slugify(manifest.id ?? manifest.game_id ?? manifest.name);
  const gameVersion = String(manifest.version ?? manifest.package_version ?? '0.1.0');
  const worldId = slugify(manifest.world_id ?? `${gameId}-world`);
  return { gameId, gameVersion, worldId };
}

function validateRuntimeManifest(runtimeManifest, runtimeVersion) {
  if (!runtimeManifest.package_id || !runtimeManifest.package_id.trim()) {
    throw new Error('Runtime manifest package_id is empty');
  }
  if (!/^[a-f0-9]{64}$/.test(runtimeManifest.wasm_hash)) {
    throw new Error('Runtime manifest wasm_hash must be exactly 64 hex characters');
  }
  if (runtimeManifest.signature !== `sha256:${runtimeManifest.wasm_hash}`) {
    throw new Error('Runtime manifest signature must match sha256:<wasm_hash>');
  }
  if (runtimeManifest.runtime_compatibility !== runtimeVersion) {
    throw new Error(`Runtime manifest compatibility ${runtimeManifest.runtime_compatibility} does not match ${runtimeVersion}`);
  }
}

function orderedJson(data, keys) {
  const ordered = {};
  for (const key of keys) ordered[key] = data[key];
  return `${JSON.stringify(ordered, null, 2)}\n`;
}

function packageGame(projectDir) {
  const manifest = readManifest(projectDir);
  const dist = path.join(projectDir, 'dist');
  if (!fs.existsSync(path.join(dist, 'build.json'))) build(projectDir);

  const runtimeVersion = discoverRuntimeVersion();
  const { gameId, gameVersion, worldId } = runtimePackageMetadata(manifest);
  const outDir = runtimePackageDir(projectDir);
  fs.rmSync(outDir, { recursive: true, force: true });
  fs.mkdirSync(outDir, { recursive: true });

  const worldMetadata = {
    world_id: worldId,
    game_id: gameId,
    game_name: manifest.name,
    template: manifest.template,
    classification: 'deterministic-placeholder-wasm',
    package_classification: 'placeholder-runtime-package',
    created_by: 'everarcade-creator-sdk',
    runtime_package_version: '0.1',
    runtime_compatibility: runtimeVersion
  };

  const placeholderWasm = {
    classification: 'deterministic-placeholder-wasm',
    entry: manifest.entry ?? 'src/game.js',
    format: 'everarcade-runtime-placeholder-world-wasm',
    game_id: gameId,
    game_name: manifest.name,
    runtime_compatibility: runtimeVersion,
    template: manifest.template,
    world_id: worldId
  };
  const deterministicPayload = Buffer.from(orderedJson(placeholderWasm, [
    'classification',
    'entry',
    'format',
    'game_id',
    'game_name',
    'runtime_compatibility',
    'template',
    'world_id'
  ]));
  const wasmHash = crypto.createHash('sha256').update(deterministicPayload).digest('hex');
  const runtimeManifest = {
    package_id: gameId,
    package_version: gameVersion,
    runtime_compatibility: runtimeVersion,
    wasm_path: 'world.wasm',
    wasm_hash: wasmHash,
    signature: `sha256:${wasmHash}`,
    world_id: worldId
  };
  validateRuntimeManifest(runtimeManifest, runtimeVersion);

  fs.writeFileSync(path.join(outDir, 'world.wasm'), deterministicPayload);
  writeJson(path.join(outDir, 'manifest.json'), runtimeManifest);
  writeJson(path.join(outDir, 'world.json'), worldMetadata);
  console.log(`Runtime Package: PASS (${gameId})`);
  return { manifest: runtimeManifest, worldMetadata, packageDir: outDir };
}

function prepareRuntimeCargoWorkspace() {
  const workspaceRoot = path.join('/tmp', 'everarcade-runtime-launch-workspace');
  const runtimeSource = path.join(repoRoot, 'runtime', 'everarcade-runtime');
  const runtimeTarget = path.join(workspaceRoot, 'everarcade-runtime');
  fs.mkdirSync(workspaceRoot, { recursive: true });
  fs.rmSync(runtimeTarget, { recursive: true, force: true });
  fs.mkdirSync(runtimeTarget, { recursive: true });
  fs.copyFileSync(path.join(runtimeSource, 'Cargo.toml'), path.join(runtimeTarget, 'Cargo.toml'));
  fs.cpSync(path.join(runtimeSource, 'src'), path.join(runtimeTarget, 'src'), { recursive: true });
  fs.writeFileSync(path.join(workspaceRoot, 'Cargo.toml'), '[workspace]\nmembers = ["everarcade-runtime"]\nresolver = "2"\n');
  return workspaceRoot;
}

function launchLocal(projectDir) {
  const packaged = fs.existsSync(path.join(runtimePackageDir(projectDir), 'manifest.json'))
    ? { manifest: JSON.parse(fs.readFileSync(path.join(runtimePackageDir(projectDir), 'manifest.json'), 'utf8')), packageDir: runtimePackageDir(projectDir) }
    : packageGame(projectDir);
  const runtimeRoot = path.resolve(value('--runtime-root', path.join(projectDir, 'dist', 'runtime-root')));
  const cargoArgs = [
    'run', '-q', '-p', 'everarcade-runtime', '--bin', 'runtime', '--',
    'start', runtimeRoot, packaged.manifest.world_id, packaged.packageDir
  ];
  const startedAt = new Date().toISOString();
  const cargoWorkspace = prepareRuntimeCargoWorkspace();
  const result = spawnSync('cargo', cargoArgs, {
    cwd: cargoWorkspace,
    env: { ...process.env, CARGO_BUILD_JOBS: process.env.CARGO_BUILD_JOBS ?? '1' },
    encoding: 'utf8'
  });
  const report = {
    command: `cargo ${cargoArgs.join(' ')}`,
    project_dir: projectDir,
    runtime_root: runtimeRoot,
    runtime_package_dir: packaged.packageDir,
    runtime_source_dir: path.join(repoRoot, 'runtime', 'everarcade-runtime'),
    cargo_workspace: cargoWorkspace,
    world_id: packaged.manifest.world_id,
    package_id: packaged.manifest.package_id,
    status: result.status === 0 ? 'PASS' : 'FAIL',
    started_at: startedAt,
    completed_at: new Date().toISOString(),
    stdout: result.stdout ?? '',
    stderr: result.stderr ?? '',
    exit_code: result.status
  };
  writeJson(path.join(projectDir, 'dist', 'local-launch-report.json'), report);
  if (result.status !== 0) {
    throw new Error(`Runtime start failed with exit code ${result.status}: ${(result.stderr || result.stdout || '').trim()}`);
  }
  console.log(`Local Runtime Launch: PASS (${packaged.manifest.world_id})`);
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
  else if (command === 'package') packageGame(projectDir);
  else if (command === 'launch-local') launchLocal(projectDir);
  else if (command === 'deploy') deploy(projectDir);
  else if (command === 'publish') publish(projectDir);
  else {
    console.log('everarcade <new|build|test|package|launch-local|deploy|publish> [--project DIR]');
    process.exit(command ? 1 : 0);
  }
} catch (error) {
  console.error(`Creator CLI Error: ${error.message}`);
  process.exit(1);
}
