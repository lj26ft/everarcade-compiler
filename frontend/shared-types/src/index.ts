export type HealthState = "healthy" | "warning" | "failed" | "ready" | "passed" | "created" | "complete" | "running";

export interface CheckResult {
  name: string;
  status: HealthState | string;
  emoji: string;
  suggested_fix?: string | null;
}

export interface DoctorResult {
  command: "doctor";
  status: "ready" | "failed";
  checks: CheckResult[];
}

export interface RuntimeStatus {
  state: "healthy" | "warning" | "failed" | "running" | string;
  ticks_per_sec: number;
  runtime_count: number;
  checkpoint_age_seconds: number;
}

export interface FederationStatus {
  state: "healthy" | "warning" | "failed" | string;
  nodes: number;
}

export interface ReplayStatus {
  state: "healthy" | "warning" | "failed" | string;
  growth_bytes_per_minute: number;
  latest_checkpoint: string;
}

export interface StatusResult {
  command: "status";
  runtime: RuntimeStatus | string;
  replay: ReplayStatus | string;
  deployment: "ready" | "warning" | "failed" | string;
  federation: FederationStatus | string;
  metrics: {
    mode: "scaffold" | "runtime" | string;
    deterministic: boolean;
    ticks_per_sec?: number;
    checkpoint_age_seconds?: number;
    replay_growth_bytes_per_minute?: number;
    runtime_count?: number;
    deployment_count?: number;
    federation_nodes?: number;
  };
}

export interface PackageResult {
  command: "package";
  status: "complete" | "failed" | string;
  runtime_package: string;
  world_package: string;
  deployment_package: string;
  checksums: "verified" | "failed" | string;
}

export interface ValidationResult {
  command: "validate";
  profile: "quick" | "rustrigs" | "evernode" | "full" | string;
  status: "passed" | "failed" | string;
  checks: CheckResult[];
}

export interface DeploymentResult {
  command: "deploy";
  mode: "dry-run" | "stage-contract" | string;
  status: "ready" | "failed" | string;
  live_evernode: "not-implemented" | "ready" | string;
}

export interface ProjectResult {
  command: "new";
  status: "created" | string;
  game_id: string;
  path: string;
}

export interface RustrigResult {
  command: "add-rustrig";
  status: "updated" | string;
  game_id: string;
  rustrig: string;
}

export interface WalletConnection {
  provider: "xaman" | "xrpl" | "xahau-future";
  address: string;
  network: "xrpl-mainnet" | "xrpl-testnet" | "xahau-future" | string;
  connected: boolean;
}
