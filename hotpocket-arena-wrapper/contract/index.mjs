#!/usr/bin/env node
import { createServer } from 'node:http';
import { readFile } from 'node:fs/promises';
import { extname, join, normalize } from 'node:path';
import { ArenaHotPocketRuntime, defaultPaths, inputId } from '../src/runtime.mjs';

const runtimeRoot = process.env.EVERARCADE_REPO_ROOT || process.cwd();
const projectionRoot = join(runtimeRoot, 'runtime/games/arena-vanguard/projection');
const runtime = new ArenaHotPocketRuntime(defaultPaths(runtimeRoot)).load();
function send(res, code, value) { res.writeHead(code, { 'content-type': 'application/json', 'access-control-allow-origin': '*' }); res.end(`${JSON.stringify(value)}\n`); }
function contentType(file) { return { '.html': 'text/html; charset=utf-8', '.js': 'text/javascript; charset=utf-8', '.css': 'text/css; charset=utf-8' }[extname(file)] || 'application/octet-stream'; }
async function sendStatic(res, pathname) {
  const relative = pathname === '/projection' ? 'dashboard.html' : pathname.replace(/^\/projection\/?/, '') || 'dashboard.html';
  const safeRelative = normalize(relative).replace(/^(\.\.[/\\])+/, '');
  const file = join(projectionRoot, safeRelative);
  if (!file.startsWith(projectionRoot)) return send(res, 403, { status: 'failed', error: 'invalid projection path' });
  const body = await readFile(file);
  res.writeHead(200, { 'content-type': contentType(file), 'access-control-allow-origin': '*' });
  res.end(body);
}
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
    if (url.pathname === '/projection' || url.pathname.startsWith('/projection/')) return sendStatic(res, url.pathname);
    return send(res, 404, { status: 'failed', error: 'unknown hotpocket arena endpoint' });
  } catch (error) { return send(res, 400, { status: 'rejected', error: error.message }); }
});
if (import.meta.url === `file://${process.argv[1]}`) server.listen(Number(process.env.PORT || 8787), () => console.log('hotpocket-arena-wrapper listening'));
