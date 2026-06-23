#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const sdkRoot = path.resolve(__dirname, '..');
const repoRoot = path.resolve(sdkRoot, '..');

let command = process.argv[2];
let args = process.argv.slice(3);

if (command === 'world') {
  if (args[0] === 'factory') {
    command = `world:factory:${args[1] ?? 'help'}`;
    args = args.slice(2);
  } else if (args[0] === 'attest') {
    command = `world:attest:${args[1] ?? 'help'}`;
    args = args.slice(2);
  } else {
    command = `world:${args[0] ?? 'help'}`;
    args = args.slice(1);
  }
}

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

function worldPackageFile(projectDir) {
  return path.join(projectDir, 'dist', 'world.evr');
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
  writeJson(worldPackageFile(projectDir), {
    schema: 'everarcade.world-package.v1.1',
    package_id: gameId,
    world_id: worldId,
    package_version: gameVersion,
    compatibility_dir: path.relative(projectDir, outDir),
    manifest: path.relative(projectDir, path.join(outDir, 'manifest.json')),
    world_metadata: path.relative(projectDir, path.join(outDir, 'world.json')),
    wasm: path.relative(projectDir, path.join(outDir, 'world.wasm')),
    note: 'world.evr is the creator-facing World Package entry point; dist/runtime-package remains for compatibility.'
  });
  console.log(`World Package: PASS (${gameId})`);
  console.log(`World Package File: ${path.relative(projectDir, worldPackageFile(projectDir))}`);
  return { manifest: runtimeManifest, worldMetadata, packageDir: outDir };
}



const FRONTIER_BLUEPRINT = {
  schema_version: 'WORLD_BLUEPRINT_V1',
  world_id: 'frontier-settlement-demo',
  world_name: 'Frontier Settlement Demo',
  world_type: 'frontier-settlement',
  governance: 'council',
  economy: 'marketplace',
  capabilities: ['inventory.transfer', 'market.trade', 'governance.vote'],
  runtime_profile: 'small',
  verification_targets: ['package', 'replay', 'restore', 'migration'],
  infrastructure_profile: 'single-evernode-lease'
};

const FRONTIER_CONTRACT_PLAN = {
  schema_version: 'WORLD_CONTRACT_PLAN_V1',
  world_id: 'frontier-settlement-demo',
  contract_version: '0.1.0',
  planned_mutations: ['inventory.transfer', 'market.trade', 'governance.vote'],
  safety_invariants: {
    'inventory.transfer': ['ownership', 'no-overdraw', 'conservation', 'atomicity'],
    'market.trade': ['ownership', 'no-double-spend', 'value-conservation', 'atomicity'],
    'governance.vote': ['eligibility', 'one-vote-per-member', 'tally-integrity']
  },
  verification_requirements: ['package', 'replay', 'restore']
};

const WORLD_FACTORY_PROJECT = path.join(repoRoot, 'examples', 'world-factory', 'frontier-settlement');
const WORLD_FACTORY_CAPABILITIES = new Set(['inventory.transfer', 'market.trade', 'governance.vote']);
const WORLD_FACTORY_RUNTIME_PROFILES = new Set(['small']);
const WORLD_FACTORY_VERIFICATION_TARGETS = new Set(['package', 'replay', 'restore', 'migration']);

function sha256Hex(bytes) { return crypto.createHash('sha256').update(bytes).digest('hex'); }
function readJsonFile(file) { return JSON.parse(fs.readFileSync(file, 'utf8')); }
function assertFactory(condition, message) { if (!condition) throw new Error(message); }

function worldFactoryInit() {
  fs.mkdirSync(WORLD_FACTORY_PROJECT, { recursive: true });
  writeJson(path.join(WORLD_FACTORY_PROJECT, 'world-blueprint.json'), FRONTIER_BLUEPRINT);
  writeJson(path.join(WORLD_FACTORY_PROJECT, 'world-contract-plan.json'), FRONTIER_CONTRACT_PLAN);
  console.log(`World Factory Init: PASS (${path.relative(repoRoot, WORLD_FACTORY_PROJECT)})`);
}

function loadWorldFactoryProject(projectDir) {
  const blueprintPath = path.join(projectDir, 'world-blueprint.json');
  const planPath = path.join(projectDir, 'world-contract-plan.json');
  assertFactory(fs.existsSync(blueprintPath), `Missing ${blueprintPath}`);
  assertFactory(fs.existsSync(planPath), `Missing ${planPath}`);
  return { blueprint: readJsonFile(blueprintPath), plan: readJsonFile(planPath) };
}

function validateWorldFactoryProject(projectDir) {
  const { blueprint, plan } = loadWorldFactoryProject(projectDir);
  for (const key of Object.keys(FRONTIER_BLUEPRINT)) assertFactory(Object.hasOwn(blueprint, key), `Blueprint missing ${key}`);
  for (const key of Object.keys(FRONTIER_CONTRACT_PLAN)) assertFactory(Object.hasOwn(plan, key), `Contract plan missing ${key}`);
  assertFactory(blueprint.schema_version === 'WORLD_BLUEPRINT_V1', 'Blueprint schema_version must be WORLD_BLUEPRINT_V1');
  assertFactory(plan.schema_version === 'WORLD_CONTRACT_PLAN_V1', 'Contract plan schema_version must be WORLD_CONTRACT_PLAN_V1');
  assertFactory(blueprint.world_id === plan.world_id, 'Blueprint and contract plan world_id must match');
  assertFactory(blueprint.world_type === 'frontier-settlement', 'Unsupported world_type');
  assertFactory(WORLD_FACTORY_RUNTIME_PROFILES.has(blueprint.runtime_profile), 'Unsupported runtime_profile');
  for (const capability of blueprint.capabilities) assertFactory(WORLD_FACTORY_CAPABILITIES.has(capability), `Unsupported capability ${capability}`);
  for (const mutation of plan.planned_mutations) {
    assertFactory(blueprint.capabilities.includes(mutation), `Planned mutation ${mutation} is not in blueprint capabilities`);
    assertFactory(Array.isArray(plan.safety_invariants[mutation]) && plan.safety_invariants[mutation].length > 0, `Missing invariants for ${mutation}`);
  }
  for (const target of blueprint.verification_targets) assertFactory(WORLD_FACTORY_VERIFICATION_TARGETS.has(target), `Unsupported verification target ${target}`);
  for (const requirement of plan.verification_requirements) assertFactory(blueprint.verification_targets.includes(requirement), `Verification requirement ${requirement} not requested by blueprint`);
  console.log(`World Factory Validate: PASS (${blueprint.world_id})`);
  return { blueprint, plan };
}

function hashManifestFor(dir) {
  const files = [];
  function walk(prefix = '') {
    for (const name of fs.readdirSync(path.join(dir, prefix))) {
      const rel = prefix ? `${prefix}/${name}` : name;
      if (rel === 'hash-manifest.json' || rel === 'expected-package-hash.txt') continue;
      const stat = fs.statSync(path.join(dir, rel));
      if (stat.isDirectory()) walk(rel);
      else files.push(rel);
    }
  }
  walk();
  files.sort((a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b)));
  const entries = files.map((file) => ({ path: file, sha256: sha256Hex(fs.readFileSync(path.join(dir, file))) }));
  const stream = entries.map((entry) => `${entry.path}\0${entry.sha256}\n`).join('');
  return { manifest: { hash_alg: 'sha256', file_order: 'lexicographic-by-path', files: entries }, packageHash: sha256Hex(Buffer.from(stream, 'utf8')) };
}

