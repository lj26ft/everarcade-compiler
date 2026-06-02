export const statusIcons = {
  healthy: "🟢 Healthy",
  warning: "🟡 Warning",
  failed: "🔴 Failed",
  game: "🎮 Game",
  package: "📦 Package",
  deploy: "🚀 Deploy",
  runtime: "⚙ Runtime",
  world: "🌍 World",
  verification: "🔐 Verification",
  doctor: "🩺 Doctor",
  success: "🎉 Success"
} as const;

export function StatusBadge({ state }: { state: keyof typeof statusIcons }) {
  return <span className={`status status-${state}`}>{statusIcons[state]}</span>;
}

export function SurfaceCard({ title, children }: { title: string; children: unknown }) {
  return <section className="surface-card"><h2>{title}</h2><div>{children as never}</div></section>;
}
