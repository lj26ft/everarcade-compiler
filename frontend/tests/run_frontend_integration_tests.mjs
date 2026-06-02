import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync, existsSync } from "node:fs";

const rootFiles = [
  "frontend/creator-dashboard/src/App.tsx",
  "frontend/player-portal/src/App.tsx",
  "frontend/operator-console/src/App.tsx",
  "frontend/shared-api/src/index.ts",
  "frontend/shared-types/src/index.ts",
  "frontend/shared-wallet/src/index.ts",
  "frontend-gateway/routes.json",
  "docs/frontend/json_contracts.md",
  "frontend/tests/arena_vanguard_playable.test.ts",
  "frontend/tests/live_session.test.ts",
  "arena-vanguard-gateway/routes.json"
];

for (const file of rootFiles) {
  test(`frontend artifact exists: ${file}`, () => assert.equal(existsSync(file), true));
}

const flowNames = [
  "testDoctorFlow",
  "testProjectCreationFlow",
  "testRustrigInstallationFlow",
  "testPackageFlow",
  "testRehearsalFlow",
  "testDeploymentFlow",
  "testStatusFlow",
  "testWalletConnectFlow",
  "testArenaVanguardFlow",
  "testOperatorDashboardFlow"
];

test("declares all required frontend integration flows", () => {
  const body = readFileSync("frontend/tests/frontend_integration_tests.ts", "utf8");
  for (const name of flowNames) assert.match(body, new RegExp(`function ${name}`));
});

test("shared API consumes Product Command Facade commands", () => {
  const body = readFileSync("frontend/shared-api/src/index.ts", "utf8");
  for (const command of ["doctor", "new", "add-rustrig", "run", "package", "rehearse", "deploy", "validate", "status"]) {
    assert.match(body, new RegExp(`\\b${command}\\b`));
  }
  assert.match(body, /CliJsonProvider/);
  assert.match(body, /HttpProvider/);
  assert.match(body, /WebsocketProvider/);
});

test("frontend gateway exposes required endpoints", () => {
  const routes = JSON.parse(readFileSync("frontend-gateway/routes.json", "utf8"));
  for (const endpoint of ["/doctor", "/status", "/package", "/rehearse", "/deploy", "/validate", "/projects", "/rustrigs"]) {
    assert.ok(routes.endpoints[endpoint], endpoint);
  }
});

test("json contracts freeze required schemas", () => {
  const body = readFileSync("docs/frontend/json_contracts.md", "utf8");
  for (const contract of ["DoctorResult", "StatusResult", "PackageResult", "ValidationResult", "DeploymentResult"]) {
    assert.match(body, new RegExp(`## ${contract}`));
  }
});


test("arena vanguard frontend playable flow is wired", () => {
  const body = readFileSync("frontend/player-portal/src/App.tsx", "utf8");
  for (const token of ["Play Arena Vanguard", "Join Session", "Arena Vanguard HUD", "Movement Input", "Combat Input", "Inventory View", "Reconnect Flow"]) {
    assert.match(body, new RegExp(token));
  }
});

test("arena vanguard gateway exposes playable session endpoints", () => {
  const routes = JSON.parse(readFileSync("arena-vanguard-gateway/routes.json", "utf8"));
  for (const endpoint of ["/join", "/leave", "/move", "/attack", "/interact", "/status"]) {
    assert.ok(routes.endpoints[endpoint], endpoint);
  }
});


test("arena vanguard live session frontend validation is wired", () => {
  const body = readFileSync("frontend/tests/live_session.test.ts", "utf8");
  for (const token of ["Play Flow", "Join Flow", "Movement", "Combat", "Inventory", "Reconnect", "HUD Sync", "Session Resume"]) {
    assert.match(body, new RegExp(token));
  }
});

test("arena vanguard gateway exposes live session transport endpoints", () => {
  const routes = JSON.parse(readFileSync("arena-vanguard-gateway/routes.json", "utf8"));
  for (const endpoint of ["/join", "/leave", "/move", "/attack", "/interact", "/use-item", "/resume", "/heartbeat", "/world-state", "/status"]) {
    assert.ok(routes.endpoints[endpoint], endpoint);
  }
  assert.equal(routes.transport.resumeToken, true);
  assert.equal(routes.runtimeBinding.attachesRuntime, true);
});