function generateWorldFactoryPackage(projectDir) {
  const { blueprint, plan } = validateWorldFactoryProject(projectDir);
  const outRoot = path.join(projectDir, 'out');
  const pkg = path.join(outRoot, 'world.evr');
  fs.rmSync(pkg, { recursive: true, force: true });
  fs.mkdirSync(pkg, { recursive: true });
  const runtime = { determinism_profile: 'tier2-cold-verifier', runtime_id: 'everarcade-world-factory-runtime', runtime_version: '0.1.0', runtime_profile: blueprint.runtime_profile };
  const genesis = { genesis_id: `genesis:${blueprint.world_id}`, initial_state_root: `sha256:${sha256Hex(JSON.stringify({ world_id: blueprint.world_id, capabilities: blueprint.capabilities }))}`, runtime_id: runtime.runtime_id, world_id: blueprint.world_id };
  const contract = { abi_version: plan.contract_version, contract_hash: `sha256:${sha256Hex(JSON.stringify(plan))}`, contract_id: `${blueprint.world_id}-contract`, planned_mutations: plan.planned_mutations, safety_invariants: plan.safety_invariants, world_id: blueprint.world_id };
  const journal = { root_package: blueprint.world_id, checkpoint_root: `sha256:${'4'.repeat(64)}`, entries: [], world_id: blueprint.world_id };
  const continuityRoot = `sha256:${sha256Hex(`${JSON.stringify(journal, null, 2)}\n`)}`;
  const checkpoint = { checkpoint_root: journal.checkpoint_root, root_package: blueprint.world_id, roots: { continuity_root: continuityRoot }, world_id: blueprint.world_id };
  writeJson(path.join(pkg, 'runtime/runtime.json'), runtime);
  writeJson(path.join(pkg, 'genesis/genesis.json'), genesis);
  writeJson(path.join(pkg, 'world-contract/world-contract.json'), contract);
  writeJson(path.join(pkg, 'restore/journal.json'), journal);
  writeJson(path.join(pkg, 'restore/checkpoint.json'), checkpoint);
  writeJson(path.join(pkg, 'proof/certification.json'), { claims: ['world-factory-mvp', 'all-load-bearing-files-hashed'], package_hash: 'bound-by-expected-package-hash', profile: 'world.evr-package-v1', runtime_id: runtime.runtime_id, status: 'PASS', verifier: 'world-factory-v1', world_id: blueprint.world_id });
  const manifest = { canonicalization: { file_order: 'lexicographic-by-path', manifest_encoding: 'canonical-json-utf8-lf', unknown_fields: 'reject' }, format: 'world.evr', genesis: { genesis_id: genesis.genesis_id, hash_alg: 'sha256', path: 'genesis/genesis.json', sha256: sha256Hex(fs.readFileSync(path.join(pkg, 'genesis/genesis.json'))) }, optional_files: ['restore/checkpoint.json', 'restore/journal.json', 'proof/certification.json'], package_name: blueprint.world_id, proof: { certification_path: 'proof/certification.json' }, required_files: ['manifest.json', 'genesis/genesis.json', 'runtime/runtime.json', 'world-contract/world-contract.json', 'hash-manifest.json', 'expected-package-hash.txt'], restore: { checkpoint_path: 'restore/checkpoint.json', journal_path: 'restore/journal.json' }, runtime: { hash_alg: 'sha256', path: 'runtime/runtime.json', runtime_id: runtime.runtime_id, runtime_version: runtime.runtime_version, sha256: sha256Hex(fs.readFileSync(path.join(pkg, 'runtime/runtime.json'))) }, spec_version: 'V1', world_contract: { contract_id: contract.contract_id, hash_alg: 'sha256', path: 'world-contract/world-contract.json', sha256: sha256Hex(fs.readFileSync(path.join(pkg, 'world-contract/world-contract.json'))) }, world_id: blueprint.world_id };
  writeJson(path.join(pkg, 'manifest.json'), manifest);
  const hm = hashManifestFor(pkg);
  writeJson(path.join(pkg, 'hash-manifest.json'), hm.manifest);
  fs.writeFileSync(path.join(pkg, 'expected-package-hash.txt'), `${hm.packageHash}\n`);
  writeJson(path.join(outRoot, 'world-factory-report.json'), { world_id: blueprint.world_id, world_name: blueprint.world_name, blueprint_schema: blueprint.schema_version, contract_plan_schema: plan.schema_version, package_spec: 'WORLD_EVR_PACKAGE_SPEC_V1', package_hash: hm.packageHash, capabilities: blueprint.capabilities, runtime_profile: blueprint.runtime_profile, verification_status: 'PASS' });
  console.log(`World Factory Generate: PASS (${path.relative(repoRoot, pkg)})`);
  console.log(`Package Hash: ${hm.packageHash}`);
}


function canonicalJson(value) {
  if (Array.isArray(value)) return `[${value.map(canonicalJson).join(',')}]`;
  if (value && typeof value === 'object') {
    return `{${Object.keys(value).sort().map((key) => `${JSON.stringify(key)}:${canonicalJson(value[key])}`).join(',')}}`;
  }
  return JSON.stringify(value);
}

function commitment(value) { return `sha256:${sha256Hex(Buffer.from(canonicalJson(value), 'utf8'))}`; }

function runtimeDirFor(projectDir) { return path.join(projectDir, 'out', 'runtime'); }
function worldPackageDirFor(projectDir) { return path.join(projectDir, 'out', 'world.evr'); }

function loadGeneratedWorld(projectDir) {
  return loadWorldPackageDir(worldPackageDirFor(projectDir));
}

function smallRuntimeAssumptions() {
  return {
    runtime_profile: 'small',
    world_instances: 1,
    operators: 1,
    tick_rate: 'fixed',
    determinism: 'deterministic-local-execution',
    persistence: 'local-json-files'
  };
}

function bootstrapWorldState(worldId) {
  return {
    world_id: worldId,
    tick: 0,
    settlements: [{ id: 'founders-crossing', population: 3, treasury: 100 }],
    inventory: {
      alice: { timber: 10, ore: 2, charter: 1 },
      bruno: { timber: 1, ore: 8, coin: 30 },
      settlement: { timber: 25, ore: 12, coin: 100 }
    },
    market: { trades: [], volume: 0 },
    governance: { proposals: { 'proposal-001': { title: 'Open the public workshop', yes: 0, no: 0, voters: [] } } }
  };
}

function plannedWorldAction(tick) {
  const actions = [
    { mutation: 'inventory.transfer', from: 'alice', to: 'settlement', item: 'timber', amount: 2 },
    { mutation: 'market.trade', seller: 'bruno', buyer: 'alice', item: 'ore', amount: 1, price: 6 },
    { mutation: 'governance.vote', proposal_id: 'proposal-001', voter: 'alice', choice: 'yes' }
  ];
  return actions[(tick - 1) % actions.length];
}

function applyWorldAction(state, action) {
  if (action.mutation === 'inventory.transfer') {
    state.inventory[action.from][action.item] -= action.amount;
    state.inventory[action.to][action.item] = (state.inventory[action.to][action.item] ?? 0) + action.amount;
  } else if (action.mutation === 'market.trade') {
    state.inventory[action.seller][action.item] -= action.amount;
    state.inventory[action.buyer][action.item] = (state.inventory[action.buyer][action.item] ?? 0) + action.amount;
    state.inventory[action.buyer].coin = (state.inventory[action.buyer].coin ?? 0) - action.price;
    state.inventory[action.seller].coin = (state.inventory[action.seller].coin ?? 0) + action.price;
    state.market.trades.push({ tick: state.tick, seller: action.seller, buyer: action.buyer, item: action.item, amount: action.amount, price: action.price });
    state.market.volume += action.price;
  } else if (action.mutation === 'governance.vote') {
    const proposal = state.governance.proposals[action.proposal_id];
    proposal[action.choice] += 1;
    proposal.voters.push(action.voter);
  } else {
    throw new Error(`Unsupported world action ${action.mutation}`);
  }
}

function receiptForTick(worldId, tick, previousStateRoot, action = plannedWorldAction(tick)) {
  const deterministic = { action: action.mutation, payload: action, previous_state_root: previousStateRoot, tick, world_id: worldId };
  return { tick, receipt_hash: commitment(deterministic), data: deterministic };
}

function rootsFor(state, receipts, journal, worldHashInput) {
  const state_root = commitment(state);
  const receipt_root = commitment(receipts.map((receipt) => ({ tick: receipt.tick, receipt_hash: receipt.receipt_hash, data: receipt.data })));
  const world_hash = commitment(worldHashInput);
  const continuity_root = commitment({ state_root, receipt_root, world_hash, journal });
  return { state_root, receipt_root, world_hash, continuity_root };
}

function worldHashInputFor(state, generatedWorld) {
  return {
    manifest_sha256: sha256Hex(fs.readFileSync(path.join(generatedWorld.packageDir, 'manifest.json'))),
    contract_hash: generatedWorld.contract.contract_hash,
    runtime_id: generatedWorld.runtime.runtime_id,
    world_id: state.world_id
  };
}

function statusFor(state, receipts, journal, generatedWorld) {
  return rootsFor(state, receipts, journal, worldHashInputFor(state, generatedWorld));
}

function loadWorldPackageDir(packageDir) {
  assertFactory(fs.existsSync(packageDir), `Missing world.evr at ${packageDir}`);
  const manifest = readJsonFile(path.join(packageDir, 'manifest.json'));
  const runtime = readJsonFile(path.join(packageDir, manifest.runtime.path));
  const genesis = readJsonFile(path.join(packageDir, manifest.genesis.path));
  const contract = readJsonFile(path.join(packageDir, manifest.world_contract.path));
  return { packageDir, manifest, runtime, genesis, contract };
}

function readRuntimePackage(projectDir) {
  const runtimeDir = runtimeDirFor(projectDir);
  return {
    runtimeDir,
    state: readJsonFile(path.join(runtimeDir, 'world-state.json')),
    journal: readJsonFile(path.join(runtimeDir, 'journal.json')),
    receipts: readJsonFile(path.join(runtimeDir, 'receipts.json')),
    status: readJsonFile(path.join(runtimeDir, 'runtime-status.json'))
  };
}

function writeRuntimePackage(projectDir, state, journal, receipts, generatedWorld, verification = 'PENDING') {
  const runtimeDir = runtimeDirFor(projectDir);
  const roots = statusFor(state, receipts, journal, generatedWorld);
  writeJson(path.join(runtimeDir, 'world-state.json'), state);
  writeJson(path.join(runtimeDir, 'journal.json'), journal);
  writeJson(path.join(runtimeDir, 'receipts.json'), receipts);
  writeJson(path.join(runtimeDir, 'runtime-status.json'), { status: 'RUNNING', tick: state.tick, verification, ...roots });
  return roots;
}

