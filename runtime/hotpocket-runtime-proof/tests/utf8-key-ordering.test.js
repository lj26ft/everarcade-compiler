'use strict';

const assert = require('assert/strict');
const fs = require('fs');
const path = require('path');
const test = require('node:test');

const adapter = require('../adapter/runtime-adapter');

const fixturePath = path.resolve(__dirname, '../../../docs/proofs/fixtures/utf8-key-ordering.json');

test('dynamic object keys are ordered by raw UTF-8 bytes instead of locale collation', () => {
  const fixture = JSON.parse(fs.readFileSync(fixturePath, 'utf8'));
  const extensions = fixture.input.metadata.extensions;

  assert.deepEqual(Object.keys(extensions).sort(adapter.compareUtf8Bytes), fixture.expected_dynamic_key_order);
  assert.equal(adapter.canonicalize(extensions), fixture.expected_canonical_extensions_json);

  const utf16Sorted = Object.keys(extensions).sort();
  assert.notDeepEqual(utf16Sorted, fixture.expected_dynamic_key_order);
});
