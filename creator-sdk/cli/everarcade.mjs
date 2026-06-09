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

function guestContractDir(projectDir, manifest) {
  const configured = manifest.guest_contract ?? manifest.contract;
  if (configured) return path.resolve(projectDir, configured) === path.resolve(configured)
    ? path.resolve(configured)
    : path.resolve(repoRoot, configured);
  if (fs.existsSync(path.join(projectDir, 'Cargo.toml'))) return projectDir;
  return path.join(repoRoot, 'contracts', 'arena-proof-contract');
}

function ensureWasmTarget() {
  const installed = spawnSync('rustup', ['target', 'list', '--installed'], { encoding: 'utf8' });
  if (installed.status === 0 && installed.stdout.includes('wasm32-unknown-unknown')) return;
  const added = spawnSync('rustup', ['target', 'add', 'wasm32-unknown-unknown'], { encoding: 'utf8' });
  if (added.status !== 0) {
    throw new Error(`Unable to install wasm32-unknown-unknown target: ${(added.stderr || added.stdout || '').trim()}`);
  }
}

function buildGuestWasm(projectDir, manifest) {
  const contractDir = guestContractDir(projectDir, manifest);
  const cargoToml = path.join(contractDir, 'Cargo.toml');
  if (!fs.existsSync(cargoToml)) throw new Error(`Missing guest Cargo.toml at ${cargoToml}`);
  const cargo = fs.readFileSync(cargoToml, 'utf8');
  const name = (cargo.match(/name\s*=\s*"([^"]+)"/) || [null, path.basename(contractDir)])[1];
  const buildRoot = path.join('/tmp', 'everarcade-guest-build', name);
  fs.rmSync(buildRoot, { recursive: true, force: true });
  fs.mkdirSync(buildRoot, { recursive: true });
  fs.copyFileSync(cargoToml, path.join(buildRoot, 'Cargo.toml'));
  fs.cpSync(path.join(contractDir, 'src'), path.join(buildRoot, 'src'), { recursive: true });
  ensureWasmTarget();
  const result = spawnSync('cargo', [
    'build', '--manifest-path', path.join(buildRoot, 'Cargo.toml'), '--target', 'wasm32-unknown-unknown', '--release'
  ], {
    cwd: buildRoot,
    env: { ...process.env, CARGO_BUILD_JOBS: process.env.CARGO_BUILD_JOBS ?? '1' },
    encoding: 'utf8'
  });
  if (result.status !== 0) {
    throw new Error(`Guest build failed: ${(result.stderr || result.stdout || '').trim()}`);
  }
  const wasmName = `${name.replaceAll('-', '_')}.wasm`;
  const wasmPath = path.join(buildRoot, 'target', 'wasm32-unknown-unknown', 'release', wasmName);
  if (!fs.existsSync(wasmPath)) throw new Error(`Guest build did not produce ${wasmPath}`);
  return { contractDir, wasmPath, crateName: name };
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

  let wasmBytes;
  let worldMetadata;
  const wantsGuest = manifest.guest_contract || fs.existsSync(path.join(projectDir, 'Cargo.toml')) || gameId === 'arena-proof-contract';
  if (wantsGuest) {
    const guest = buildGuestWasm(projectDir, manifest);
    wasmBytes = fs.readFileSync(guest.wasmPath);
    if (!(wasmBytes[0] === 0x00 && wasmBytes[1] === 0x61 && wasmBytes[2] === 0x73 && wasmBytes[3] === 0x6d)) {
      throw new Error('Built guest artifact is not a WebAssembly module');
    }
    worldMetadata = {
      world_id: worldId,
      game_id: gameId,
      game_name: manifest.name,
      template: manifest.template,
      classification: 'wasm-guest-execution-proven-candidate',
      package_classification: 'wasm-guest-runtime-package',
      guest_contract: path.relative(repoRoot, guest.contractDir),
      guest_crate: guest.crateName,
      guest_entrypoint: 'everarcade_guest_execute',
      created_by: 'everarcade-creator-sdk',
      runtime_package_version: '0.1',
      runtime_compatibility: runtimeVersion
    };
  } else {
    worldMetadata = {
      world_id: worldId,
      game_id: gameId,
      game_name: manifest.name,
      template: manifest.template,
      classification: 'deterministic-placeholder-wasm',
      package_classification: manifest.template === 'arena' ? 'official-template-runtime-package' : 'placeholder-runtime-package',
      created_by: 'everarcade-creator-sdk',
      runtime_package_version: '0.1',
      runtime_compatibility: runtimeVersion
    };
    const placeholderWasm = {
      classification: manifest.template === 'arena' ? 'arena-template-gameplay-model' : 'deterministic-placeholder-wasm',
      entry: manifest.entry ?? 'src/game.js',
      format: 'everarcade-runtime-placeholder-world-wasm',
      game_id: gameId,
      game_name: manifest.name,
      runtime_compatibility: runtimeVersion,
      template: manifest.template,
      world_id: worldId
    };
    wasmBytes = Buffer.from(orderedJson(placeholderWasm, [
      'classification', 'entry', 'format', 'game_id', 'game_name', 'runtime_compatibility', 'template', 'world_id'
    ]));
  }
  const wasmHash = crypto.createHash('sha256').update(wasmBytes).digest('hex');
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

  fs.writeFileSync(path.join(outDir, 'world.wasm'), wasmBytes);
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


function runRuntimeCommand(commandName, projectDir, runtimeRoot, packaged) {
  const cargoArgs = [
    'run', '-q', '-p', 'everarcade-runtime', '--bin', 'runtime', '--',
    commandName, runtimeRoot, packaged.manifest.world_id, packaged.packageDir
  ];
  const cargoWorkspace = prepareRuntimeCargoWorkspace();
  const result = spawnSync('cargo', cargoArgs, {
    cwd: cargoWorkspace,
    env: { ...process.env, CARGO_BUILD_JOBS: process.env.CARGO_BUILD_JOBS ?? '1' },
    encoding: 'utf8'
  });
  return { cargoArgs, cargoWorkspace, result };
}

function ensurePackaged(projectDir) {
  return fs.existsSync(path.join(runtimePackageDir(projectDir), 'manifest.json'))
    ? { manifest: JSON.parse(fs.readFileSync(path.join(runtimePackageDir(projectDir), 'manifest.json'), 'utf8')), packageDir: runtimePackageDir(projectDir) }
    : packageGame(projectDir);
}

function launchLocal(projectDir) {
  const packaged = ensurePackaged(projectDir);
  const runtimeRoot = path.resolve(value('--runtime-root', path.join(projectDir, 'dist', 'runtime-root')));
  const startedAt = new Date().toISOString();
  const { cargoArgs, cargoWorkspace, result } = runRuntimeCommand('start', projectDir, runtimeRoot, packaged);
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

function executeLocal(projectDir) {
  const packaged = ensurePackaged(projectDir);
  const runtimeRoot = path.resolve(value('--runtime-root', path.join(projectDir, 'dist', 'runtime-root')));
  const startedAt = new Date().toISOString();
  const launch = runRuntimeCommand('start', projectDir, runtimeRoot, packaged);
  if (launch.result.status !== 0) {
    throw new Error(`Runtime start failed with exit code ${launch.result.status}: ${(launch.result.stderr || launch.result.stdout || '').trim()}`);
  }
  const proof = runRuntimeCommand('execute-proof', projectDir, runtimeRoot, packaged);
  const proofReport = {
    command: `cargo ${proof.cargoArgs.join(' ')}`,
    launch_command: `cargo ${launch.cargoArgs.join(' ')}`,
    project_dir: projectDir,
    runtime_root: runtimeRoot,
    runtime_package_dir: packaged.packageDir,
    runtime_source_dir: path.join(repoRoot, 'runtime', 'everarcade-runtime'),
    cargo_workspace: proof.cargoWorkspace,
    world_id: packaged.manifest.world_id,
    package_id: packaged.manifest.package_id,
    status: proof.result.status === 0 ? 'PASS' : 'FAIL',
    started_at: startedAt,
    completed_at: new Date().toISOString(),
    stdout: proof.result.stdout ?? '',
    stderr: proof.result.stderr ?? '',
    exit_code: proof.result.status
  };
  writeJson(path.join(projectDir, 'dist', 'deterministic-execution-report.json'), proofReport);
  if (proof.result.status !== 0) {
    throw new Error(`Runtime execute-proof failed with exit code ${proof.result.status}: ${(proof.result.stderr || proof.result.stdout || '').trim()}`);
  }
  const parsedProof = JSON.parse(proof.result.stdout);
  if (parsedProof.replay_verification !== 'PASS' || parsedProof.status !== 'Deterministic Execution: PASS') {
    throw new Error(`Deterministic execution failed replay verification: ${proof.result.stdout}`);
  }
  console.log('Deterministic Execution: PASS');
}


function executeTemplate(projectDir) {
  const template = value('--template', null);
  if (template && template !== 'arena') throw new Error(`Unsupported template gameplay proof ${template}`);
  const packaged = ensurePackaged(projectDir);
  const runtimeRoot = path.resolve(value('--runtime-root', path.join(projectDir, 'dist', 'runtime-root')));
  const startedAt = new Date().toISOString();
  const launch = runRuntimeCommand('start', projectDir, runtimeRoot, packaged);
  if (launch.result.status !== 0) {
    throw new Error(`Runtime start failed with exit code ${launch.result.status}: ${(launch.result.stderr || launch.result.stdout || '').trim()}`);
  }
  const proof = runRuntimeCommand('execute-template-proof', projectDir, runtimeRoot, packaged);
  const proofReport = {
    command: `cargo ${proof.cargoArgs.join(' ')}`,
    launch_command: `cargo ${launch.cargoArgs.join(' ')}`,
    project_dir: projectDir,
    runtime_root: runtimeRoot,
    runtime_package_dir: packaged.packageDir,
    runtime_source_dir: path.join(repoRoot, 'runtime', 'everarcade-runtime'),
    cargo_workspace: proof.cargoWorkspace,
    world_id: packaged.manifest.world_id,
    package_id: packaged.manifest.package_id,
    status: proof.result.status === 0 ? 'PASS' : 'FAIL',
    started_at: startedAt,
    completed_at: new Date().toISOString(),
    stdout: proof.result.stdout ?? '',
    stderr: proof.result.stderr ?? '',
    exit_code: proof.result.status
  };
  writeJson(path.join(projectDir, 'dist', 'template-gameplay-execution-report.json'), proofReport);
  if (proof.result.status !== 0) {
    throw new Error(`Runtime execute-template-proof failed with exit code ${proof.result.status}: ${(proof.result.stderr || proof.result.stdout || '').trim()}`);
  }
  const parsedProof = JSON.parse(proof.result.stdout);
  if (parsedProof.replay_verification !== 'PASS' || parsedProof.status !== 'Template Gameplay Execution: PASS') {
    throw new Error(`Template gameplay execution failed replay verification: ${proof.result.stdout}`);
  }
  console.log('Template Gameplay Execution: PASS');
}


function playLocal(projectDir) {
  const template = value('--template', null);
  if (template && template !== 'arena') throw new Error(`Unsupported local play template ${template}`);
  const packaged = ensurePackaged(projectDir);
  const runtimeRoot = path.resolve(value('--runtime-root', path.join(projectDir, 'dist', 'runtime-root')));
  const startedAt = new Date().toISOString();
  const proof = runRuntimeCommand('local-session', projectDir, runtimeRoot, packaged);
  const playReport = {
    command: `cargo ${proof.cargoArgs.join(' ')}`,
    project_dir: projectDir,
    runtime_root: runtimeRoot,
    runtime_package_dir: packaged.packageDir,
    runtime_source_dir: path.join(repoRoot, 'runtime', 'everarcade-runtime'),
    cargo_workspace: proof.cargoWorkspace,
    world_id: packaged.manifest.world_id,
    package_id: packaged.manifest.package_id,
    status: proof.result.status === 0 ? 'PASS' : 'FAIL',
    started_at: startedAt,
    completed_at: new Date().toISOString(),
    stdout: proof.result.stdout ?? '',
    stderr: proof.result.stderr ?? '',
    exit_code: proof.result.status
  };
  writeJson(path.join(projectDir, 'dist', 'playable-local-game-report.json'), playReport);
  if (proof.result.status !== 0) {
    throw new Error(`Runtime local-session failed with exit code ${proof.result.status}: ${(proof.result.stderr || proof.result.stdout || '').trim()}`);
  }
  const parsedProof = JSON.parse(proof.result.stdout);
  if (parsedProof.replay_verification !== 'PASS' || parsedProof.status !== 'Playable Local Game: PASS') {
    throw new Error(`Playable local game failed replay verification: ${proof.result.stdout}`);
  }
  console.log('Playable Local Game: PASS');
}

function playLocalMultiplayer(projectDir) {
  const template = value('--template', null);
  if (template && template !== 'arena') throw new Error(`Unsupported multiplayer local play template ${template}`);
  const packaged = packageGame(projectDir);
  const runtimeRoot = path.resolve(value('--runtime-root', path.join(projectDir, 'dist', 'runtime-root')));
  const startedAt = new Date().toISOString();
  const proof = runRuntimeCommand('multiplayer-local-session', projectDir, runtimeRoot, packaged);
  const playReport = {
    command: `cargo ${proof.cargoArgs.join(' ')}`,
    project_dir: projectDir,
    runtime_root: runtimeRoot,
    runtime_package_dir: packaged.packageDir,
    runtime_source_dir: path.join(repoRoot, 'runtime', 'everarcade-runtime'),
    cargo_workspace: proof.cargoWorkspace,
    world_id: packaged.manifest.world_id,
    package_id: packaged.manifest.package_id,
    status: proof.result.status === 0 ? 'PASS' : 'FAIL',
    started_at: startedAt,
    completed_at: new Date().toISOString(),
    stdout: proof.result.stdout ?? '',
    stderr: proof.result.stderr ?? '',
    exit_code: proof.result.status
  };
  writeJson(path.join(projectDir, 'dist', 'multiplayer-local-session-report.json'), playReport);
  if (proof.result.status !== 0) {
    throw new Error(`Runtime multiplayer-local-session failed with exit code ${proof.result.status}: ${(proof.result.stderr || proof.result.stdout || '').trim()}`);
  }
  const parsedProof = JSON.parse(proof.result.stdout);
  if (parsedProof.replay_verification !== 'PASS' || parsedProof.status !== 'Multiplayer Local Session: PASS') {
    throw new Error(`Multiplayer local session failed replay verification: ${proof.result.stdout}`);
  }
  console.log('Multiplayer Local Session: PASS');
}

function playNetworkLocal(projectDir) {
  const template = value('--template', null);
  if (template && template !== 'arena') throw new Error(`Unsupported network local play template ${template}`);
  const packaged = packageGame(projectDir);
  const runtimeRoot = path.resolve(value('--runtime-root', path.join(projectDir, 'dist', 'runtime-root')));
  const startedAt = new Date().toISOString();
  const proof = runRuntimeCommand('network-local-session', projectDir, runtimeRoot, packaged);
  const playReport = {
    command: `cargo ${proof.cargoArgs.join(' ')}`,
    project_dir: projectDir,
    runtime_root: runtimeRoot,
    runtime_package_dir: packaged.packageDir,
    runtime_source_dir: path.join(repoRoot, 'runtime', 'everarcade-runtime'),
    cargo_workspace: proof.cargoWorkspace,
    world_id: packaged.manifest.world_id,
    package_id: packaged.manifest.package_id,
    status: proof.result.status === 0 ? 'PASS' : 'FAIL',
    started_at: startedAt,
    completed_at: new Date().toISOString(),
    stdout: proof.result.stdout ?? '',
    stderr: proof.result.stderr ?? '',
    exit_code: proof.result.status
  };
  writeJson(path.join(projectDir, 'dist', 'network-transport-session-report.json'), playReport);
  if (proof.result.status !== 0) {
    throw new Error(`Runtime network-local-session failed with exit code ${proof.result.status}: ${(proof.result.stderr || proof.result.stdout || '').trim()}`);
  }
  const parsedProof = JSON.parse(proof.result.stdout);
  if (parsedProof.replay_verification !== 'PASS' || parsedProof.status !== 'Network Transport Session: PASS') {
    throw new Error(`Network transport session failed replay verification: ${proof.result.stdout}`);
  }
  console.log('Network Transport Session: PASS');
}


function playFederatedLocal(projectDir) {
  const template = value('--template', null);
  if (template && template !== 'arena') throw new Error(`Unsupported federated local play template ${template}`);
  const runtimeRoot = path.resolve(value('--runtime-root', path.join(projectDir, 'dist', 'federated-runtime-root')));
  const proofScript = path.join(repoRoot, 'runtime', 'federated-runtime-proof', 'federated_runtime_proof.mjs');
  const startedAt = new Date().toISOString();
  const result = spawnSync('node', [proofScript, '--project', projectDir, '--template', template ?? 'arena', '--runtime-root', runtimeRoot], {
    cwd: repoRoot,
    encoding: 'utf8'
  });
  const playReport = {
    command: `node ${proofScript} --project ${projectDir} --template ${template ?? 'arena'} --runtime-root ${runtimeRoot}`,
    project_dir: projectDir,
    runtime_root: runtimeRoot,
    proof_script: proofScript,
    template: template ?? 'arena',
    status: result.status === 0 ? 'PASS' : 'FAIL',
    started_at: startedAt,
    completed_at: new Date().toISOString(),
    stdout: result.stdout ?? '',
    stderr: result.stderr ?? '',
    exit_code: result.status
  };
  writeJson(path.join(projectDir, 'dist', 'federated-runtime-synchronization-report.json'), playReport);
  if (result.status !== 0) {
    throw new Error(`Federated local runtime proof failed with exit code ${result.status}: ${(result.stderr || result.stdout || '').trim()}`);
  }
  const summaryPath = path.join(runtimeRoot, 'federation', 'summary.json');
  const summary = JSON.parse(fs.readFileSync(summaryPath, 'utf8'));
  if (summary.status !== 'Federated Runtime Synchronization: PASS' || summary.replay_verification !== true) {
    throw new Error(`Federated local runtime proof failed verification: ${JSON.stringify(summary)}`);
  }
  process.stdout.write(result.stdout ?? '');
}


function playMultiLeaseLocal(projectDir) {
  const template = value('--template', null);
  if (template && template !== 'civilization') throw new Error(`Unsupported multi-lease local play template ${template}`);
  const runtimeRoot = path.resolve(value('--runtime-root', path.join(projectDir, 'dist', 'multi-lease-civilization-root')));
  const proofScript = path.join(repoRoot, 'runtime', 'multi-lease-civilization-proof', 'multi_lease_civilization_proof.mjs');
  const startedAt = new Date().toISOString();
  const result = spawnSync('node', [proofScript, '--project', projectDir, '--template', template ?? 'civilization', '--runtime-root', runtimeRoot], {
    cwd: repoRoot,
    encoding: 'utf8'
  });
  const playReport = {
    command: `node ${proofScript} --project ${projectDir} --template ${template ?? 'civilization'} --runtime-root ${runtimeRoot}`,
    project_dir: projectDir,
    runtime_root: runtimeRoot,
    proof_script: proofScript,
    template: template ?? 'civilization',
    status: result.status === 0 ? 'PASS' : 'FAIL',
    started_at: startedAt,
    completed_at: new Date().toISOString(),
    stdout: result.stdout ?? '',
    stderr: result.stderr ?? '',
    exit_code: result.status
  };
  writeJson(path.join(projectDir, 'dist', 'multi-lease-civilization-report.json'), playReport);
  if (result.status !== 0) {
    throw new Error(`Multi-lease civilization proof failed with exit code ${result.status}: ${(result.stderr || result.stdout || '').trim()}`);
  }
  const summaryPath = path.join(runtimeRoot, 'civilization', 'summary.json');
  const summary = JSON.parse(fs.readFileSync(summaryPath, 'utf8'));
  if (summary.status !== 'Multi-Lease Civilization Runtime: PASS' || summary.replay_verification !== true) {
    throw new Error(`Multi-lease civilization proof failed verification: ${JSON.stringify(summary)}`);
  }
  process.stdout.write(result.stdout ?? '');
}

function executeGuest(projectDir) {
  const packaged = packageGame(projectDir);
  const runtimeRoot = path.resolve(value('--runtime-root', path.join(projectDir, 'dist', 'runtime-root')));
  const startedAt = new Date().toISOString();
  const launch = runRuntimeCommand('start', projectDir, runtimeRoot, packaged);
  if (launch.result.status !== 0) {
    throw new Error(`Runtime start failed with exit code ${launch.result.status}: ${(launch.result.stderr || launch.result.stdout || '').trim()}`);
  }
  const proof = runRuntimeCommand('execute-guest-proof', projectDir, runtimeRoot, packaged);
  const proofReport = {
    command: `cargo ${proof.cargoArgs.join(' ')}`,
    launch_command: `cargo ${launch.cargoArgs.join(' ')}`,
    project_dir: projectDir,
    runtime_root: runtimeRoot,
    runtime_package_dir: packaged.packageDir,
    runtime_source_dir: path.join(repoRoot, 'runtime', 'everarcade-runtime'),
    cargo_workspace: proof.cargoWorkspace,
    world_id: packaged.manifest.world_id,
    package_id: packaged.manifest.package_id,
    status: proof.result.status === 0 ? 'PASS' : 'FAIL',
    started_at: startedAt,
    completed_at: new Date().toISOString(),
    stdout: proof.result.stdout ?? '',
    stderr: proof.result.stderr ?? '',
    exit_code: proof.result.status
  };
  writeJson(path.join(projectDir, 'dist', 'wasm-guest-execution-report.json'), proofReport);
  if (proof.result.status !== 0) {
    throw new Error(`Runtime execute-guest-proof failed with exit code ${proof.result.status}: ${(proof.result.stderr || proof.result.stdout || '').trim()}`);
  }
  const parsedProof = JSON.parse(proof.result.stdout);
  if (parsedProof.replay_verification !== 'PASS' || parsedProof.status !== 'WASM Guest Execution: PASS') {
    throw new Error(`WASM guest execution failed replay verification: ${proof.result.stdout}`);
  }
  console.log('WASM Guest Execution: PASS');
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
  else if (command === 'execute-local') executeLocal(projectDir);
  else if (command === 'execute-template') executeTemplate(projectDir);
  else if (command === 'execute-guest') executeGuest(projectDir);
  else if (command === 'play-local') playLocal(projectDir);
  else if (command === 'play-local-multiplayer') playLocalMultiplayer(projectDir);
  else if (command === 'play-network-local') playNetworkLocal(projectDir);
  else if (command === 'play-federated-local') playFederatedLocal(projectDir);
  else if (command === 'play-multi-lease-local') playMultiLeaseLocal(projectDir);
  else if (command === 'deploy') deploy(projectDir);
  else if (command === 'publish') publish(projectDir);
  else {
    console.log('everarcade <new|build|test|package|launch-local|execute-local|execute-template|execute-guest|play-local|play-local-multiplayer|play-network-local|play-federated-local|play-multi-lease-local|deploy|publish> [--project DIR]');
    process.exit(command ? 1 : 0);
  }
} catch (error) {
  console.error(`Creator CLI Error: ${error.message}`);
  process.exit(1);
}