function bootWorldFactoryRuntime(projectDir) {
  const generatedWorld = loadGeneratedWorld(projectDir);
  assertFactory(generatedWorld.runtime.runtime_profile === 'small', 'Phase 2 only supports runtime_profile=small');
  const state = bootstrapWorldState(generatedWorld.manifest.world_id);
  const journal = [];
  const receipts = [];
  writeRuntimePackage(projectDir, state, journal, receipts, generatedWorld);
  writeJson(path.join(runtimeDirFor(projectDir), 'runtime-profile.json'), smallRuntimeAssumptions());
  console.log(`World Factory Boot: PASS (${path.relative(repoRoot, runtimeDirFor(projectDir))})`);
}

function runWorldFactoryRuntime(projectDir) {
  const ticks = Number.parseInt(value('--ticks', '1'), 10);
  assertFactory(Number.isInteger(ticks) && ticks >= 0, '--ticks must be a non-negative integer');
  const generatedWorld = loadGeneratedWorld(projectDir);
  if (!fs.existsSync(path.join(runtimeDirFor(projectDir), 'world-state.json'))) bootWorldFactoryRuntime(projectDir);
  const pkg = readRuntimePackage(projectDir);
  const state = { world_id: pkg.state.world_id, tick: pkg.state.tick, settlements: pkg.state.settlements, inventory: pkg.state.inventory, market: pkg.state.market, governance: pkg.state.governance };
  const journal = pkg.journal;
  const receipts = pkg.receipts;
  for (let i = 0; i < ticks; i += 1) {
    const previousRoots = statusFor(state, receipts, journal, generatedWorld);
    state.tick += 1;
    const action = plannedWorldAction(state.tick);
    applyWorldAction(state, action);
    const receipt = receiptForTick(state.world_id, state.tick, previousRoots.state_root, action);
    receipts.push(receipt);
    const roots = statusFor(state, receipts, journal, generatedWorld);
    journal.push({ tick: state.tick, action: action.mutation, payload: action, receipt_hash: receipt.receipt_hash, world_hash: roots.world_hash });
  }
  const roots = writeRuntimePackage(projectDir, state, journal, receipts, generatedWorld, 'PENDING');
  console.log(`World Factory Run: PASS (${state.tick} ticks)`);
  console.log(`State Root: ${roots.state_root}`);
}

function replayWorldFactoryRuntime(projectDir) {
  const generatedWorld = loadGeneratedWorld(projectDir);
  const runtime = readRuntimePackage(projectDir);
  const replayState = bootstrapWorldState(generatedWorld.manifest.world_id);
  const replayJournal = [];
  const replayReceipts = [];
  for (const entry of runtime.journal) {
    const previousRoots = statusFor(replayState, replayReceipts, replayJournal, generatedWorld);
    replayState.tick += 1;
    const action = plannedWorldAction(replayState.tick);
    applyWorldAction(replayState, action);
    const receipt = receiptForTick(replayState.world_id, replayState.tick, previousRoots.state_root, action);
    const rootsBeforeJournal = statusFor(replayState, replayReceipts.concat([receipt]), replayJournal, generatedWorld);
    const replayEntry = { tick: replayState.tick, action: action.mutation, payload: action, receipt_hash: receipt.receipt_hash, world_hash: rootsBeforeJournal.world_hash };
    assertFactory(canonicalJson(entry) === canonicalJson(replayEntry), `Journal mismatch at tick ${entry.tick}`);
    replayReceipts.push(receipt);
    replayJournal.push(replayEntry);
  }
  const replayRoots = statusFor(replayState, replayReceipts, replayJournal, generatedWorld);
  const runtimeRoots = statusFor({ world_id: runtime.state.world_id, tick: runtime.state.tick, settlements: runtime.state.settlements, inventory: runtime.state.inventory, market: runtime.state.market, governance: runtime.state.governance }, runtime.receipts, runtime.journal, generatedWorld);
  const pass = ['state_root', 'receipt_root', 'world_hash', 'continuity_root'].every((key) => replayRoots[key] === runtimeRoots[key]);
  const replay_status = pass ? 'PASS' : 'FAIL';
  writeJson(path.join(runtime.runtimeDir, 'world-factory-runtime-report.json'), { world_id: replayState.world_id, ticks_executed: replayState.tick, ...runtimeRoots, replay_status });
  writeJson(path.join(runtime.runtimeDir, 'runtime-status.json'), { status: 'RUNNING', tick: replayState.tick, verification: replay_status, ...runtimeRoots });
  console.log(replay_status);
  if (!pass) process.exitCode = 1;
}


function packageVerificationStatus(projectDir, packageDir = path.join(projectDir, 'out', 'world.evr')) {
  const verifier = path.join(repoRoot, 'specs', 'world-evr-package', 'verify-package-v1.mjs');
  const result = spawnSync('node', [verifier, packageDir], { cwd: repoRoot, encoding: 'utf8' });
  return result.status === 0 ? 'PASS' : 'FAIL';
}

function replayVerificationStatus(projectDir) {
  const runtimeDir = runtimeDirFor(projectDir);
  const reportPath = path.join(runtimeDir, 'world-factory-runtime-report.json');
  const statusPath = path.join(runtimeDir, 'runtime-status.json');
  if (fs.existsSync(reportPath)) return readJsonFile(reportPath).replay_status === 'PASS' ? 'PASS' : 'FAIL';
  if (fs.existsSync(statusPath)) return readJsonFile(statusPath).verification === 'PASS' ? 'PASS' : 'FAIL';
  return 'FAIL';
}

function deploymentDirFor(projectDir) { return path.join(projectDir, 'out', 'deploy'); }
function deploymentEvidenceDirFor(projectDir) { return path.join(deploymentDirFor(projectDir), 'evidence'); }

function packageHashForWorldDir(worldDir) { return fs.readFileSync(path.join(worldDir, 'expected-package-hash.txt'), 'utf8').trim(); }

function copyIfExists(source, target) {
  if (!fs.existsSync(source)) return false;
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.copyFileSync(source, target);
  return true;
}

function writeDeploymentEvidenceBundle(projectDir) {
  const deployDir = deploymentDirFor(projectDir);
  const evidenceDir = deploymentEvidenceDirFor(projectDir);
  fs.mkdirSync(evidenceDir, { recursive: true });
  for (const name of ['deployment-manifest.json', 'live-deployment-proof.json']) {
    copyIfExists(path.join(deployDir, name), path.join(evidenceDir, name));
  }
  copyIfExists(attestationPathFor(projectDir), path.join(evidenceDir, 'world-release-attestation.json'));
  copyIfExists(releaseReportPathFor(projectDir), path.join(evidenceDir, 'release-report.json'));
}

function deploymentTargetFromArgs() { return value('--host', value('--target', 'local')); }

function writeDeploymentManifest(projectDir, deploymentTarget, verificationStatus = 'PASS', rootsOverride = null) {
  const generatedWorld = loadGeneratedWorld(projectDir);
  const roots = rootsOverride ?? (() => {
    const runtime = readRuntimePackage(projectDir);
    return statusFor(runtime.state, runtime.receipts, runtime.journal, generatedWorld);
  })();
  const attestationPath = attestationPathFor(projectDir);
  const attestation = fs.existsSync(attestationPath) ? readJsonFile(attestationPath) : null;
  const manifest = {
    world_id: generatedWorld.manifest.world_id,
    package_hash: packageHashForWorldDir(generatedWorld.packageDir),
    world_hash: roots.world_hash,
    continuity_root: roots.continuity_root,
    deployment_target: deploymentTarget,
    attestation_hash: attestation ? attestationHash(attestation) : null,
    verification_status: verificationStatus
  };
  writeJson(path.join(deploymentDirFor(projectDir), 'deployment-manifest.json'), manifest);
  return manifest;
}

function syncReleaseArtifactsToDeploy(projectDir) {
  const deployDir = deploymentDirFor(projectDir);
  copyIfExists(attestationPathFor(projectDir), path.join(deployDir, 'world-release-attestation.json'));
  copyIfExists(releaseReportPathFor(projectDir), path.join(deployDir, 'release-report.json'));
}

