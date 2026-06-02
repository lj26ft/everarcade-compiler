import { createServer } from "node:http";
import { spawn } from "node:child_process";

const endpointCommands = {
  "/doctor": (body) => ["doctor"],
  "/status": (body) => ["status"],
  "/package": (body) => ["package"],
  "/rehearse": (body) => ["rehearse"],
  "/deploy": (body) => ["deploy", ...(body?.dryRun === false ? [] : ["--dry-run"])],
  "/validate": (body) => ["validate", "--profile", body?.profile ?? "quick"],
  "/projects": (body) => ["new", body?.gameId ?? "arena-vanguard"],
  "/rustrigs": (body) => ["add-rustrig", body?.name ?? "combat"]
};

function readBody(request) {
  return new Promise((resolve) => {
    let data = "";
    request.on("data", chunk => { data += chunk; });
    request.on("end", () => {
      try { resolve(data ? JSON.parse(data) : {}); } catch { resolve({}); }
    });
  });
}

function runEverArcade(args) {
  return new Promise((resolve) => {
    const child = spawn("everarcade", [...args, "--json"], { stdio: ["ignore", "pipe", "pipe"] });
    let stdout = "";
    let stderr = "";
    child.stdout.on("data", chunk => { stdout += chunk; });
    child.stderr.on("data", chunk => { stderr += chunk; });
    child.on("close", code => resolve({ code, stdout, stderr }));
  });
}

export const server = createServer(async (request, response) => {
  const url = new URL(request.url ?? "/", "http://localhost");
  const commandFactory = endpointCommands[url.pathname];
  if (!commandFactory) {
    response.writeHead(404, { "content-type": "application/json" });
    response.end(JSON.stringify({ status: "failed", error: "unknown endpoint" }));
    return;
  }
  const body = await readBody(request);
  const result = await runEverArcade(commandFactory(body));
  response.writeHead(result.code === 0 ? 200 : 500, { "content-type": "application/json" });
  response.end(result.stdout || JSON.stringify({ status: "failed", error: result.stderr.trim() }));
});

if (import.meta.url === `file://${process.argv[1]}`) {
  server.listen(Number(process.env.PORT ?? 8787), () => console.log("frontend-gateway listening"));
}
