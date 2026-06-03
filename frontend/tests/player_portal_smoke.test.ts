import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";

const indexHtml = readFileSync("frontend/player-portal/index.html", "utf8");
const app = readFileSync("frontend/player-portal/src/App.tsx", "utf8");
const sharedApi = readFileSync("frontend/shared-api/src/index.ts", "utf8");
const mainEntry = "/src/main.tsx";

test("player portal index.html loads the React main entrypoint", () => {
  assert.match(indexHtml, /^<!DOCTYPE html>/);
  assert.match(indexHtml, /<div id="root"><\/div>/);
  assert.match(indexHtml, /src="\/src\/main\.tsx"/);
  assert.equal(mainEntry, "/src/main.tsx");
  assert.doesNotMatch(indexHtml, /App\.tsx/);
});

test("player portal renders visible smoke UI instead of a blank page", () => {
  for (const token of [
    "🎮 EverArcade Player Portal",
    "Arena Vanguard",
    "Runtime Status",
    "Gateway Status",
    "WebSocket Status",
    "Play Arena Vanguard",
    "🟡 Waiting for runtime"
  ]) {
    assert.match(app, new RegExp(token));
  }
});

test("arena vanguard first-run browser card is visible", () => {
  for (const token of ["Game Card", "Play Arena Vanguard", "Connection Status", "Session Status", "HUD Preview", "Controls Help"]) {
    assert.match(app, new RegExp(token));
  }
});

test("arena vanguard play flow surfaces runtime connection stages and failure hint", () => {
  for (const token of [
    "🔌 Connecting to gateway",
    "🎮 Joining session",
    "🧍 Character spawned",
    "🌍 World feed active",
    "🟢 Ready",
    "🔴 Could not connect to runtime",
    "everarcade run arena-vanguard"
  ]) {
    assert.match(app, new RegExp(token));
  }
});

test("runtime websocket URL follows the current browser host", () => {
  assert.match(sharedApi, /locationLike\.hostname/);
  assert.match(sharedApi, /DEFAULT_ARENA_VANGUARD_GATEWAY_PORT = "8791"/);
  assert.match(sharedApi, /\/runtime-feed/);
});