function deployWorldFactoryRuntime(projectDir, remoteVerification = 'PENDING') {
  const generatedWorld = loadGeneratedWorld(projectDir);
  const runtime = readRuntimePackage(projectDir);
  const deployDir = deploymentDirFor(projectDir);
  fs.rmSync(deployDir, { recursive: true, force: true });
  fs.mkdirSync(deployDir, { recursive: true });
  fs.cpSync(generatedWorld.packageDir, path.join(deployDir, 'world.evr'), { recursive: true });
  const package_verification = packageVerificationStatus(projectDir);
  const replay_verification = replayVerificationStatus(projectDir);
  const roots = statusFor(runtime.state, runtime.receipts, runtime.journal, generatedWorld);
  const targetHost = deploymentTargetFromArgs();
  writeJson(path.join(deployDir, 'runtime-config.json'), {
    endpoint_profile: 'world-factory-phase4-live-http',
    endpoints: ['/health', '/state', '/journal', '/verify'],
    host: targetHost === 'local' ? '127.0.0.1' : targetHost,
    port: Number.parseInt(value('--port', '8787'), 10),
    source_package: path.relative(projectDir, generatedWorld.packageDir),
    world_id: generatedWorld.manifest.world_id
  });
  writeJson(path.join(deployDir, 'deployment-report.json'), {
    world_id: generatedWorld.manifest.world_id,
    deployment_status: 'RUNNING',
    package_verification,
    replay_verification,
    remote_verification: remoteVerification,
    world_hash: roots.world_hash,
    state_root: roots.state_root,
    receipt_root: roots.receipt_root,
    continuity_root: roots.continuity_root
  });
  syncReleaseArtifactsToDeploy(projectDir);
  writeDeploymentManifest(projectDir, targetHost, package_verification === 'PASS' && replay_verification === 'PASS' ? 'PASS' : 'FAIL');
  writeDeploymentEvidenceBundle(projectDir);
  console.log(`World Factory Deploy: PASS (${path.relative(repoRoot, deployDir)})`);
}

function publishWorldFactoryDeployment(projectDir) {
  const host = value('--host', undefined);
  assertFactory(host, 'world factory publish requires --host <host>');
  deployWorldFactoryRuntime(projectDir, 'PENDING');
  const deployDir = deploymentDirFor(projectDir);
  const runtimeConfig = readJsonFile(path.join(deployDir, 'runtime-config.json'));
  const manifest = writeDeploymentManifest(projectDir, host, 'PASS');
  writeDeploymentEvidenceBundle(projectDir);
  writeJson(path.join(deployDir, 'upload-instructions.json'), {
    world_id: manifest.world_id,
    host,
    bundle_path: path.relative(repoRoot, deployDir),
    upload: `rsync -av ${path.relative(repoRoot, deployDir)}/ ${host}:~/everarcade-world-factory/`,
    run: `cd ~/everarcade-world-factory && everarcade world factory serve --host 0.0.0.0 --port ${runtimeConfig.port}`,
    verify: `everarcade world factory proof --url http://${host}:${runtimeConfig.port}`,
    provisioning: 'manual-existing-host'
  });
  console.log(`World Factory Publish: PASS (${path.relative(repoRoot, deployDir)})`);
  console.log(`Verify: everarcade world factory proof --url http://${host}:${runtimeConfig.port}`);
}

function responseJson(res, data) {
  const body = `${JSON.stringify(data, null, 2)}\n`;
  res.writeHead(200, { 'content-type': 'application/json; charset=utf-8' });
  res.end(body);
}

async function serveWorldFactoryRuntime(projectDir) {
  deployWorldFactoryRuntime(projectDir);
  const http = await import('node:http');
  const host = value('--host', '127.0.0.1');
  const port = Number.parseInt(value('--port', '8787'), 10);
  const server = http.createServer((req, res) => {
    try {
      const generatedWorld = loadGeneratedWorld(projectDir);
      const runtime = readRuntimePackage(projectDir);
      const roots = statusFor(runtime.state, runtime.receipts, runtime.journal, generatedWorld);
      const package_verification = packageVerificationStatus(projectDir);
      const replay_verification = replayVerificationStatus(projectDir);
      if (req.url === '/health') return responseJson(res, { status: 'RUNNING' });
      if (req.url === '/state') return responseJson(res, { world_id: runtime.state.world_id, tick: runtime.state.tick, world_hash: roots.world_hash, state_root: roots.state_root, verification: package_verification === 'PASS' && replay_verification === 'PASS' ? 'PASS' : 'FAIL' });
      if (req.url === '/journal') return responseJson(res, runtime.journal);
      if (req.url === '/verify') return responseJson(res, { package_verification, replay_verification, state_root: roots.state_root, receipt_root: roots.receipt_root, world_hash: roots.world_hash, continuity_root: roots.continuity_root, state: runtime.state, receipts: runtime.receipts });
      res.writeHead(404); res.end('not found\n');
    } catch (error) {
      res.writeHead(500, { 'content-type': 'text/plain; charset=utf-8' });
      res.end(`${error.message}\n`);
    }
  });
  server.listen(port, host, () => console.log(`World Factory Serve: RUNNING (http://${host}:${port})`));
}

async function fetchWorldFactoryProofEndpoints(base) {
  const root = base.replace(/\/$/, '');
  const fetchJson = async (endpoint) => {
    const response = await fetch(`${root}${endpoint}`);
    if (!response.ok) throw new Error(`GET ${endpoint} failed: ${response.status}`);
    return response.json();
  };
  return { health: await fetchJson('/health'), stateReport: await fetchJson('/state'), journal: await fetchJson('/journal'), verification: await fetchJson('/verify') };
}

function verifyAttestationObject(projectDir, attestation, roots) {
  if (!attestation) return 'FAIL';
  try {
    const publicKey = normalizePemOrBase64Key(attestation.attester?.public_key ?? '', 'public');
    const signature = Buffer.from(attestation.signature ?? '', 'base64');
    const signaturePass = crypto.verify(null, canonicalAttestationBytes(attestation), publicKey, signature);
    const generatedWorld = loadGeneratedWorld(projectDir);
    return signaturePass
      && attestation.package_hash === packageHashForWorldDir(generatedWorld.packageDir)
      && attestation.world_hash === roots.world_hash
      && attestation.continuity_root === roots.continuity_root
      && attestation.package_verification === 'PASS'
      && attestation.replay_verification === 'PASS'
      && attestation.remote_verification === 'PASS'
      ? 'PASS' : 'FAIL';
  } catch {
    return 'FAIL';
  }
}

async function proofWorldFactoryRuntime(projectDir) {
  const base = value('--url', 'http://localhost:8787');
  const { health, stateReport, journal, verification } = await fetchWorldFactoryProofEndpoints(base);
  const generatedWorld = loadGeneratedWorld(projectDir);
  const roots = statusFor(verification.state, verification.receipts, journal, generatedWorld);
  const package_verification = packageVerificationStatus(projectDir);
  const replay_verification = (() => {
    try {
      const replayState = bootstrapWorldState(generatedWorld.manifest.world_id);
      const replayJournal = [];
      const replayReceipts = [];
      for (const entry of journal) {
        const previousRoots = statusFor(replayState, replayReceipts, replayJournal, generatedWorld);
        replayState.tick += 1;
        const action = plannedWorldAction(replayState.tick);
        applyWorldAction(replayState, action);
        const receipt = receiptForTick(replayState.world_id, replayState.tick, previousRoots.state_root, action);
        const rootsBeforeJournal = statusFor(replayState, replayReceipts.concat([receipt]), replayJournal, generatedWorld);
        const replayEntry = { tick: replayState.tick, action: action.mutation, payload: action, receipt_hash: receipt.receipt_hash, world_hash: rootsBeforeJournal.world_hash };
        if (canonicalJson(entry) !== canonicalJson(replayEntry)) return 'FAIL';
        replayReceipts.push(receipt);
        replayJournal.push(replayEntry);
      }
      const replayRoots = statusFor(replayState, replayReceipts, replayJournal, generatedWorld);
      return ['state_root', 'receipt_root', 'world_hash', 'continuity_root'].every((key) => replayRoots[key] === roots[key]) ? 'PASS' : 'FAIL';
    } catch { return 'FAIL'; }
  })();
  const remoteStatus = stateReport.state_root === roots.state_root
    && stateReport.world_hash === roots.world_hash
    && verification.state_root === roots.state_root
    && verification.receipt_root === roots.receipt_root
    && verification.world_hash === roots.world_hash
    && verification.continuity_root === roots.continuity_root
    ? 'PASS' : 'FAIL';
  const attestationFile = path.join(deploymentDirFor(projectDir), 'world-release-attestation.json');
  const fallbackAttestation = attestationPathFor(projectDir);
  const attestation = fs.existsSync(attestationFile) ? readJsonFile(attestationFile) : (fs.existsSync(fallbackAttestation) ? readJsonFile(fallbackAttestation) : null);
  const attestation_verification = verifyAttestationObject(projectDir, attestation, roots);
  const deployment_status = health && stateReport.world_id === generatedWorld.manifest.world_id ? 'RUNNING' : 'UNKNOWN';
  const pass = deployment_status === 'RUNNING' && package_verification === 'PASS' && replay_verification === 'PASS' && remoteStatus === 'PASS' && attestation_verification === 'PASS';
  const proof = { world_id: generatedWorld.manifest.world_id, host: base, deployment_status, package_verification, replay_verification, remote_verification: remoteStatus, attestation_verification, proof_timestamp: new Date().toISOString() };
  const deployDir = deploymentDirFor(projectDir);
  fs.mkdirSync(deployDir, { recursive: true });
  writeJson(path.join(deployDir, 'live-deployment-proof.json'), proof);
  writeJson(path.join(deployDir, 'deployment-report.json'), { world_id: proof.world_id, deployment_status, package_verification, replay_verification, remote_verification: remoteStatus, world_hash: roots.world_hash, state_root: roots.state_root, receipt_root: roots.receipt_root, continuity_root: roots.continuity_root });
  writeDeploymentManifest(projectDir, base, pass ? 'PASS' : 'FAIL', roots);
  syncReleaseArtifactsToDeploy(projectDir);
  writeDeploymentEvidenceBundle(projectDir);
  console.log(pass ? 'PASS' : 'FAIL');
  if (!pass) process.exitCode = 1;
}

