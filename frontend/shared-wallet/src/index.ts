import type { WalletConnection } from "@everarcade/shared-types";

export type WalletProviderKind = "xaman" | "xrpl" | "xahau-future";

export interface WalletProviderAdapter {
  kind: WalletProviderKind;
  connect(): Promise<WalletConnection>;
  disconnect(): Promise<void>;
}

export class DeterministicWalletAdapter implements WalletProviderAdapter {
  constructor(public readonly kind: WalletProviderKind = "xaman") {}
  async connect(): Promise<WalletConnection> {
    return {
      provider: this.kind,
      address: "rEverArcadeDemoWalletAddress",
      network: this.kind === "xahau-future" ? "xahau-future" : "xrpl-testnet",
      connected: true
    };
  }
  async disconnect(): Promise<void> {}
}

export async function connectWallet(adapter: WalletProviderAdapter = new DeterministicWalletAdapter()): Promise<WalletConnection> {
  return adapter.connect();
}

export async function disconnectWallet(adapter: WalletProviderAdapter = new DeterministicWalletAdapter()): Promise<{ connected: false }> {
  await adapter.disconnect();
  return { connected: false };
}
