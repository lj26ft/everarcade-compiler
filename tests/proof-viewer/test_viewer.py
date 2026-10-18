import copy
import hashlib
from html.parser import HTMLParser
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location('viewer', ROOT / 'tools/proof-viewer/viewer.py')
viewer = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(viewer)
FIXTURE = ROOT / 'test-vectors/commitments/receipt-mmr-v1.json'


class Tags(HTMLParser):
    def __init__(self):
        super().__init__()
        self.tags = []
        self.text = []

    def handle_starttag(self, tag, attrs):
        self.tags.append((tag, dict(attrs)))

    def handle_data(self, data):
        self.text.append(data)


class ViewerTests(unittest.TestCase):
    def test_all_positive_empty_and_descriptive_negative_fixtures(self):
        raw = FIXTURE.read_text()
        page, status = viewer.render(raw)
        self.assertTrue(status.startswith('UNVERIFIED:'))
        self.assertEqual(page.count('DESCRIPTIVE NEGATIVE ONLY'), 8)
        for row in json.loads(raw)['vectors']:
            self.assertIn(row['name'], page)
        self.assertIn('empty-receipt-log', page)
        self.assertIn('No proof verification', page)

    def test_proof_fields_are_faithful_without_mutation(self):
        proof = json.loads(FIXTURE.read_text())['vectors'][1]['proofs'][0]
        before = copy.deepcopy(proof)
        parser = Tags()
        parser.feed(viewer.render(json.dumps(proof))[0])
        text = ''.join(parser.text)
        for field in viewer.FIELDS:
            self.assertIn(field, text)
            self.assertIn(json.dumps(proof[field], ensure_ascii=True, indent=2), text)
        self.assertEqual(proof, before)

    def test_untrusted_text_never_creates_html_or_access(self):
        attack = '</pre><script>alert(1)</script><img src="https://evil.invalid/x">$(touch /tmp/owned)'
        raw = json.dumps({'version': 'unknown', 'path': '/etc/passwd', 'text': attack})
        with patch('builtins.open', side_effect=AssertionError('unexpected file read')), patch('socket.socket', side_effect=AssertionError('unexpected network')):
            page, status = viewer.render(raw)
        parser = Tags()
        parser.feed(page)
        self.assertIn(raw, ''.join(parser.text))
        self.assertFalse(any(t in ('script', 'img', 'iframe', 'a') for t, _ in parser.tags))
        self.assertIn('REFUSED', status)
        self.assertIn("default-src &#x27;none&#x27;", page.replace("'", '&#x27;'))

    def test_malformed_unsupported_and_duplicate_data_stay_visible(self):
        for raw in ('{bad', '[]', '{"version":"future"}', '{"a":1,"a":2}', '{"a":NaN}', '{"version":"world.evr.receipt_mmr_vectors.v1","vectors":[null]}'):
            with self.subTest(raw=raw):
                page, status = viewer.render(raw)
                self.assertIn('REFUSED', status)
                parser = Tags()
                parser.feed(page)
                self.assertIn(raw, ''.join(parser.text))

    def test_missing_or_wrong_type_fields_never_claim_validation(self):
        raw = '{"version":"world.evr.receipt_proof.v1","commitment_profile":"world.evr.commitment.v1","receipt_count":"wrong"}'
        page, status = viewer.render(raw)
        self.assertIn('UNVERIFIED', status)
        self.assertIn('fields have not been validated', page)
        self.assertIn('MISSING', page)

    def test_exponent_overflow_is_refused_without_changing_input(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / 'overflow.json'
            for number in ('1e999', '-1e999'):
                raw = ('{"version":"world.evr.receipt_proof.v1",'
                       '"commitment_profile":"world.evr.commitment.v1",'
                       '"receipt_count":' + number + '}')
                path.write_bytes(raw.encode())
                result = subprocess.run([sys.executable, str(SPEC.origin), str(path)], capture_output=True)
                self.assertEqual(result.returncode, 1)
                self.assertIn(b'REFUSED', result.stderr)
                parser = Tags()
                parser.feed(result.stdout.decode())
                self.assertIn(raw, ''.join(parser.text))
                self.assertNotIn('Infinity', result.stdout.decode())
                self.assertEqual(path.read_bytes(), raw.encode())

    def test_cli_never_changes_input_and_exit_codes(self):
        before = hashlib.sha256(FIXTURE.read_bytes()).digest()
        result = subprocess.run([sys.executable, str(SPEC.origin), str(FIXTURE)], capture_output=True)
        self.assertEqual(result.returncode, 0)
        self.assertIn(b'UNVERIFIED', result.stderr)
        self.assertEqual(hashlib.sha256(FIXTURE.read_bytes()).digest(), before)
        with tempfile.TemporaryDirectory() as temp:
            path = Path(temp) / 'input'
            for data, expected in ((b'{bad', 1), (b'\xff', 2), (b'x' * (viewer.MAX_BYTES + 1), 2)):
                path.write_bytes(data)
                result = subprocess.run([sys.executable, str(SPEC.origin), str(path)], capture_output=True)
                self.assertEqual(result.returncode, expected)
                self.assertEqual(path.read_bytes(), data)
            result = subprocess.run([sys.executable, str(SPEC.origin), str(path.parent / 'absent')], capture_output=True)
            self.assertEqual(result.returncode, 2)


if __name__ == '__main__':
    unittest.main()
