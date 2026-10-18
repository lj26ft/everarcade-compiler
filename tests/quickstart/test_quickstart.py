"""Integration checks use the real CLI; native checks require the pinned toolchain."""
import importlib.util
import json
from pathlib import Path
import subprocess
import tempfile
import unittest

REPO = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location('quickstart_smoke', REPO / 'scripts/quickstart/smoke.py')
SMOKE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(SMOKE)


class QuickstartTests(unittest.TestCase):
    def test_real_source_path_rejects_missing_entry(self):
        with tempfile.TemporaryDirectory(prefix='quickstart-test-') as directory:
            project = Path(directory) / 'arena'
            cli = ['node', str(REPO / 'creator-sdk/cli/everarcade.mjs')]
            for args in [
                ['new', '--template', 'arena', '--name', 'arena-demo', '--dir', str(project)],
                ['build', '--project', str(project)],
                ['test', '--project', str(project)],
            ]:
                result = subprocess.run(cli + args, cwd=REPO, capture_output=True, text=True, timeout=30)
                self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            manifest = json.loads((project / 'everarcade.game.json').read_text())
            (project / manifest['entry']).unlink()
            result = subprocess.run(cli + ['test', '--project', str(project)], cwd=REPO, capture_output=True, text=True, timeout=30)
            self.assertNotEqual(result.returncode, 0, 'broken generated project must fail the documented CLI test')

    def test_replay_assertion_rejects_changed_root(self):
        with tempfile.TemporaryDirectory(prefix='quickstart-replay-') as directory:
            root = Path(directory)
            (root / 'replay').mkdir()
            (root / 'replay/gameplay-replay-proof.json').write_text(json.dumps({
                'replay_verification': 'PASS', 'session_id': 'session-0001',
                'replay_root': 'changed', 'state_root': 'original',
            }))
            with self.assertRaisesRegex(RuntimeError, 'root mismatch'):
                SMOKE.check_replay(root)

    def test_package_assertion_rejects_placeholder_bytes(self):
        with tempfile.TemporaryDirectory(prefix='quickstart-package-') as directory:
            project = Path(directory)
            package = project / 'dist/runtime-package'
            package.mkdir(parents=True)
            (package / 'manifest.json').write_text('{}')
            (package / 'world.json').write_text('{}')
            (package / 'world.wasm').write_bytes(b'{"placeholder":true}')
            with self.assertRaisesRegex(RuntimeError, 'not compiled Wasm'):
                SMOKE.check_package(project)


if __name__ == '__main__':
    unittest.main()