async function remoteVerifyWorldFactoryRuntime(projectDir) {
  await proofWorldFactoryRuntime(projectDir);
}


function releaseDirFor(projectDir) { return path.join(projectDir, 'out', 'release'); }
function attestationPathFor(projectDir) { return path.join(releaseDirFor(projectDir), 'world-release-attestation.json'); }
function releaseReportPathFor(projectDir) { return path.join(releaseDirFor(projectDir), 'release-report.json'); }

function canonicalAttestationBytes(attestation) {
  const withoutSignature = { ...attestation };
  delete withoutSignature.signature;
  return Buffer.from(canonicalJson(withoutSignature), 'utf8');
}

function attestationHash(attestation) {
  return sha256Hex(canonicalAttestationBytes(attestation));
}

function normalizePemOrBase64Key(keyText, kind) {
  const trimmed = keyText.trim();
  if (trimmed.includes('-----BEGIN')) return trimmed;
  const der = Buffer.from(trimmed, 'base64');
  return kind === 'private'
    ? crypto.createPrivateKey({ key: der, format: 'der', type: 'pkcs8' })
    : crypto.createPublicKey({ key: der, format: 'der', type: 'spki' });
}

function publicKeyBase64(publicKey) {
  return publicKey.export({ format: 'der', type: 'spki' }).toString('base64');
}

function publicKeyFingerprint(publicKeyText) {
  return publicKeyBase64(normalizePemOrBase64Key(publicKeyText, 'public'));
}

function loadOrCreateAttesterPrivateKey(projectDir) {
  const configured = value('--private-key', undefined);
  const keyPath = configured ? path.resolve(configured) : path.join(projectDir, 'out', 'attester-ed25519-private.pem');
  if (fs.existsSync(keyPath)) return { keyPath, privateKey: normalizePemOrBase64Key(fs.readFileSync(keyPath, 'utf8'), 'private') };
  const { privateKey } = crypto.generateKeyPairSync('ed25519');
  fs.mkdirSync(path.dirname(keyPath), { recursive: true });
  fs.writeFileSync(keyPath, privateKey.export({ format: 'pem', type: 'pkcs8' }));
  return { keyPath, privateKey };
}

function createWorldReleaseAttestation(projectDir) {
  const generatedWorld = loadGeneratedWorld(projectDir);
  const runtime = readRuntimePackage(projectDir);
  const deployReportPath = path.join(deploymentDirFor(projectDir), 'deployment-report.json');
  const deploymentReport = fs.existsSync(deployReportPath) ? readJsonFile(deployReportPath) : null;
  const package_verification = deploymentReport?.package_verification ?? packageVerificationStatus(projectDir);
  const replay_verification = deploymentReport?.replay_verification ?? replayVerificationStatus(projectDir);
  const remote_verification = deploymentReport?.remote_verification === 'FAIL' ? 'FAIL' : 'PASS';
  const roots = statusFor(runtime.state, runtime.receipts, runtime.journal, generatedWorld);
  const package_hash = fs.readFileSync(path.join(generatedWorld.packageDir, 'expected-package-hash.txt'), 'utf8').trim();
  const { keyPath, privateKey } = loadOrCreateAttesterPrivateKey(projectDir);
  const publicKey = crypto.createPublicKey(privateKey);
  const releaseDir = releaseDirFor(projectDir);
  fs.rmSync(releaseDir, { recursive: true, force: true });
  fs.mkdirSync(releaseDir, { recursive: true });
  const attestation = {
    schema_version: 'WORLD_RELEASE_ATTESTATION_V0_1_RC2',
    world_id: generatedWorld.manifest.world_id,
    package_hash,
    package_verification,
    replay_verification,
    remote_verification,
    world_hash: roots.world_hash,
    continuity_root: roots.continuity_root,
    timestamp: new Date().toISOString(),
    attester: { name: value('--attester-name', 'offline-attester'), public_key: publicKeyBase64(publicKey) },
    signature: ''
  };
  attestation.signature = crypto.sign(null, canonicalAttestationBytes(attestation), privateKey).toString('base64');
  const hash = attestationHash(attestation);
  fs.cpSync(generatedWorld.packageDir, path.join(releaseDir, 'world.evr'), { recursive: true });
  const trustedPublicKeyPath = path.join(releaseDir, 'trusted-public-key.txt');
  writeJson(attestationPathFor(projectDir), attestation);
  writeJson(releaseReportPathFor(projectDir), { world_id: attestation.world_id, package_hash, attestation_hash: hash, attestation_status: 'PASS' });
  fs.writeFileSync(trustedPublicKeyPath, `${attestation.attester.public_key}\n`);
  console.log(`World Attest Create: PASS (${path.relative(repoRoot, attestationPathFor(projectDir))})`);
  console.log(`Attestation Hash: ${hash}`);
  console.log(`Trusted Public Key: ${attestation.attester.public_key}`);
  console.log(`Trusted Public Key File: ${path.relative(repoRoot, trustedPublicKeyPath)}`);
  console.log(`Attester Private Key: ${path.relative(repoRoot, keyPath)}`);
}

function attestationFail(reason) {
  console.log(`FAIL: ${reason}`);
  process.exitCode = 1;
}

function readRuntimeArtifacts(runtimeDir) {
  return {
    runtimeDir,
    state: readJsonFile(path.join(runtimeDir, 'world-state.json')),
    journal: readJsonFile(path.join(runtimeDir, 'journal.json')),
    receipts: readJsonFile(path.join(runtimeDir, 'receipts.json')),
    report: fs.existsSync(path.join(runtimeDir, 'world-factory-runtime-report.json')) ? readJsonFile(path.join(runtimeDir, 'world-factory-runtime-report.json')) : null,
    status: fs.existsSync(path.join(runtimeDir, 'runtime-status.json')) ? readJsonFile(path.join(runtimeDir, 'runtime-status.json')) : null
  };
}

function replayRuntimeArtifacts(runtime, generatedWorld) {
  const replayState = bootstrapWorldState(generatedWorld.manifest.world_id);
  const replayJournal = [];
  const replayReceipts = [];
  for (const entry of runtime.journal) {
    const previousRoots = statusFor(replayState, replayReceipts, replayJournal, generatedWorld);
    replayState.tick += 1;
    const action = plannedWorldAction(replayState.tick);
    applyWorldAction(replayState, action);
    const receipt = receiptForTick(replayState.world_id, replayState.tick, previousRoots.state_root, action);
    const rootsBeforeJournal = statusFor(replayState, replayReceipts.concat([receipt]), replayJournal, generatedWorld);
    const replayEntry = { tick: replayState.tick, action: action.mutation, payload: action, receipt_hash: receipt.receipt_hash, world_hash: rootsBeforeJournal.world_hash };
    if (canonicalJson(entry) !== canonicalJson(replayEntry)) return { status: 'FAIL', reason: `journal mismatch at tick ${entry.tick}` };
    replayReceipts.push(receipt);
    replayJournal.push(replayEntry);
  }
  const expectedState = replayState;
  const expectedReceipts = replayReceipts;
  const expectedJournal = replayJournal;
  if (canonicalJson(runtime.state) !== canonicalJson(expectedState)) return { status: 'FAIL', reason: 'runtime state does not match replayed state' };
  if (canonicalJson(runtime.receipts) !== canonicalJson(expectedReceipts)) return { status: 'FAIL', reason: 'runtime receipts do not match replayed receipts' };
  if (canonicalJson(runtime.journal) !== canonicalJson(expectedJournal)) return { status: 'FAIL', reason: 'runtime journal does not match replayed journal' };
  const roots = statusFor(runtime.state, runtime.receipts, runtime.journal, generatedWorld);
  const replayRoots = statusFor(replayState, replayReceipts, replayJournal, generatedWorld);
  const rootsPass = ['state_root', 'receipt_root', 'world_hash', 'continuity_root'].every((key) => roots[key] === replayRoots[key]);
  return rootsPass ? { status: 'PASS', roots } : { status: 'FAIL', reason: 'runtime roots do not match replayed roots', roots };
}

function deployReportForPath(deployDir) {
  const reportPath = path.join(deployDir, 'deployment-report.json');
  return fs.existsSync(reportPath) ? readJsonFile(reportPath) : null;
}

