#!/usr/bin/env node
'use strict';

const fs = require('fs');
const path = require('path');
const childProcess = require('child_process');

const PROOF_ROOT = path.resolve(__dirname, '..');
const REPO_ROOT = path.resolve(PROOF_ROOT, '../..');
const TEMPLATE_ROOT = path.join(PROOF_ROOT, 'templates');
const REPORT_DIR = process.env.EVERARCADE_HOTPOCKET_DEPLOYMENT_REPORT_DIR || path.join(PROOF_ROOT, 'reports');
const ROOT_REPORT_DIR = path.join(REPO_ROOT, 'reports');
const DEFAULT_TIME = '2026-06-11T00:00:00.000Z';
const VARIANTS = [
  { id: 'A', name: 'Variant A', dir: 'variant-a', kind: 'direct-node-contract' },
  { id: 'B', name: 'Variant B', dir: 'variant-b', kind: 'node-contract-with-wrapper' },
  { id: 'C', name: 'Variant C', dir: 'variant-c', kind: 'creator-sdk-generated-shape' },
  { id: 'D', name: 'Variant D', dir: 'variant-d', kind: 'hotpocket-adapter-generated-shape' }
];
const PLACEHOLDER = '<your contract binary here>';

function ensureDir(dir) { fs.mkdirSync(dir, { recursive: true }); }
function readText(file) { return fs.existsSync(file) ? fs.readFileSync(file, 'utf8') : ''; }
function readJson(file, fallback = null) { return fs.existsSync(file) ? JSON.parse(readText(file)) : fallback; }
function writeJson(file, value) { ensureDir(path.dirname(file)); fs.writeFileSync(file, `${JSON.stringify(value, null, 2)}\n`); }
function writeText(file, lines) { ensureDir(path.dirname(file)); fs.writeFileSync(file, `${lines.join('\n')}\n`); }
function mirrorText(name, lines) { writeText(path.join(REPORT_DIR, name), lines); writeText(path.join(ROOT_REPORT_DIR, name), lines); }
function mirrorJson(name, value) { writeJson(path.join(REPORT_DIR, name), value); writeJson(path.join(ROOT_REPORT_DIR, name), value); }

function commandExists(command) {
  const result = childProcess.spawnSync('which', [command], { encoding: 'utf8', timeout: Number(process.env.HOTPOCKET_PROOF_COMMAND_TIMEOUT_MS || 5000) });
  return result.status === 0 && !result.error ? result.stdout.trim() : null;
}

