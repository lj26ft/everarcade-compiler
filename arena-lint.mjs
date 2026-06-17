#!/usr/bin/env node
import { readFileSync } from 'node:fs';
const file = 'hotpocket-arena-wrapper/contract/hotpocket-adapter.mjs';
const source = readFileSync(file, 'utf8');
const findings = [];
if (!/new\s+HotPocket\.Contract\s*\(\s*\)/.test(source) || !/\.init\s*\(\s*async\s*\(\s*ctx\s*\)/.test(source)) findings.push('missing-init');
if (!/ctx\.users/.test(source) || !/ctx\.lclSeqNo/.test(source) || !/ctx\.npl/.test(source)) findings.push('ignores-ctx');
for (const forbidden of ['Date.now', 'new Date', 'Math.random', 'randomUUID', 'fetch(', 'axios', 'setTimeout', 'setInterval', 'process.env', 'process.cwd']) if (source.includes(forbidden)) findings.push(`forbidden:${forbidden}`);
if (findings.length) {
  console.log(`Arena lint: FAIL ${findings.join(', ')}`);
  process.exit(1);
}
console.log('Arena lint: PASS');
