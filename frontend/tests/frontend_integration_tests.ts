export const requiredFlows = [
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

export function testDoctorFlow() { return { command: "everarcade doctor --json", expected: "DoctorResult" }; }
export function testProjectCreationFlow() { return { command: "everarcade new arena-vanguard --json", expected: "ProjectResult" }; }
export function testRustrigInstallationFlow() { return { command: "everarcade add-rustrig combat --json", expected: "RustrigResult" }; }
export function testPackageFlow() { return { command: "everarcade package --json", expected: "PackageResult" }; }
export function testRehearsalFlow() { return { command: "everarcade rehearse --json", expected: "RehearsalResult" }; }
export function testDeploymentFlow() { return { command: "everarcade deploy --dry-run --json", expected: "DeploymentResult" }; }
export function testStatusFlow() { return { command: "everarcade status --json", expected: "StatusResult" }; }
export function testWalletConnectFlow() { return { provider: "xaman", actions: ["Connect", "Disconnect", "Display Address", "Display Network"] }; }
export function testArenaVanguardFlow() { return { portal: "Arena Vanguard", surfaces: ["Play", "Profile", "Inventory", "Progression", "Leaderboards", "World Status"] }; }
export function testOperatorDashboardFlow() { return { statusSource: "everarcade status --json", panels: ["Runtime Health", "Replay Health", "Deployment Status", "Federation Health", "Alerts", "Metrics"] }; }