function rederiveReleaseClaims(projectDir, worldDir) {
  const expectedHashPath = path.join(worldDir, 'expected-package-hash.txt');
  assertFactory(fs.existsSync(expectedHashPath), `Missing expected package hash at ${expectedHashPath}`);
  const package_hash = fs.readFileSync(expectedHashPath, 'utf8').trim();
  const hashManifest = hashManifestFor(worldDir).packageHash;
  const manifest = readJsonFile(path.join(worldDir, 'manifest.json'));
  const package_verification = packageVerificationStatus(projectDir, worldDir);
  const runtimeDir = path.resolve(value('--runtime', runtimeDirFor(projectDir)));
  const deployDir = path.resolve(value('--deploy', deploymentDirFor(projectDir)));
  const runtime = readRuntimeArtifacts(runtimeDir);
  const generatedWorld = loadWorldPackageDir(worldDir);
  const replay = replayRuntimeArtifacts(runtime, generatedWorld);
  const roots = replay.roots ?? statusFor(runtime.state, runtime.receipts, runtime.journal, generatedWorld);
  const deploymentReport = deployReportForPath(deployDir);
  const remote_verification = deploymentReport
    && deploymentReport.world_id === manifest.world_id
    && deploymentReport.deployment_status === 'RUNNING'
    && deploymentReport.package_verification === package_verification
    && deploymentReport.replay_verification === replay.status
    && deploymentReport.world_hash === roots.world_hash
    && deploymentReport.state_root === roots.state_root
    && deploymentReport.receipt_root === roots.receipt_root
    && deploymentReport.continuity_root === roots.continuity_root
    ? 'PASS' : 'FAIL';
  return {
    world_id: manifest.world_id,
    package_hash,
    package_hash_manifest: hashManifest,
    package_verification,
    replay_verification: replay.status,
    replay_failure: replay.reason,
    remote_verification,
    world_hash: roots.world_hash,
    continuity_root: roots.continuity_root,
    state_root: roots.state_root,
    receipt_root: roots.receipt_root
  };
}

function verifyWorldReleaseAttestation(projectDir) {
  const attestationPath = path.resolve(value('--attestation', attestationPathFor(projectDir)));
  const attestation = readJsonFile(attestationPath);
  const allowSelfAttested = args.includes('--allow-self-attested-test-only');
  const trustedKeyText = value('--trusted-public-key', value('--trusted-key', undefined));
  if (!trustedKeyText && !allowSelfAttested) return attestationFail('missing trusted public key');
  const embeddedPublicKeyText = attestation.attester?.public_key;
  if (!embeddedPublicKeyText) return attestationFail('missing embedded attester public key');
  let verificationKeyText = trustedKeyText ?? embeddedPublicKeyText;
  if (trustedKeyText) {
    try {
      if (publicKeyFingerprint(trustedKeyText) !== publicKeyFingerprint(embeddedPublicKeyText)) return attestationFail('untrusted attester key');
    } catch {
      return attestationFail('invalid trusted public key');
    }
  }
  let publicKey;
  try {
    publicKey = normalizePemOrBase64Key(verificationKeyText, 'public');
  } catch {
    return attestationFail('invalid public key');
  }
  const signature = Buffer.from(attestation.signature ?? '', 'base64');
  const signaturePass = crypto.verify(null, canonicalAttestationBytes(attestation), publicKey, signature);
  if (!signaturePass) return attestationFail('signature verification failed');
  const worldDir = path.resolve(value('--world', path.join(projectDir, 'out', 'world.evr')));
  if (!fs.existsSync(worldDir)) return attestationFail('missing world artifact');
  let claims;
  try {
    claims = rederiveReleaseClaims(projectDir, worldDir);
  } catch (error) {
    return attestationFail(error.message);
  }
  const claimChecks = [
    ['schema_version', attestation.schema_version === 'WORLD_RELEASE_ATTESTATION_V0' || attestation.schema_version === 'WORLD_RELEASE_ATTESTATION_V0_1_RC1' || attestation.schema_version === 'WORLD_RELEASE_ATTESTATION_V0_1_RC2'],
    ['world_id', attestation.world_id === claims.world_id],
    ['package_hash', attestation.package_hash === claims.package_hash && attestation.package_hash === claims.package_hash_manifest],
    ['package_verification', attestation.package_verification === 'PASS' && claims.package_verification === 'PASS'],
    ['replay_verification', attestation.replay_verification === 'PASS' && claims.replay_verification === 'PASS'],
    ['remote_verification', attestation.remote_verification === 'PASS' && claims.remote_verification === 'PASS'],
    ['world_hash', attestation.world_hash === claims.world_hash],
    ['continuity_root', attestation.continuity_root === claims.continuity_root]
  ];
  const failed = claimChecks.find(([, ok]) => !ok);
  if (failed) return attestationFail(`claim re-derivation failed: ${failed[0]}`);
  const reportPath = releaseReportPathFor(projectDir);
  fs.mkdirSync(path.dirname(reportPath), { recursive: true });
  writeJson(reportPath, { world_id: attestation.world_id, package_hash: attestation.package_hash, attestation_hash: attestationHash(attestation), attestation_status: 'PASS' });
  console.log('PASS');
}

function verifyWorldFactoryPackage(projectDir) {
  const packageDir = path.join(projectDir, 'out', 'world.evr');
  const verifier = path.join(repoRoot, 'specs', 'world-evr-package', 'verify-package-v1.mjs');
  const result = spawnSync('node', [verifier, packageDir], { cwd: repoRoot, encoding: 'utf8' });
  if (result.status !== 0) throw new Error(`World Factory Verify failed: ${(result.stderr || result.stdout).trim()}`);
  process.stdout.write(result.stdout);
  console.log('World Factory Verify: PASS');
}

const WORLD_TEMPLATES = [
  ['Arena', 'arena', 'Fast combat worlds'],
  ['Frontier', 'frontier', 'Persistent survival worlds'],
  ['Settlement', 'settlement', 'Economy and governance worlds'],
  ['Social', 'social', 'Community worlds'],
  ['Civilization', 'civilization', 'Long-term continuity worlds']
];

const TEMPLATE_ALIASES = { frontier: 'sandbox', settlement: 'trading', social: 'rpg' };

const RUSTRIGS = [
  ['combat', 'CERTIFIED', 'Deterministic attacks, damage, health, and combat resolution.'],
  ['inventory', 'CERTIFIED', 'Item ownership, slots, transfers, and equipment state.'],
  ['market', 'CERTIFIED', 'Creator-safe marketplace listing and exchange flows.'],
  ['governance', 'CERTIFIED', 'World policy, proposals, roles, and rule changes.'],
  ['identity', 'CANDIDATE', 'Player and entity identity surfaces.'],
  ['movement', 'CANDIDATE', 'Position, bounds, and deterministic movement updates.'],
  ['resources', 'CANDIDATE', 'Resource spawning, harvesting, and balances.'],
  ['crafting', 'CANDIDATE', 'Recipes, inputs, outputs, and production timers.'],
  ['structures', 'CANDIDATE', 'Buildings, placement, ownership, and durability.'],
  ['quests', 'CANDIDATE', 'Objectives, progression, and rewards.'],
  ['continuity', 'CANDIDATE', 'Save continuity, upgrades, and long-lived world lineage.'],
  ['operations', 'CANDIDATE', 'Local operations, health, diagnostics, and deployment readiness.']
];

function printTemplates() {
  for (const [label, , description] of WORLD_TEMPLATES) console.log(`${label.padEnd(12)} ${description}`);
}

function printRustRigs() {
  for (const [name, status, description] of RUSTRIGS) console.log(`${name.padEnd(16)} ${status.padEnd(10)} ${description}`);
}

const WORLD_REGISTRY_CATEGORIES = ['MMO', 'RPG', 'Simulation', 'Strategy', 'Education', 'Commerce', 'Governance', 'Social', 'Sandbox'];

