#!/usr/bin/env node
import { createServer } from 'node:http';
import { ArenaHotPocketRuntime, defaultPaths, inputId } from '../src/runtime.mjs';

const runtime = new ArenaHotPocketRuntime(defaultPaths(process.env.EVERARCADE_REPO_ROOT || process.cwd())).load();
function send(res, code, value) { res.writeHead(code, { 'content-type': 'application/json', 'access-control-allow-origin': '*' }); res.end(`${JSON.stringify(value)}\n`); }
function readBody(req) { return new Promise((resolve) => { let data = ''; req.on('data', (chunk) => { data += chunk; }); req.on('end', () => { try { resolve(data ? JSON.parse(data) : {}); } catch (error) { resolve({ __parse_error: error.message }); } }); }); }
async function handleInput(body) {
  if (body.__parse_error) throw new Error(body.__parse_error);
  const result = runtime.process(body);
  return { status: 'accepted', input_id: inputId(result.journal.action), receipt: result.receipt, output: result.output };
}
export const server = createServer(async (req, res) => {
  const url = new URL(req.url || '/', 'http://localhost');
  if (req.method === 'OPTIONS') return send(res, 204, {});
  try {
    if (url.pathname === '/input' || url.pathname === '/action') return send(res, 200, await handleInput(await readBody(req)));
    if (url.pathname === '/state' || url.pathname === '/world-state') return send(res, 200, runtime.snapshot());
    if (url.pathname === '/journal') return send(res, 200, runtime.journal);
    if (url.pathname === '/verify') return send(res, 200, runtime.verify());
    if (url.pathname === '/health') return send(res, 200, { status: 'healthy', authority: 'hotpocket-contract-wrapper', tick: runtime.state.tick });
    return send(res, 404, { status: 'failed', error: 'unknown hotpocket arena endpoint' });
  } catch (error) { return send(res, 400, { status: 'rejected', error: error.message }); }
});
if (import.meta.url === `file://${process.argv[1]}`) server.listen(Number(process.env.PORT || 8787), () => console.log('hotpocket-arena-wrapper listening'));
