#!/usr/bin/env python3
"""Exercise the actual Creator SDK in a fresh disposable directory."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import signal
import subprocess
import tempfile


def require(condition, message):
    if not condition:
        raise RuntimeError(message)


def check_package(project):
    package = project / 'dist/runtime-package'
    manifest = json.loads((package / 'manifest.json').read_text())
    world = json.loads((package / 'world.json').read_text())
    wasm = (package / 'world.wasm').read_bytes()
    require(wasm[:4] == b'\0asm', 'package is not compiled Wasm')
    require(hashlib.sha256(wasm).hexdigest() == manifest['wasm_hash'], 'package Wasm digest mismatch')
    require(world['package_classification'] == 'wasm-guest-runtime-package', 'unexpected package classification')


def check_replay(runtime):
    replay = json.loads((runtime / 'replay/gameplay-replay-proof.json').read_text())
    require(replay['replay_verification'] == 'PASS', 'replay verification failed')
    require(replay['session_id'] == 'session-0001', 'unexpected session')
    require(replay.get('replay_root') and replay['replay_root'] == replay['state_root'], 'replay root mismatch')
    for relative in ['sessions/session-0001.json', 'gameplay/arena-state.json', 'journals/journal.jsonl']:
        require((runtime / relative).is_file(), f'missing runtime evidence: {relative}')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source-only', action='store_true', help='new/build/test only; explicitly NOT a native gate')
    parser.add_argument('--output-parent', type=Path, default=Path(tempfile.gettempdir()), help='existing directory OUTSIDE the checkout for fresh retained evidence/project/runtime')
    args = parser.parse_args()
    repo = Path(__file__).resolve().parents[2]
    parent = args.output_parent.resolve()
    require(parent.is_dir(), 'output parent must exist')
    require(not parent.is_relative_to(repo), 'output parent must be outside the checkout')
    output = Path(tempfile.mkdtemp(prefix='everarcade-quickstart-', dir=parent))
    project, runtime = output / 'arena-demo', output / 'runtime-root'
    result = {'mode': 'source-only' if args.source_only else 'native', 'steps': [], 'native': 'NOT_RUN', 'output': str(output)}
    print(f'Evidence: {output}', flush=True)
    env = {**os.environ, 'CARGO_NET_OFFLINE': 'true', 'CARGO_BUILD_JOBS': '1'}

    def run(label, command, timeout=120):
        record = {'step': label, 'command': command, 'exit': None}
        result['steps'].append(record)
        try:
            step_env = dict(env)
            if label == 'play-local':
                step_env['CARGO_TARGET_DIR'] = str(output / 'cargo-target')
            else:
                step_env.pop('CARGO_TARGET_DIR', None)  # upstream guest builder reads its fixed target path
            proc = subprocess.Popen(command, cwd=repo, env=step_env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, start_new_session=True)
            try:
                text, _ = proc.communicate(timeout=timeout)
            except subprocess.TimeoutExpired:
                os.killpg(proc.pid, signal.SIGKILL)
                text, _ = proc.communicate()
                record['exit'] = proc.returncode
                record['timed_out'] = True
                (output / f'{label}.log').write_text(text + f'\nTIMEOUT after {timeout}s\n')
                raise RuntimeError(f'{label}: timeout after {timeout}s')
            record['exit'] = proc.returncode
        except OSError as error:
            (output / f'{label}.log').write_text(str(error))
            raise RuntimeError(f'{label}: {error}') from error
        (output / f'{label}.log').write_text(text)
        print(f'{label}: exit {proc.returncode}', flush=True)
        require(proc.returncode == 0, f'{label} failed; see {output / (label + ".log")}')
        return text.strip()

    try:
        node = run('node-version', ['node', '--version'])
        expected_node = 'v' + (repo / '.nvmrc').read_text().strip()
        result['node_pin_match'] = node == expected_node
        if not args.source_only:
            require(node == expected_node, f'unsupported toolchain: expected Node {expected_node}, got {node}')
            pin = re.search(r'channel\s*=\s*"([^"]+)"', (repo / 'rust-toolchain.toml').read_text())[1]
            for tool in ['rustc', 'cargo']:
                version = run(f'{tool}-version', [tool, '--version'])
                require(version.split()[1] == pin, f'unsupported toolchain: expected {tool} {pin}, got {version}')
            targets = run('wasm-target', ['rustup', 'target', 'list', '--installed'])
            require('wasm32-unknown-unknown' in targets.splitlines(), 'install the pinned wasm32-unknown-unknown target before this offline smoke')
            require(not Path('/tmp/everarcade-guest-build/arena-proof-contract').exists(), 'upstream package scratch already exists; preserve it and use an isolated environment')
            run('vendor', ['bash', 'scripts/ensure_vendor_offline.sh'], 600)
        cli = ['node', 'creator-sdk/cli/everarcade.mjs']
        run('new', cli + ['new', '--template', 'arena', '--name', 'arena-demo', '--dir', str(project)])
        run('build', cli + ['build', '--project', str(project)])
        run('test', cli + ['test', '--project', str(project)])
        require((project / 'dist/build.json').is_file(), 'build metadata missing')
        if not args.source_only:
            run('package', cli + ['package', '--project', str(project)], 600)
            check_package(project)
            result['native'] = 'FAIL'  # attempted; PASS only after runtime and replay checks
            run('play-local', cli + ['play-local', '--project', str(project), '--template', 'arena', '--runtime-root', str(runtime)], 1200)
            check_replay(runtime)
            result['native'] = 'PASS'
        result['status'] = 'PASS'
        print('Source scaffold: PASS; package/native/replay: NOT_RUN' if args.source_only else 'Bounded native local path: PASS', flush=True)
        return 0
    except (RuntimeError, KeyError, ValueError, OSError) as error:
        result['status'] = 'FAIL'
        result['error'] = str(error)
        print(f'Quickstart: FAIL — {error}', flush=True)
        return 1
    finally:
        (output / 'result.json').write_text(json.dumps(result, indent=2) + '\n')
        files = sorted(path for path in output.rglob('*') if path.is_file() and 'cargo-target' not in path.relative_to(output).parts)
        (output / 'SHA256SUMS').write_text(''.join(f'{hashlib.sha256(path.read_bytes()).hexdigest()}  {path.relative_to(output)}\n' for path in files))


if __name__ == '__main__':
    raise SystemExit(main())
