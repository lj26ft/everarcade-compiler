import { status } from "@everarcade/shared-api";

export const healthPanels = ["Connected Browsers", "WebSocket Connections", "Session Count", "Active Sessions", "Players Online", "Gateway Status", "Runtime Health", "Gateway Health", "Runtime Status", "Replay Growth", "Checkpoint Age", "Recovery State", "Alerts", "Metrics"];
export const metrics = ["WebSocket Count", "Connection Rate", "Reconnect Rate", "World Feed Rate", "Action Throughput", "Join Rate", "Gateway Latency", "Session Duration", "Session Count", "Player Count", "Runtime Tick", "Checkpoint Age", "Replay Growth"];
export const alertStates = ["Healthy", "Warning", "Failed", "Recovering"];
export const alerts = ["Runtime Stalled", "Checkpoint Delay", "Recovery Failure", "Deployment Failure"];

export function App() {
  return <main>
    <h1>⚙ EverArcade Operator Console</h1>
    <section aria-label="Deployment Monitoring">
      {healthPanels.map(panel => <article key={panel}>{panel}</article>)}
    </section>
    <section aria-label="Metrics Dashboard">
      {metrics.map(metric => <article key={metric} data-source="everarcade status --json">{metric}</article>)}
    </section>
    <section aria-label="Alert Dashboard">
      {alertStates.map(state => <strong key={state}>{state}</strong>)}
      {alerts.map(alert => <article key={alert}>{alert}</article>)}
    </section>
  </main>;
}

export const operatorBindings = { status };