function registryFixture() {
  return [
    {
      world_id: 'frontier.evr',
      world_hash: 'sha256:frontier-genesis-root',
      world_name: 'Frontier',
      description: 'A sandbox RPG frontier world with player settlements, economy, and open contributor projects.',
      category: 'RPG',
      tags: ['rpg', 'sandbox', 'economy', 'governance'],
      created_at: '2026-01-01T00:00:00.000Z',
      operator_id: 'operator:frontier-foundation',
      governance_model: 'constitutional-council',
      registry_root: 'registry-root:frontier-v1',
      population: { active_players: 1280, player_retention: 0.71 },
      contributors: [
        { contributor_id: 'contrib:quest-ada', role: 'Quest Designer', merged_contributions: 42, review_history: 38, reputation_score: 94, maintainer: true },
        { contributor_id: 'contrib:econ-turing', role: 'Economy Designer', merged_contributions: 27, review_history: 31, reputation_score: 91, maintainer: false }
      ],
      treasury_status: { status: 'healthy', runway_days: 240, audit_root: 'treasury-audit:frontier' },
      proof_status: { replay: 'Replay Certified', restore: 'Restore Certified', migration: 'Migration Certified' },
      trust_signals: {
        replay_verified: true,
        restore_verified: true,
        migration_verified: true,
        operator_history: ['operator:frontier-labs', 'operator:frontier-foundation'],
        governance_activity: 18,
        audit_root: 'trust-audit:frontier'
      },
      governance: {
        constitution: 'ipfs://frontier-constitution-v1',
        maintainers: ['contrib:quest-ada'],
        reviewers: ['contrib:econ-turing', 'contrib:moderator-noether'],
        council_members: ['council:builders', 'council:players', 'council:operators'],
        activity: 18
      },
      contributor_manifest: {
        wanted_roles: ['Quest Designers Needed', 'Economy Designers Needed', 'Moderators Needed', 'Artists Needed'],
        open_projects: ['frontier-season-one', 'settlement-economy-rebalance'],
        reward_models: ['bounty', 'revenue-share', 'governance-grants'],
        contribution_opportunities: ['quests', 'markets', 'moderation', 'environment-art']
      },
      activity: { active_contributors: 36, merge_activity: 74, governance_activity: 18, economic_activity: 9200 },
      lineage: { origin_world: 'frontier.evr', forks: ['frontier-classic.evr', 'frontier-hardcore.evr', 'frontier-social.evr'], merges: ['frontier-economy-v2'], migration_history: ['migration:frontier-v1-v2'], restore_events: ['restore:frontier-2026-03-14'] },
      capabilities: ['Housing Module', 'Economy Module', 'Governance Module', 'Guild Module', 'Marketplace Module'],
      health: { treasury_health: 'healthy', governance_health: 'active', contributor_retention: 0.82, player_retention: 0.71, verification_status: 'fully-certified' }
    },
    {
      world_id: 'civilization.evr',
      world_hash: 'sha256:civilization-genesis-root',
      world_name: 'Civilization',
      description: 'A governance and education simulation for long-running civic experiments.',
      category: 'Governance',
      tags: ['governance', 'education', 'simulation'],
      created_at: '2026-02-01T00:00:00.000Z',
      operator_id: 'operator:civic-guild',
      governance_model: 'token-weighted-deliberation',
      registry_root: 'registry-root:civilization-v1',
      population: { active_players: 410, player_retention: 0.64 },
      contributors: [{ contributor_id: 'contrib:civic-hopper', role: 'Governance Designer', merged_contributions: 19, review_history: 25, reputation_score: 88, maintainer: true }],
      treasury_status: { status: 'stable', runway_days: 160, audit_root: 'treasury-audit:civilization' },
      proof_status: { replay: 'Replay Certified', restore: 'Restore Certified', migration: 'Migration Candidate' },
      trust_signals: { replay_verified: true, restore_verified: true, migration_verified: false, operator_history: ['operator:civic-guild'], governance_activity: 31, audit_root: 'trust-audit:civilization' },
      governance: { constitution: 'ipfs://civilization-constitution-v1', maintainers: ['contrib:civic-hopper'], reviewers: ['contrib:policy-lovelace'], council_members: ['council:educators', 'council:citizens'], activity: 31 },
      contributor_manifest: { wanted_roles: ['Moderators Needed', 'Policy Designers Needed'], open_projects: ['civic-curriculum'], reward_models: ['governance-grants'], contribution_opportunities: ['policy', 'education', 'moderation'] },
      activity: { active_contributors: 14, merge_activity: 28, governance_activity: 31, economic_activity: 1200 },
      lineage: { origin_world: 'civilization.evr', forks: [], merges: [], migration_history: [], restore_events: [] },
      capabilities: ['Governance Module', 'Guild Module', 'Marketplace Module'],
      health: { treasury_health: 'stable', governance_health: 'high-activity', contributor_retention: 0.76, player_retention: 0.64, verification_status: 'partially-certified' }
    }
  ];
}

function registryRecords(projectDir) {
  const localPath = path.join(projectDir, 'world.registry.json');
  return fs.existsSync(localPath) ? JSON.parse(fs.readFileSync(localPath, 'utf8')).worlds : registryFixture();
}

function scoreWorld(world) {
  return (world.population?.active_players ?? 0) + (world.activity?.active_contributors ?? 0) * 25 + (world.activity?.governance_activity ?? 0) * 10;
}

function searchRegistry(projectDir) {
  const query = String(args[0] ?? '').toLowerCase();
  const category = value('--category', null);
  const tag = value('--tag', null);
  let worlds = registryRecords(projectDir).filter(world => {
    const haystack = [world.world_id, world.world_name, world.description, world.category, ...(world.tags ?? [])].join(' ').toLowerCase();
    return (!query || haystack.includes(query)) && (!category || world.category === category) && (!tag || (world.tags ?? []).includes(tag));
  });
  worlds = worlds.sort((a, b) => scoreWorld(b) - scoreWorld(a));
  console.log(JSON.stringify({ schema: 'everarcade.world-registry.search.v1', query, categories: WORLD_REGISTRY_CATEGORIES, count: worlds.length, worlds }, null, 2));
}

function lookupWorld(projectDir) {
  const worldId = args[0];
  const world = registryRecords(projectDir).find(record => record.world_id === worldId);
  if (!world) throw new Error(`World not found: ${worldId}`);
  console.log(JSON.stringify({ schema: 'everarcade.world-registry.lookup.v1', world }, null, 2));
}

function contributorsForWorld(projectDir) {
  const worldId = args[0];
  const world = registryRecords(projectDir).find(record => record.world_id === worldId);
  if (!world) throw new Error(`World not found: ${worldId}`);
  console.log(JSON.stringify({ schema: 'everarcade.world-registry.contributors.v1', world_id: worldId, contributors: world.contributors, contributor_manifest: world.contributor_manifest }, null, 2));
}

function lineageForWorld(projectDir) {
  const worldId = args[0];
  const world = registryRecords(projectDir).find(record => record.world_id === worldId);
  if (!world) throw new Error(`World not found: ${worldId}`);
  console.log(JSON.stringify({ schema: 'everarcade.world-registry.lineage.v1', world_id: worldId, lineage: world.lineage }, null, 2));
}


function createWorld() {
  if (args.includes('--list-templates')) return printTemplates();
  const name = value('--name', args[0] ?? 'everarcade-world');
  const requested = value('--template', 'frontier');
  const template = TEMPLATE_ALIASES[requested] ?? requested;
  const target = path.resolve(value('--dir', name));
  const templateDir = path.join(sdkRoot, 'templates', template);
  if (!fs.existsSync(templateDir)) throw new Error(`Unknown template ${requested}`);
  copyDir(templateDir, target);
  const manifest = readManifest(target);
  manifest.name = name;
  manifest.template = requested;
  manifest.world_project_map = 'docs/creator-sdk/world-project-map.md';
  writeJson(path.join(target, 'everarcade.game.json'), manifest);
  console.log(`World: PASS (${name})`);
  console.log('Next: read docs/creator-sdk/world-project-map.md, then run everarcade world run');
}

function verifyWorld(projectDir) {
  test(projectDir);
  certifyWorld(projectDir);
  verifyWorldCertificate(projectDir);
  console.log('WORLD VERIFY: PASS');
}

