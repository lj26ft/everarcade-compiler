import type { DeploymentResult, DoctorResult, PackageResult, ProjectResult, RustrigResult, StatusResult, ValidationResult } from "@everarcade/shared-types";

export type ProductCommand = "doctor" | "new" | "add-rustrig" | "run" | "package" | "rehearse" | "deploy" | "validate" | "status";

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