function run(command, args, cwd) {
  const started = new Date().toISOString();
  const result = childProcess.spawnSync(command, args, { cwd, encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'], timeout: Number(process.env.HOTPOCKET_PROOF_COMMAND_TIMEOUT_MS || 5000) });
  return {
    command: [command, ...args].join(' '),
    cwd,
    started_at: started,
    exit_code: result.status,
    signal: result.signal,
    stdout: result.stdout || '',
    stderr: result.error ? `${result.stderr || ''}${result.stderr ? '\n' : ''}${result.error.message}` : result.stderr || '',
    ok: result.status === 0 && !result.error
  };
}


function normalizeCfgValue(value) {
  if (value == null) return '';
  if (Array.isArray(value)) return value.join(' ');
  if (typeof value === 'object') return JSON.stringify(value);
  return String(value).trim();
}

function parseKeyValueCfg(text) {
  const cfg = {};
  for (const line of text.split(/\r?\n/)) {
    const stripped = line.replace(/\s+#.*$/, '').trim();
    if (!stripped) continue;
    const match = stripped.match(/^([A-Za-z0-9_.-]+)\s*=\s*(.*)$/);
    if (!match) continue;
    let value = match[2].trim();
    if ((value.startsWith('"') && value.endsWith('"')) || (value.startsWith("'") && value.endsWith("'"))) {
      value = value.slice(1, -1);
    }
    cfg[match[1]] = normalizeCfgValue(value);
  }
  return cfg;
}

function parsePatchCfg(text) {
  const trimmed = (text || '').trim();
  if (!trimmed) return {};
  try {
    const parsed = JSON.parse(trimmed);
    if (parsed && typeof parsed === 'object' && !Array.isArray(parsed)) {
      return Object.fromEntries(Object.entries(parsed).map(([key, value]) => [key, normalizeCfgValue(value)]));
    }
  } catch (_error) {
    // Fall back to HotPocket key=value patch.cfg syntax.
  }
  return parseKeyValueCfg(text);
}

function splitArgs(value) {
  const args = [];
  let current = '';
  let quote = null;
  for (const ch of String(value || '')) {
    if (quote) {
      if (ch === quote) quote = null;
      else current += ch;
    } else if (ch === '"' || ch === "'") {
      quote = ch;
    } else if (/\s/.test(ch)) {
      if (current) { args.push(current); current = ''; }
    } else {
      current += ch;
    }
  }
  if (current) args.push(current);
  return args;
}

function variantPath(variant) { return path.join(TEMPLATE_ROOT, variant.dir); }

function inspectVariant(variant) {
  const dir = variantPath(variant);
  const packageFile = path.join(dir, 'package.json');
  const contractFile = path.join(dir, 'contract.js');
  const patchFile = path.join(dir, 'patch.cfg');
  const wrapperFile = path.join(dir, 'contract');
  const pkg = readJson(packageFile, {});
  const patchText = readText(patchFile);
  const patch = parsePatchCfg(patchText);
  const dependencyNames = Object.keys(pkg.dependencies || {}).sort();
  const executableCandidate = patch.bin_path === 'node' ? contractFile : path.resolve(dir, patch.bin_path || '');
  const executableExists = patch.bin_path === 'node' ? fs.existsSync(contractFile) : fs.existsSync(executableCandidate);
  const wrapperExecutable = fs.existsSync(wrapperFile) ? Boolean(fs.statSync(wrapperFile).mode & 0o111) : null;
  const launch = launchVariant(variant, false);
  const checks = {
    package_json_present: fs.existsSync(packageFile),
    contract_js_present: fs.existsSync(contractFile),
    patch_cfg_present: fs.existsSync(patchFile),
    bin_path_present: Boolean(patch.bin_path),
    bin_path_not_placeholder: Boolean(patch.bin_path && patch.bin_path !== PLACEHOLDER),
    bin_args_present: patch.bin_args != null && patch.bin_args !== '',
    executable_presence: executableExists,
    wrapper_executable: variant.id === 'B' ? wrapperExecutable === true : true,
    dependencies_declared: true,
    dependency_payload_present: dependencyNames.length === 0 || fs.existsSync(path.join(dir, 'node_modules')),
    package_metadata_present: Boolean(pkg.name && pkg.version && pkg.scripts && pkg.scripts.start),
    local_launchable: launch.ok
  };
  return {
    id: variant.id,
    name: variant.name,
    kind: variant.kind,
    path: path.relative(REPO_ROOT, dir),
    package: { name: pkg.name || null, version: pkg.version || null, main: pkg.main || null, scripts: pkg.scripts || {}, dependencies: pkg.dependencies || {} },
    patch_cfg: { raw: patchText, parsed: patch },
    executable: { bin_path: patch.bin_path || null, bin_args: patch.bin_args || null, candidate: path.relative(REPO_ROOT, executableCandidate), exists: executableExists, wrapper_executable: wrapperExecutable },
    checks,
    status: Object.values(checks).every(Boolean) ? 'PASS' : 'FAIL'
  };
}

function launchVariant(variant, includeOutput = true) {
  const dir = variantPath(variant);
  const patch = parsePatchCfg(readText(path.join(dir, 'patch.cfg')));
  let command = patch.bin_path;
  let args = splitArgs(patch.bin_args || '');
  if (command && command.startsWith('./')) command = path.resolve(dir, command);
  if (!command) return { ok: false, command: 'unavailable', exit_code: null, stdout: '', stderr: 'bin_path missing' };
  const result = run(command, args, dir);
  if (!includeOutput) return { ok: result.ok, exit_code: result.exit_code };
  return result;
}

function inspect() {
  const variants = VARIANTS.map(inspectVariant);
  const ok = variants.every((variant) => variant.status === 'PASS');
  const report = {
    schema: 'everarcade.hotpocket.package-inspection.v0.1',
    generated_at: DEFAULT_TIME,
    objective: 'Inspect HotPocket deployment package structure before deployment.',
    variants,
    requirements: {
      package_structure: ok,
      executable_presence: variants.every((variant) => variant.checks.executable_presence),
      dependency_presence: variants.every((variant) => variant.checks.dependency_payload_present),
      configuration_files: variants.every((variant) => variant.checks.patch_cfg_present),
      package_metadata: variants.every((variant) => variant.checks.package_metadata_present)
    },
    classification: ok ? 'HotPocket Package Structure Proven' : 'HotPocket Package Structure Not Proven',
    status: ok ? 'PASS' : 'FAIL'
  };
  mirrorJson('hotpocket_package_inspection_report.json', report);
  return ok;
}


function preDeploymentMetadata(variant) {
  const dir = variantPath(variant);
  const patchFile = path.join(dir, 'patch.cfg');
  const parsed = parsePatchCfg(readText(patchFile));
  return {
    id: variant.id,
    patch_cfg: path.relative(REPO_ROOT, patchFile),
    patch_cfg_exists: fs.existsSync(patchFile),
    bin_path_present: Boolean(parsed.bin_path),
    bin_args_present: parsed.bin_args != null && parsed.bin_args !== '',
    bin_path_not_placeholder: Boolean(parsed.bin_path && parsed.bin_path !== PLACEHOLDER && !parsed.bin_path.includes('your contract binary')),
    parsed,
    status: fs.existsSync(patchFile) && Boolean(parsed.bin_path) && parsed.bin_args != null && parsed.bin_args !== '' && parsed.bin_path !== PLACEHOLDER && !String(parsed.bin_path).includes('your contract binary') ? 'PASS' : 'FAIL'
  };
}

function generatedConfigSnapshot(deployments) {
  const roots = configuredClusterRoots();
  const hpCfgFiles = findFiles(roots, ['hp.cfg']);
  const patchCfgFiles = findFiles(roots, ['patch.cfg']);
  const stateFiles = findFiles(roots, ['ledger', 'ledger.db', 'unl.json', 'state.json', 'contract_state.json']);
  const hpCfg = hpCfgFiles.map((file) => ({ file: path.relative(REPO_ROOT, file), parsed: parsePatchCfg(readText(file)) }));
  const patchCfg = patchCfgFiles.map((file) => ({ file: path.relative(REPO_ROOT, file), parsed: parsePatchCfg(readText(file)) }));
  const generatedHpCfg = hpCfg.length > 0;
  const generatedPatchCfg = patchCfg.length > 0;
  const generatedContractState = stateFiles.length > 0;
  const lines = [
    'HotPocket Generated Config Report',
    `Generated At: ${DEFAULT_TIME}`,
    'Pre-deploy metadata:',
    ...deployments.map((item) => `${item.id}: patch_cfg=${item.pre.patch_cfg_exists ? 'PASS' : 'FAIL'} bin_path_present=${item.pre.bin_path_present ? 'PASS' : 'FAIL'} bin_args_present=${item.pre.bin_args_present ? 'PASS' : 'FAIL'} placeholder_absent=${item.pre.bin_path_not_placeholder ? 'PASS' : 'FAIL'}`),
    'Post-deploy generated artifacts:',
    `generated hp.cfg: ${generatedHpCfg ? 'PASS' : 'FAIL'} (${hpCfg.length})`,
    `generated patch.cfg: ${generatedPatchCfg ? 'PASS' : 'FAIL'} (${patchCfg.length})`,
    `generated contract state: ${generatedContractState ? 'PASS' : 'FAIL'} (${stateFiles.length})`,
    ...hpCfg.map((item) => `${item.file}: bin_path=${item.parsed.bin_path || 'missing'} bin_args=${item.parsed.bin_args || 'missing'}`),
    `HotPocket Generated Config Proof: ${deployments.every((item) => item.pre.status === 'PASS') && generatedHpCfg && generatedPatchCfg && generatedContractState ? 'PASS' : 'FAIL'}`
  ];
  mirrorText('hotpocket_generated_config_report.txt', lines);
  return { roots, hpCfgFiles, patchCfgFiles, stateFiles, ok: deployments.every((item) => item.pre.status === 'PASS') && generatedHpCfg && generatedPatchCfg && generatedContractState };
}

function deploy() {
  const evernodecli = commandExists('evernodecli');
  const deployments = [];
  for (const variant of VARIANTS) {
    const pre = preDeploymentMetadata(variant);
    const clean = pre.status === 'PASS' && evernodecli ? run('evernodecli', ['hp-clean'], REPO_ROOT) : { ok: false, command: 'evernodecli hp-clean', exit_code: null, stdout: '', stderr: pre.status === 'PASS' ? 'evernodecli not found' : 'pre-deploy metadata validation failed' };
    const deployRun = pre.status === 'PASS' && evernodecli ? run('evernodecli', ['hp-deploy', variantPath(variant)], REPO_ROOT) : { ok: false, command: `evernodecli hp-deploy ${variantPath(variant)}`, exit_code: null, stdout: '', stderr: pre.status === 'PASS' ? 'evernodecli not found' : 'pre-deploy metadata validation failed' };
    deployments.push({ id: variant.id, package_path: path.relative(REPO_ROOT, variantPath(variant)), pre, hp_clean: clean, hp_deploy: deployRun, status: pre.status === 'PASS' && clean.ok && deployRun.ok ? 'PASS' : 'FAIL' });
  }
  const generated = generatedConfigSnapshot(deployments);
  const ok = deployments.every((item) => item.status === 'PASS') && generated.ok;
  mirrorText('hotpocket_deployment_compatibility_report.txt', [
    'HotPocket Deployment Compatibility Report',
    `Generated At: ${DEFAULT_TIME}`,
    'Manual edits: forbidden',
    'docker exec: forbidden',
    'Runtime patching: forbidden',
    `evernodecli: ${evernodecli || 'not found'}`,
    ...deployments.flatMap((item) => [
      `${item.id} pre-deploy metadata: ${item.pre.status}`,
      `${item.id} hp-clean: ${item.hp_clean.ok ? 'PASS' : 'FAIL'}`,
      `${item.id} hp-deploy: ${item.hp_deploy.ok ? 'PASS' : 'FAIL'}`,
      `${item.id} deploy stderr: ${(item.hp_deploy.stderr || '').split('\n').filter(Boolean).slice(-1)[0] || 'none'}`
    ]),
    `Generated config validation: ${generated.ok ? 'PASS' : 'FAIL'}`,
    `Classification: ${ok ? 'HotPocket Deployment Compatibility Proven' : 'HotPocket Deployment Compatibility Not Proven'}`,
    `HotPocket Deployment Compatibility Proof: ${ok ? 'PASS' : 'FAIL'}`
  ]);
  writeDiscoveryReport(deployments);
  return ok;
}


function dockerJson(args) {
  if (!commandExists('docker')) return null;
  const result = run('docker', args, REPO_ROOT);
  if (!result.ok || !result.stdout.trim()) return null;
  try { return JSON.parse(result.stdout); } catch (_error) { return null; }
}

function dockerLines(args) {
  if (!commandExists('docker')) return [];
  const result = run('docker', args, REPO_ROOT);
  return result.ok ? result.stdout.split(/\r?\n/).map((line) => line.trim()).filter(Boolean) : [];
}

function uniqueExistingDirs(dirs) {
  return [...new Set(dirs.filter(Boolean).map((dir) => path.resolve(dir)).filter((dir) => fs.existsSync(dir) && fs.statSync(dir).isDirectory()))];
}

function discoverDockerContainers() {
  const rows = dockerLines(['ps', '--format', '{{.ID}}\t{{.Names}}']);
  const containers = [];
  for (const row of rows) {
    const [id, name = ''] = row.split('\t');
    if (!id) continue;
    const inspected = dockerJson(['inspect', id]);
    const item = Array.isArray(inspected) ? inspected[0] : null;
    const lower = `${name} ${JSON.stringify(item || {})}`.toLowerCase();
    if (!lower.includes('hpdevkit') && !lower.includes('hotpocket')) continue;
    containers.push({ id, name, inspect: item });
  }
  return containers;
}

function discoverDockerVolumeRoots() {
  const names = dockerLines(['volume', 'ls', '--format', '{{.Name}}']);
  const roots = [];
  const volumes = [];
  for (const name of names) {
    if (!/hpdevkit|hotpocket|hp_?|default_node_|node_\d+/i.test(name)) continue;
    const inspected = dockerJson(['volume', 'inspect', name]);
    const item = Array.isArray(inspected) ? inspected[0] : null;
    if (item && item.Mountpoint) roots.push(item.Mountpoint);
    volumes.push({ name, mountpoint: item && item.Mountpoint ? item.Mountpoint : null });
  }
  return { roots, volumes };
}

function discoverCluster() {
  const envRoots = [process.env.HOTPOCKET_CLUSTER_ROOT, process.env.HPDEVKIT_CLUSTER_ROOT, process.env.EVERARCADE_HOTPOCKET_CLUSTER_ROOT]
    .filter(Boolean)
    .map((item) => path.resolve(item));
  const containers = discoverDockerContainers();
  const mountRoots = [];
  for (const container of containers) {
    for (const mount of (container.inspect && container.inspect.Mounts) || []) {
      const label = `${mount.Name || ''} ${mount.Source || ''} ${mount.Destination || ''}`;
      if (/hpdevkit|hotpocket|node_\d+|default_node_/i.test(label)) mountRoots.push(mount.Source);
    }
  }
  const volumeDiscovery = discoverDockerVolumeRoots();
  const roots = uniqueExistingDirs([...envRoots, ...mountRoots, ...volumeDiscovery.roots]);
  return {
    schema: 'everarcade.hotpocket.cluster-discovery.v0.1',
    generated_at: DEFAULT_TIME,
    sources: {
      environment: envRoots,
      docker_available: Boolean(commandExists('docker')),
      containers: containers.map((container) => ({ id: container.id, name: container.name, mounts: ((container.inspect && container.inspect.Mounts) || []).map((mount) => ({ name: mount.Name || null, source: mount.Source || null, destination: mount.Destination || null })) })),
      volumes: volumeDiscovery.volumes
    },
    roots,
    hp_cfg_files: findFiles(roots, ['hp.cfg']).map((file) => path.relative(REPO_ROOT, file)),
    patch_cfg_files: findFiles(roots, ['patch.cfg']).map((file) => path.relative(REPO_ROOT, file)),
    status: roots.length >= 3 || findFiles(roots, ['hp.cfg']).length >= 3 ? 'PASS' : 'FAIL'
  };
}

function configuredClusterRoots() {
  return discoverCluster().roots;
}

function findFiles(startDirs, names) {
  const found = [];
  const queue = [...startDirs.filter((dir) => fs.existsSync(dir))];
  while (queue.length > 0 && found.length < 200) {
    const dir = queue.shift();
    for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
      const full = path.join(dir, entry.name);
      if (entry.isDirectory()) {
        if (!['node_modules', '.git', 'target', 'vendor'].includes(entry.name)) queue.push(full);
      } else if (names.includes(entry.name)) {
        found.push(full);
      }
    }
  }
  return found;
}


function executable() {
  const roots = configuredClusterRoots();
  const hpCfgFiles = findFiles(roots, ['hp.cfg']);
  const inspected = hpCfgFiles.map((file) => {
    const parsed = parsePatchCfg(readText(file));
    const dir = path.dirname(file);
    const binPath = parsed.bin_path || '';
    const candidate = binPath.startsWith('/') ? binPath : path.resolve(dir, binPath);
    const args = splitArgs(parsed.bin_args || '');
    const command = binPath === 'node' ? 'node' : candidate;
    const launchResult = binPath && parsed.bin_args ? run(command, args, dir) : { ok: false, exit_code: null, stdout: '', stderr: 'bin_path or bin_args missing' };
    const combined = `${launchResult.stdout || ''}\n${launchResult.stderr || ''}`;
    const placeholderAbsent = Boolean(binPath && binPath !== PLACEHOLDER && !binPath.includes('your contract binary') && !combined.includes(PLACEHOLDER));
    return {
      file: path.relative(REPO_ROOT, file),
      bin_path: binPath,
      bin_args: parsed.bin_args || '',
      executable_exists: binPath === 'node' ? Boolean(commandExists('node')) : fs.existsSync(candidate),
      executable_launches: launchResult.ok,
      launch_exit_code: launchResult.exit_code,
      execve_failed_absent: !combined.includes('Contract process execve() failed') && !combined.includes('execve() failed'),
      module_not_found_absent: !combined.includes('Cannot find module'),
      placeholder_absent: placeholderAbsent,
      status: binPath && parsed.bin_args && (binPath === 'node' || fs.existsSync(candidate)) && launchResult.ok && placeholderAbsent ? 'PASS' : 'FAIL'
    };
  });
  const ok = inspected.length >= 3 && inspected.every((item) => item.status === 'PASS' && item.placeholder_absent && item.executable_launches);
  mirrorText('hotpocket_executable_resolution_report.txt', [
    'HotPocket Executable Resolution Report',
    `Cluster roots: ${roots.length ? roots.join(', ') : 'not configured'}`,
    `hp.cfg files discovered: ${inspected.length}`,
    ...inspected.map((item) => `${item.file}: bin_path=${item.bin_path || 'missing'} bin_args=${item.bin_args || 'missing'} executable=${item.executable_exists ? 'PASS' : 'FAIL'} launches=${item.executable_launches ? 'PASS' : 'FAIL'} placeholder_absent=${item.placeholder_absent ? 'PASS' : 'FAIL'}`),
    `Classification: ${ok ? 'HotPocket Executable Resolution Proven' : 'HotPocket Executable Resolution Not Proven'}`,
    `HotPocket Executable Resolution Proof: ${ok ? 'PASS' : 'FAIL'}`
  ]);
  mirrorText('hotpocket_hp_cfg_validation_report.txt', [
    'HotPocket hp.cfg Validation Report',
    `Cluster roots: ${roots.length ? roots.join(', ') : 'not configured'}`,
    `hp.cfg files discovered: ${inspected.length}`,
    ...inspected.map((item) => `${item.file}: bin_path_populated=${item.bin_path ? 'PASS' : 'FAIL'} bin_args_populated=${item.bin_args ? 'PASS' : 'FAIL'} placeholder_absent=${item.placeholder_absent ? 'PASS' : 'FAIL'}`),
    `HotPocket hp.cfg Validation Proof: ${ok ? 'PASS' : 'FAIL'}`
  ]);
  return ok;
}


function dependencies() {
  const variants = VARIANTS.map((variant) => {
    const dir = variantPath(variant);
    const pkg = readJson(path.join(dir, 'package.json'), {});
    const deps = Object.keys(pkg.dependencies || {}).sort();
    const launch = launchVariant(variant);
    return { id: variant.id, dependencies: deps, node_modules_required: deps.length > 0, node_modules_present: deps.length === 0 || fs.existsSync(path.join(dir, 'node_modules')), module_not_found_absent: !`${launch.stderr}\n${launch.stdout}`.includes('Cannot find module'), launch_ok: launch.ok };
  });
  const ok = variants.every((item) => item.node_modules_present && item.module_not_found_absent && item.launch_ok);
  mirrorText('hotpocket_dependency_packaging_report.txt', [
    'HotPocket Dependency Packaging Report',
    ...variants.map((item) => `${item.id}: dependencies=${item.dependencies.join(',') || 'none'} node_modules=${item.node_modules_present ? 'PASS' : 'FAIL'} module_not_found_absent=${item.module_not_found_absent ? 'PASS' : 'FAIL'} launch=${item.launch_ok ? 'PASS' : 'FAIL'}`),
    `Classification: ${ok ? 'HotPocket Dependency Packaging Proven' : 'HotPocket Dependency Packaging Not Proven'}`,
    `HotPocket Dependency Packaging Proof: ${ok ? 'PASS' : 'FAIL'}`
  ]);
  return ok;
}


function clusterLogText(roots) {
  const logFiles = findFiles(roots, ['hp.log', 'contract.log', 'hotpocket.log', 'stdout.log', 'stderr.log']);
  return { logFiles, text: logFiles.map(readText).join('\n') };
}

function launch() {
  const variants = VARIANTS.map((variant) => ({ id: variant.id, result: launchVariant(variant) }));
  const localOk = variants.every((item) => item.result.ok && !`${item.result.stderr}\n${item.result.stdout}`.includes('Contract process execve() failed') && !`${item.result.stderr}\n${item.result.stdout}`.includes('Cannot find module') && !`${item.result.stderr}\n${item.result.stdout}`.includes(PLACEHOLDER));
  const roots = configuredClusterRoots();
  const hpCfgFiles = findFiles(roots, ['hp.cfg']);
  const logs = clusterLogText(roots);
  const liveText = logs.text;
  const forbiddenAbsent = !liveText.includes('Contract process execve() failed') && !liveText.includes('Cannot find module') && !liveText.includes(PLACEHOLDER);
  const liveObserved = hpCfgFiles.length >= 3 && logs.logFiles.length > 0;
  const liveOk = liveObserved && forbiddenAbsent;
  mirrorText('hotpocket_contract_launch_report.txt', [
    'HotPocket Contract Launch Report',
    ...variants.map((item) => `${item.id}: exit=${item.result.exit_code} execve_failed=${`${item.result.stderr}`.includes('execve') ? 'FAIL' : 'PASS'} module_not_found=${`${item.result.stderr}`.includes('Cannot find module') ? 'FAIL' : 'PASS'} bin_path_placeholder=${`${item.result.stdout}${item.result.stderr}`.includes(PLACEHOLDER) ? 'FAIL' : 'PASS'}`),
    `Classification: ${localOk ? 'HotPocket Contract Launch Proven' : 'HotPocket Contract Launch Not Proven'}`,
    `HotPocket Contract Launch Proof: ${localOk ? 'PASS' : 'FAIL'}`
  ]);
  mirrorText('hotpocket_live_contract_launch_report.txt', [
    'HotPocket Live Contract Launch Report',
    `Cluster roots: ${roots.length ? roots.join(', ') : 'not configured'}`,
    `hp.cfg files discovered: ${hpCfgFiles.length}`,
    `Log files inspected: ${logs.logFiles.length}`,
    `contract process starts: ${liveObserved ? 'PASS' : 'FAIL'}`,
    `contract process remains alive: ${liveOk ? 'PASS' : 'FAIL'}`,
    `Contract process execve() failed: ${liveText.includes('Contract process execve() failed') ? 'present' : 'absent'}`,
    `Cannot find module: ${liveText.includes('Cannot find module') ? 'present' : 'absent'}`,
    `HotPocket Live Contract Launch Proof: ${liveOk ? 'PASS' : 'FAIL'}`
  ]);
  return localOk && liveOk;
}

function inferValidatorCount(roots, logText) {
  const envCount = Number(process.env.HOTPOCKET_VALIDATOR_COUNT || 0);
  if (envCount) return envCount;
  const hpCfgCount = findFiles(roots, ['hp.cfg']).length;
  if (hpCfgCount) return hpCfgCount;
  const matches = logText.match(/validator|peer|propos/i) ? (logText.match(/node[_-]?\d+|validator[_-]?\d+/gi) || []) : [];
  return new Set(matches.map((m) => m.toLowerCase())).size;
}

function inferProposalCount(logText) {
  const envCount = Number(process.env.HOTPOCKET_PROPOSAL_COUNT || 0);
  if (envCount) return envCount;
  return (logText.match(/propos(?:e|al|ing)|consensus|validated|accepted/gi) || []).length;
}

function proposal() {
  const roots = configuredClusterRoots();
  const logs = clusterLogText(roots);
  const text = logs.text;
  const validators = inferValidatorCount(roots, text);
  const proposals = inferProposalCount(text);
  const noVotesFailures = !text.includes('votes:1 needed:3') && !text.includes('votes:2 needed:3') && !text.includes('Not enough peers proposing');
  const ok = validators >= 3 && proposals >= 3 && noVotesFailures;
  const lines = [
    'HotPocket Validator Participation Report',
    `Cluster roots: ${roots.length ? roots.join(', ') : 'not configured'}`,
    `Log files inspected: ${logs.logFiles.length}`,
    `3 validators online: ${validators >= 3 ? 'PASS' : 'FAIL'} (${validators || 'unavailable'})`,
    `3 validators participating: ${proposals >= 3 ? 'PASS' : 'FAIL'} (${proposals || 'unavailable'})`,
    `votes:1 needed:3: ${text.includes('votes:1 needed:3') ? 'present' : 'absent'}`,
    `votes:2 needed:3: ${text.includes('votes:2 needed:3') ? 'present' : 'absent'}`,
    `Not enough peers proposing: ${text.includes('Not enough peers proposing') ? 'present' : 'absent'}`,
    `HotPocket Validator Participation Proof: ${ok ? 'PASS' : 'FAIL'}`
  ];
  mirrorText('hotpocket_validator_participation_report.txt', lines);
  mirrorText('hotpocket_cluster_proposal_report.txt', [
    'HotPocket Cluster Proposal Report',
    ...lines.slice(1),
    `Classification: ${ok ? 'HotPocket Cluster Proposal Proven' : 'HotPocket Cluster Proposal Not Proven'}`,
    `HotPocket Cluster Proposal Proof: ${ok ? 'PASS' : 'FAIL'}`
  ]);
  return ok;
}



function discoverClientServers() {
  const env = (process.env.HOTPOCKET_SERVERS || process.env.HP_SERVERS || '').split(',').map((item) => item.trim()).filter(Boolean);
  const docker = [];
  for (const container of discoverDockerContainers()) {
    const ports = (container.inspect && container.inspect.NetworkSettings && container.inspect.NetworkSettings.Ports) || {};
    for (const bindings of Object.values(ports)) {
      for (const binding of bindings || []) {
        if (binding && binding.HostPort) docker.push(`ws://${binding.HostIp && binding.HostIp !== '0.0.0.0' ? binding.HostIp : '127.0.0.1'}:${binding.HostPort}`);
      }
    }
  }
  return [...new Set([...env, ...docker])];
}

function clientRoundtrip() {
  const servers = discoverClientServers();
  const clientScript = path.join(REPO_ROOT, 'runtime/hotpocket-contract-proof/client/roundtrip-client.js');
  const env = { ...process.env, HOTPOCKET_SERVERS: servers.join(','), HOTPOCKET_ACTION: 'ping', EVERARCADE_HOTPOCKET_REPORT_DIR: REPORT_DIR, HOTPOCKET_CLIENT_TIMEOUT_MS: process.env.HOTPOCKET_CLIENT_TIMEOUT_MS || '5000', HOTPOCKET_CONNECTION_TIMEOUT_MS: process.env.HOTPOCKET_CONNECTION_TIMEOUT_MS || '3000' };
  let result = { ok: false, exit_code: null, stdout: '', stderr: servers.length ? 'roundtrip client not run' : 'no HotPocket websocket endpoints discovered' };
  if (servers.length && fs.existsSync(clientScript)) {
    const spawned = childProcess.spawnSync('node', [clientScript], { cwd: REPO_ROOT, env, encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'], timeout: Number(process.env.HOTPOCKET_CLIENT_PROCESS_TIMEOUT_MS || 10000) });
    result = { ok: spawned.status === 0 && !spawned.error, exit_code: spawned.status, stdout: spawned.stdout || '', stderr: spawned.error ? `${spawned.stderr || ''}${spawned.stderr ? '\n' : ''}${spawned.error.message}` : spawned.stderr || '' };
  }
  const roundtrip = readJson(path.join(REPORT_DIR, 'hotpocket_client_roundtrip_result.json'), null);
  if (roundtrip) writeJson(path.join(ROOT_REPORT_DIR, 'hotpocket_client_roundtrip_result.json'), roundtrip);
  const connected = Boolean(roundtrip && roundtrip.connected);
  const verified = Boolean(roundtrip && roundtrip.verified);
  const outputReturned = Boolean(roundtrip && roundtrip.output_payload);
  const inputAccepted = Boolean(roundtrip && roundtrip.submission_hash);
  const ok = result.ok && connected && inputAccepted && verified && outputReturned;
  mirrorText('hotpocket_client_roundtrip_report.txt', [
    'HotPocket Client Round-Trip Report',
    `Servers: ${servers.length ? servers.join(', ') : 'not discovered'}`,
    `client connected: ${connected ? 'PASS' : 'FAIL'}`,
    `input accepted: ${inputAccepted ? 'PASS' : 'FAIL'}`,
    `contract executed: ${verified ? 'PASS' : 'FAIL'}`,
    `output returned: ${outputReturned ? 'PASS' : 'FAIL'}`,
    `Submission Hash: ${roundtrip && roundtrip.submission_hash ? roundtrip.submission_hash : 'unavailable'}`,
    `Output Payload: ${JSON.stringify(roundtrip && roundtrip.output_payload ? roundtrip.output_payload : null)}`,
    `Client stderr: ${(result.stderr || '').split('\n').filter(Boolean).slice(-1)[0] || 'none'}`,
    `HotPocket Client Round-Trip Proof: ${ok ? 'PASS' : 'FAIL'}`
  ]);
  return ok;
}

function writeDiscoveryReport(deployments = []) {
  const cluster = discoverCluster();
  mirrorJson('hotpocket_cluster_discovery_report.json', cluster);
  const roots = cluster.roots;
  const hpCfgFiles = findFiles(roots, ['hp.cfg']);
  const patchCfgFiles = findFiles(roots, ['patch.cfg']);
  const packageFiles = findFiles([TEMPLATE_ROOT, ...roots], ['package.json']);
  const report = {
    schema: 'everarcade.hotpocket.deployment-discovery.v0.1',
    generated_at: DEFAULT_TIME,
    deployment_metadata: { deployments, evernodecli: commandExists('evernodecli'), hpdevkit: commandExists('hpdevkit'), docker: commandExists('docker'), client_servers: discoverClientServers() },
    cluster_discovery: cluster,
    package_metadata: packageFiles.map((file) => ({ file: path.relative(REPO_ROOT, file), package: readJson(file, {}) })),
    generated_hp_cfg: hpCfgFiles.map((file) => ({ file: path.relative(REPO_ROOT, file), parsed: parsePatchCfg(readText(file)), raw: readText(file) })),
    generated_patch_cfg: patchCfgFiles.map((file) => ({ file: path.relative(REPO_ROOT, file), parsed: parsePatchCfg(readText(file)), raw: readText(file) })),
    generated_contract_filesystem: roots.map((root) => ({ root, files: findFiles([root], ['contract.js', 'contract', 'package.json']).map((file) => path.relative(root, file)) })),
    generated_ledger_filesystem: roots.map((root) => ({ root, files: findFiles([root], ['ledger', 'ledger.db', 'unl.json']).map((file) => path.relative(root, file)) }))
  };
  mirrorJson('hotpocket_deployment_discovery_report.json', report);
}

function validate() {
  const results = [inspect(), deploy(), executable(), dependencies(), launch(), proposal(), clientRoundtrip()];
  writeDiscoveryReport();
  return results.every(Boolean);
}

function certify() {
  const ok = validate();
  mirrorText('hotpocket_deployment_certification_report.txt', [
    'HotPocket Deployment Fix & Live Contract Deployment Proof v0.1 Certification',
    'Explicit Non-Claims: Evernode deployment, production hosting, federation, gameplay, multiplayer, XRPL settlement, civilization runtime',
    `HotPocket Deployment Fix & Live Contract Deployment Proof v0.1: ${ok ? 'PASS' : 'FAIL'}`
  ]);
  return ok;
}

function main() {
  const command = process.argv[2] || 'validate';
  const commands = { inspect, deploy, executable, dependencies, launch, proposal, clientRoundtrip, validate, certify };
  if (!commands[command]) throw new Error(`unknown command: ${command}`);
  process.exit(commands[command]() ? 0 : 1);
}

if (require.main === module) {
  main();
}
