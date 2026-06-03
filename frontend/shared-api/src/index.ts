import type { DeploymentResult, DoctorResult, PackageResult, ProjectResult, RustrigResult, StatusResult, ValidationResult } from "@everarcade/shared-types";

export type ProductCommand = "doctor" | "new" | "add-rustrig" | "run" | "package" | "rehearse" | "deploy" | "validate" | "status" | "session-status";

export interface CommandRequest {
  command: ProductCommand;
  args?: string[];
}

export interface ProductFacadeProvider {
  execute<T>(request: CommandRequest): Promise<T>;
}

export class CliJsonProvider implements ProductFacadeProvider {
  constructor(private readonly invoke: (command: string, args: string[]) => Promise<unknown>) {}

  execute<T>(request: CommandRequest): Promise<T> {
    return this.invoke("everarcade", [request.command, ...(request.args ?? []), "--json"]) as Promise<T>;
  }
}

export class HttpProvider implements ProductFacadeProvider {
  constructor(private readonly baseUrl: string) {}

  async execute<T>(request: CommandRequest): Promise<T> {
    const response = await fetch(`${this.baseUrl}/${request.command}`, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ args: request.args ?? [] })
    });
    if (!response.ok) throw new Error(`frontend gateway failed: ${response.status}`);
    return response.json() as Promise<T>;
  }
}

export class WebsocketProvider implements ProductFacadeProvider {
  constructor(private readonly sendJson: (payload: CommandRequest) => Promise<unknown>) {}
  execute<T>(request: CommandRequest): Promise<T> {
    return this.sendJson(request) as Promise<T>;
  }
}

export const doctor = (provider: ProductFacadeProvider) => provider.execute<DoctorResult>({ command: "doctor" });
export const createProject = (provider: ProductFacadeProvider, gameId: string, template: string): Promise<ProjectResult> =>
  provider.execute<ProjectResult>({ command: "new", args: [gameId, "--template", template] });
export const addRustrig = (provider: ProductFacadeProvider, name: string): Promise<RustrigResult> =>
  provider.execute<RustrigResult>({ command: "add-rustrig", args: [name] });
export const packageGame = (provider: ProductFacadeProvider): Promise<PackageResult> => provider.execute<PackageResult>({ command: "package" });
export const rehearseGame = (provider: ProductFacadeProvider) => provider.execute({ command: "rehearse" });
export const deployGame = (provider: ProductFacadeProvider, dryRun = true): Promise<DeploymentResult> =>
  provider.execute<DeploymentResult>({ command: "deploy", args: dryRun ? ["--dry-run"] : [] });
export const validateGame = (provider: ProductFacadeProvider, profile = "quick"): Promise<ValidationResult> =>
  provider.execute<ValidationResult>({ command: "validate", args: ["--profile", profile] });
export const status = (provider: ProductFacadeProvider): Promise<StatusResult> => provider.execute<StatusResult>({ command: "status" });

export const DEFAULT_ARENA_VANGUARD_GATEWAY_PORT = "8791";

export function resolveArenaVanguardRuntimeFeedUrl(locationLike: Pick<Location, "protocol" | "hostname"> = globalThis.location): string {
  const protocol = locationLike.protocol === "https:" ? "wss" : "ws";
  const gatewayPort = import.meta.env?.VITE_ARENA_VANGUARD_GATEWAY_PORT ?? DEFAULT_ARENA_VANGUARD_GATEWAY_PORT;
  return `${protocol}://${locationLike.hostname}:${gatewayPort}/runtime-feed`;
}

export type ArenaVanguardAction = "join" | "leave" | "move" | "attack" | "interact" | "use-item" | "resume" | "heartbeat" | "world-state" | "status";

export interface ArenaVanguardGatewayProvider {
  submit<T>(action: ArenaVanguardAction, payload?: Record<string, unknown>): Promise<T>;
}

export class ArenaVanguardHttpProvider implements ArenaVanguardGatewayProvider {
  constructor(private readonly baseUrl: string) {}

  async submit<T>(action: ArenaVanguardAction, payload: Record<string, unknown> = {}): Promise<T> {
    const response = await fetch(`${this.baseUrl}/${action}`, {
      method: ["status", "heartbeat", "world-state"].includes(action) ? "GET" : "POST",
      headers: { "content-type": "application/json" },
      body: ["status", "heartbeat", "world-state"].includes(action) ? undefined : JSON.stringify(payload)
    });
    if (!response.ok) throw new Error(`arena vanguard gateway failed: ${response.status}`);
    return response.json() as Promise<T>;
  }
}

export const arenaVanguardGateway = {
  join: (provider: ArenaVanguardGatewayProvider, playerSeed: string) => provider.submit("join", { playerSeed }),
  leave: (provider: ArenaVanguardGatewayProvider, playerId: string) => provider.submit("leave", { playerId }),
  move: (provider: ArenaVanguardGatewayProvider, playerId: string, dx: number, dy: number) => provider.submit("move", { playerId, dx, dy }),
  attack: (provider: ArenaVanguardGatewayProvider, playerId: string, targetId: string) => provider.submit("attack", { playerId, targetId }),
  interact: (provider: ArenaVanguardGatewayProvider, playerId: string, item: string) => provider.submit("interact", { playerId, item }),
  useItem: (provider: ArenaVanguardGatewayProvider, playerId: string, itemId: string) => provider.submit("use-item", { playerId, itemId }),
  resume: (provider: ArenaVanguardGatewayProvider, resumeToken: string) => provider.submit("resume", { resumeToken }),
  heartbeat: (provider: ArenaVanguardGatewayProvider) => provider.submit("heartbeat"),
  worldState: (provider: ArenaVanguardGatewayProvider) => provider.submit("world-state"),
  status: (provider: ArenaVanguardGatewayProvider) => provider.submit("status")
};