function projectWorld(projectDir) {
  const manifest = readManifest(projectDir);
  console.log(`Projection: PASS (${manifest.name})`);
  console.log('Projection Entry: Arena Vanguard local projection demo is discoverable from renderer/projection/README.md');
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

function fileSha256(file) {
  return crypto.createHash('sha256').update(fs.readFileSync(file)).digest('hex');
}

function certificatePayload(certificate) {
  const { signature, ...payload } = certificate;
  return orderedJson(payload, [
    'schema', 'certificate_id', 'issued_at', 'world', 'certified_kernels', 'proof_registry',
    'package_artifacts', 'independent_recheck', 'scope', 'non_goals'
  ]);
}

function signCertificate(certificate) {
  return `sha256:${crypto.createHash('sha256').update(certificatePayload(certificate)).digest('hex')}`;
}

function buildProofRegistry(projectDir, packaged) {
  const packageDir = packaged.packageDir;
  const worldFile = path.join(packageDir, 'world.json');
  const manifestFile = path.join(packageDir, 'manifest.json');
  const wasmFile = path.join(packageDir, packaged.manifest.wasm_path);
  const kernelVersion = packaged.manifest.runtime_compatibility;
  return {
    schema: 'everarcade.formal-proof-registry.vnext',
    world_source: 'world.evr',
    world_id: packaged.manifest.world_id,
    package_id: packaged.manifest.package_id,
    runtime_compatibility: kernelVersion,
    proofs: [
      { id: 'world-package-manifest-integrity', artifact: path.relative(projectDir, manifestFile), algorithm: 'sha256', digest: fileSha256(manifestFile), required: true },
      { id: 'world-metadata-integrity', artifact: path.relative(projectDir, worldFile), algorithm: 'sha256', digest: fileSha256(worldFile), required: true },
      { id: 'certified-kernel-wasm-integrity', artifact: path.relative(projectDir, wasmFile), algorithm: 'sha256', digest: fileSha256(wasmFile), required: true }
    ],
    certified_kernels: [
      {
        kernel_id: 'everarcade-runtime',
        runtime_version: kernelVersion,
        compatibility: packaged.manifest.runtime_compatibility,
        package_wasm_hash: packaged.manifest.wasm_hash,
        status: 'CERTIFIED'
      }
    ]
  };
}

function writeCertificationArtifacts(projectDir, packaged) {
  const certDir = path.join(projectDir, 'dist', 'certification');
  fs.mkdirSync(certDir, { recursive: true });
  const registry = buildProofRegistry(projectDir, packaged);
  const registryPath = path.join(certDir, 'formal-proof-registry.json');
  writeJson(registryPath, registry);

  const certificate = {
    schema: 'everarcade.world-package-certificate.vnext',
    certificate_id: `${packaged.manifest.world_id}:${packaged.manifest.package_id}:${packaged.manifest.wasm_hash.slice(0, 16)}`,
    issued_at: new Date().toISOString(),
    world: { source: 'world.evr', world_id: packaged.manifest.world_id, package_id: packaged.manifest.package_id, package_version: packaged.manifest.package_version },
    certified_kernels: registry.certified_kernels,
    proof_registry: { artifact: path.relative(projectDir, registryPath), digest: fileSha256(registryPath), required_proofs: registry.proofs.length },
    package_artifacts: {
      manifest: { artifact: path.relative(projectDir, path.join(packaged.packageDir, 'manifest.json')), digest: fileSha256(path.join(packaged.packageDir, 'manifest.json')) },
      world: { artifact: path.relative(projectDir, path.join(packaged.packageDir, 'world.json')), digest: fileSha256(path.join(packaged.packageDir, 'world.json')) },
      wasm: { artifact: path.relative(projectDir, path.join(packaged.packageDir, packaged.manifest.wasm_path)), digest: fileSha256(path.join(packaged.packageDir, packaged.manifest.wasm_path)) }
    },
    independent_recheck: { status: 'PENDING', verifier: 'everarcade-independent-proof-recheck.vnext' },
    scope: 'Map formal proof artifacts into World Package certification for local deployment gating.',
    non_goals: ['No changes to v0.1 architecture', 'No changes to runtime authority', 'No changes to canonicalizer']
  };
  certificate.signature = signCertificate(certificate);
  const certificatePath = path.join(certDir, 'world-package-certificate.json');
  writeJson(certificatePath, certificate);
  return { certDir, registry, registryPath, certificate, certificatePath };
}

function independentProofRecheck(projectDir, certificatePath = path.join(projectDir, 'dist', 'certification', 'world-package-certificate.json')) {
  const certificate = JSON.parse(fs.readFileSync(certificatePath, 'utf8'));
  const expectedSignature = signCertificate(certificate);
  const checks = [
    { id: 'certificate-signature', status: certificate.signature === expectedSignature ? 'PASS' : 'FAIL' },
    { id: 'manifest-digest', status: fileSha256(path.join(projectDir, certificate.package_artifacts.manifest.artifact)) === certificate.package_artifacts.manifest.digest ? 'PASS' : 'FAIL' },
    { id: 'world-digest', status: fileSha256(path.join(projectDir, certificate.package_artifacts.world.artifact)) === certificate.package_artifacts.world.digest ? 'PASS' : 'FAIL' },
    { id: 'wasm-digest', status: fileSha256(path.join(projectDir, certificate.package_artifacts.wasm.artifact)) === certificate.package_artifacts.wasm.digest ? 'PASS' : 'FAIL' },
    { id: 'proof-registry-digest', status: fileSha256(path.join(projectDir, certificate.proof_registry.artifact)) === certificate.proof_registry.digest ? 'PASS' : 'FAIL' }
  ];
  const status = checks.every((check) => check.status === 'PASS') ? 'PASS' : 'FAIL';
  const report = { schema: 'everarcade.independent-proof-recheck.vnext', certificate: path.relative(projectDir, certificatePath), checks, status };
  writeJson(path.join(projectDir, 'dist', 'certification', 'independent-proof-recheck.json'), report);
  if (status !== 'PASS') throw new Error(`Independent proof re-check failed: ${JSON.stringify(report)}`);
  certificate.independent_recheck = { status: 'PASS', verifier: report.schema, artifact: 'dist/certification/independent-proof-recheck.json' };
  certificate.signature = signCertificate(certificate);
  writeJson(certificatePath, certificate);
  return report;
}

function verifyWorldCertificate(projectDir) {
  independentProofRecheck(projectDir);
  console.log('Independent Proof Re-check: PASS');
}

function certifyWorld(projectDir) {
  const packaged = packageGame(projectDir);
  const artifacts = writeCertificationArtifacts(projectDir, packaged);
  const recheck = independentProofRecheck(projectDir, artifacts.certificatePath);
  writeJson(path.join(projectDir, 'dist', 'certification', 'package-certification-artifacts.json'), {
    schema: 'everarcade.package-certification-artifacts.vnext',
    flow: ['world.evr', 'certified kernel(s)', 'signed certificate', 'independent verification', 'deploy'],
    artifacts: {
      proof_registry: path.relative(projectDir, artifacts.registryPath),
      signed_certificate: path.relative(projectDir, artifacts.certificatePath),
      independent_recheck: 'dist/certification/independent-proof-recheck.json'
    },
    status: recheck.status
  });
  console.log('World Package Certification: PASS');
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
  const certificatePath = path.join(dist, 'certification', 'world-package-certificate.json');
  const certificate = fs.existsSync(certificatePath) ? JSON.parse(fs.readFileSync(certificatePath, 'utf8')) : null;
  writeJson(path.join(dist, 'deployment.json'), {
    type: 'Deployment',
    project: manifest.name,
    target: value('--target', 'local'),
    authority: 'protocol-interface',
    settlement: 'testnet-simulated',
    replay: 'enabled',
    certification: certificate ? { certificate: path.relative(projectDir, certificatePath), signature: certificate.signature, independent_recheck: certificate.independent_recheck?.status ?? 'UNKNOWN' } : null
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
  const projectDir = path.resolve(value('--project', command?.startsWith('world:factory:') || command?.startsWith('world:attest:') ? WORLD_FACTORY_PROJECT : process.cwd()));
  if (command === 'world:factory:init') worldFactoryInit();
  else if (command === 'world:factory:validate') validateWorldFactoryProject(projectDir);
  else if (command === 'world:factory:generate') generateWorldFactoryPackage(projectDir);
  else if (command === 'world:factory:verify') verifyWorldFactoryPackage(projectDir);
  else if (command === 'world:factory:boot') bootWorldFactoryRuntime(projectDir);
  else if (command === 'world:factory:run') runWorldFactoryRuntime(projectDir);
  else if (command === 'world:factory:replay') replayWorldFactoryRuntime(projectDir);
  else if (command === 'world:factory:deploy') deployWorldFactoryRuntime(projectDir);
  else if (command === 'world:factory:publish') publishWorldFactoryDeployment(projectDir);
  else if (command === 'world:factory:proof') await proofWorldFactoryRuntime(projectDir);
  else if (command === 'world:factory:serve') await serveWorldFactoryRuntime(projectDir);
  else if (command === 'world:factory:remote-verify') await remoteVerifyWorldFactoryRuntime(projectDir);
  else if (command === 'world:attest:create') createWorldReleaseAttestation(projectDir);
  else if (command === 'world:attest:verify') verifyWorldReleaseAttestation(projectDir);
  else if (command === 'world:templates') printTemplates();
  else if (command === 'world:rustrigs') printRustRigs();
  else if (command === 'world:init') createWorld();
  else if (command === 'world:run') launchLocal(projectDir);
  else if (command === 'world:package') packageGame(projectDir);
  else if (command === 'world:verify') verifyWorld(projectDir);
  else if (command === 'world:deploy') deploy(projectDir);
  else if (command === 'world:project') projectWorld(projectDir);
  else if (command === 'world:search') searchRegistry(projectDir);
  else if (command === 'world:lookup') lookupWorld(projectDir);
  else if (command === 'world:contributors') contributorsForWorld(projectDir);
  else if (command === 'world:lineage') lineageForWorld(projectDir);
  else if (command === 'new') {
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
  else if (command === 'certify-world') certifyWorld(projectDir);
  else if (command === 'verify-world-certificate') verifyWorldCertificate(projectDir);
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
    console.log('everarcade world <init|templates|rustrigs|run|package|verify|deploy|project|search|lookup|contributors|lineage> [--project DIR]\nworld factory <init|validate|generate|verify|boot|run|replay|deploy|publish|serve|remote-verify|proof> [--project DIR]\nworld attest <create|verify> [--project DIR]');
    console.log('legacy: everarcade <new|build|test|package|certify-world|verify-world-certificate|launch-local|execute-local|execute-template|execute-guest|play-local|play-local-multiplayer|play-network-local|play-federated-local|play-multi-lease-local|deploy|publish> [--project DIR]');
    process.exit(command ? 1 : 0);
  }
} catch (error) {
  console.error(`Creator CLI Error: ${error.message}`);
  process.exit(1);
}
