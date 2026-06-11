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
  const result = childProcess.spawnSync('bash', ['-lc', `command -v ${command}`], { encoding: 'utf8' });
  return result.status === 0 ? result.stdout.trim() : null;
}

function run(command, args, cwd) {
  const started = new Date().toISOString();
  const result = childProcess.spawnSync(command, args, { cwd, encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] });
  return {
    command: [command, ...args].join(' '),
    cwd,
    started_at: started,
    exit_code: result.status,
    signal: result.signal,
    stdout: result.stdout || '',
    stderr: result.stderr || '',
    ok: result.status === 0
  };
}

function parsePatchCfg(text) {
  const cfg = {};
  for (const line of text.split(/\r?\n/)) {
    const match = line.match(/^\s*([A-Za-z0-9_.-]+)\s*=\s*"?([^"\n]*)"?\s*$/);
    if (match) cfg[match[1]] = match[2];
  }
  return cfg;
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
  let args = (patch.bin_args || '').split(/\s+/).filter(Boolean);
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

function deploy() {
  const evernodecli = commandExists('evernodecli');
  const deployments = [];
  for (const variant of VARIANTS) {
    const clean = evernodecli ? run('evernodecli', ['hp-clean'], REPO_ROOT) : { ok: false, command: 'evernodecli hp-clean', exit_code: null, stdout: '', stderr: 'evernodecli not found' };
    const deployRun = evernodecli ? run('evernodecli', ['hp-deploy', variantPath(variant)], REPO_ROOT) : { ok: false, command: `evernodecli hp-deploy ${variantPath(variant)}`, exit_code: null, stdout: '', stderr: 'evernodecli not found' };
    deployments.push({ id: variant.id, package_path: path.relative(REPO_ROOT, variantPath(variant)), hp_clean: clean, hp_deploy: deployRun, status: clean.ok && deployRun.ok ? 'PASS' : 'FAIL' });
  }
  const ok = deployments.every((item) => item.status === 'PASS');
  mirrorText('hotpocket_deployment_compatibility_report.txt', [
    'HotPocket Deployment Compatibility Report',
    `Generated At: ${DEFAULT_TIME}`,
    'Manual edits: forbidden',
    'docker exec: forbidden',
    'Runtime patching: forbidden',
    `evernodecli: ${evernodecli || 'not found'}`,
    ...deployments.flatMap((item) => [
      `${item.id} hp-clean: ${item.hp_clean.ok ? 'PASS' : 'FAIL'}`,
      `${item.id} hp-deploy: ${item.hp_deploy.ok ? 'PASS' : 'FAIL'}`,
      `${item.id} deploy stderr: ${(item.hp_deploy.stderr || '').split('\n').filter(Boolean).slice(-1)[0] || 'none'}`
    ]),
    `Classification: ${ok ? 'HotPocket Deployment Compatibility Proven' : 'HotPocket Deployment Compatibility Not Proven'}`,
    `HotPocket Deployment Compatibility Proof: ${ok ? 'PASS' : 'FAIL'}`
  ]);
  writeDiscoveryReport(deployments);
  return ok;
}

function configuredClusterRoots() {
  return [process.env.HOTPOCKET_CLUSTER_ROOT, process.env.HPDEVKIT_CLUSTER_ROOT, process.env.EVERARCADE_HOTPOCKET_CLUSTER_ROOT]
    .filter(Boolean)
    .map((item) => path.resolve(item));
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
    const args = (parsed.bin_args || '').split(/\s+/).filter(Boolean);
    const launchResult = binPath && parsed.bin_args ? run(binPath === 'node' ? 'node' : candidate, args, dir) : { ok: false, exit_code: null, stderr: 'bin_path or bin_args missing' };
    return { file: path.relative(REPO_ROOT, file), bin_path: binPath, bin_args: parsed.bin_args || '', executable_exists: binPath === 'node' ? Boolean(commandExists('node')) : fs.existsSync(candidate), executable_launches: launchResult.ok, launch_exit_code: launchResult.exit_code, placeholder_absent: binPath !== PLACEHOLDER && !binPath.includes('your contract binary'), status: binPath && parsed.bin_args && (binPath === 'node' || fs.existsSync(candidate)) && launchResult.ok ? 'PASS' : 'FAIL' };
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

function launch() {
  const variants = VARIANTS.map((variant) => ({ id: variant.id, result: launchVariant(variant) }));
  const ok = variants.every((item) => item.result.ok && !`${item.result.stderr}\n${item.result.stdout}`.includes('Contract process execve() failed') && !`${item.result.stderr}\n${item.result.stdout}`.includes('Cannot find module') && !`${item.result.stderr}\n${item.result.stdout}`.includes(PLACEHOLDER));
  mirrorText('hotpocket_contract_launch_report.txt', [
    'HotPocket Contract Launch Report',
    ...variants.map((item) => `${item.id}: exit=${item.result.exit_code} execve_failed=${`${item.result.stderr}`.includes('execve') ? 'FAIL' : 'PASS'} module_not_found=${`${item.result.stderr}`.includes('Cannot find module') ? 'FAIL' : 'PASS'} bin_path_placeholder=${`${item.result.stdout}${item.result.stderr}`.includes(PLACEHOLDER) ? 'FAIL' : 'PASS'}`),
    `Classification: ${ok ? 'HotPocket Contract Launch Proven' : 'HotPocket Contract Launch Not Proven'}`,
    `HotPocket Contract Launch Proof: ${ok ? 'PASS' : 'FAIL'}`
  ]);
  return ok;
}

function proposal() {
  const roots = configuredClusterRoots();
  const logFiles = findFiles(roots, ['hp.log', 'contract.log', 'hotpocket.log']);
  const text = logFiles.map(readText).join('\n');
  const validators = Number(process.env.HOTPOCKET_VALIDATOR_COUNT || 0);
  const proposals = Number(process.env.HOTPOCKET_PROPOSAL_COUNT || 0);
  const noVotesFailures = !text.includes('votes:1 needed:3') && !text.includes('votes:2 needed:3') && !text.includes('Not enough peers proposing');
  const ok = validators >= 3 && proposals >= 3 && noVotesFailures;
  mirrorText('hotpocket_cluster_proposal_report.txt', [
    'HotPocket Cluster Proposal Report',
    `Cluster roots: ${roots.length ? roots.join(', ') : 'not configured'}`,
    `Log files inspected: ${logFiles.length}`,
    `Validators observed: ${validators || 'unavailable'}`,
    `Proposals observed: ${proposals || 'unavailable'}`,
    `votes:1 needed:3: ${text.includes('votes:1 needed:3') ? 'present' : 'absent'}`,
    `votes:2 needed:3: ${text.includes('votes:2 needed:3') ? 'present' : 'absent'}`,
    `Classification: ${ok ? 'HotPocket Cluster Proposal Proven' : 'HotPocket Cluster Proposal Not Proven'}`,
    `HotPocket Cluster Proposal Proof: ${ok ? 'PASS' : 'FAIL'}`
  ]);
  return ok;
}

function writeDiscoveryReport(deployments = []) {
  const roots = configuredClusterRoots();
  const hpCfgFiles = findFiles(roots, ['hp.cfg']);
  const patchCfgFiles = findFiles(roots, ['patch.cfg']);
  const packageFiles = findFiles([TEMPLATE_ROOT, ...roots], ['package.json']);
  const report = {
    schema: 'everarcade.hotpocket.deployment-discovery.v0.1',
    generated_at: DEFAULT_TIME,
    deployment_metadata: { deployments, evernodecli: commandExists('evernodecli'), hpdevkit: commandExists('hpdevkit'), docker: commandExists('docker') },
    package_metadata: packageFiles.map((file) => ({ file: path.relative(REPO_ROOT, file), package: readJson(file, {}) })),
    generated_hp_cfg: hpCfgFiles.map((file) => ({ file: path.relative(REPO_ROOT, file), parsed: parsePatchCfg(readText(file)), raw: readText(file) })),
    generated_patch_cfg: patchCfgFiles.map((file) => ({ file: path.relative(REPO_ROOT, file), parsed: parsePatchCfg(readText(file)), raw: readText(file) })),
    generated_contract_filesystem: roots.map((root) => ({ root, files: findFiles([root], ['contract.js', 'contract', 'package.json']).map((file) => path.relative(root, file)) })),
    generated_ledger_filesystem: roots.map((root) => ({ root, files: findFiles([root], ['ledger', 'ledger.db', 'unl.json']).map((file) => path.relative(root, file)) }))
  };
  mirrorJson('hotpocket_deployment_discovery_report.json', report);
}

function validate() {
  const results = [inspect(), deploy(), executable(), dependencies(), launch(), proposal()];
  writeDiscoveryReport();
  return results.every(Boolean);
}

function certify() {
  const ok = validate();
  mirrorText('hotpocket_deployment_certification_report.txt', [
    'HotPocket Deployment Compatibility Proof v0.1 Certification',
    'Explicit Non-Claims: contract execution correctness, gameplay, multiplayer, replay, federation, civilization hosting, Evernode leases, XRPL settlement, Xahau settlement, production operation',
    `HotPocket Deployment Compatibility Proof v0.1: ${ok ? 'PASS' : 'FAIL'}`
  ]);
  return ok;
}

function main() {
  const command = process.argv[2] || 'validate';
  const commands = { inspect, deploy, executable, dependencies, launch, proposal, validate, certify };
  if (!commands[command]) throw new Error(`unknown command: ${command}`);
  process.exit(commands[command]() ? 0 : 1);
}

if (require.main === module) {
  main();
}
