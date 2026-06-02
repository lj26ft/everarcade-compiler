import { status } from "@everarcade/shared-api";

export const healthPanels = ["Runtime Health", "Replay Health", "Deployment Status", "Federation Health", "Alerts", "Metrics"];
export const metrics = ["Ticks/sec", "Checkpoint Age", "Replay Growth", "Runtime Count", "Deployment Count", "Federation Nodes"];
export const alertStates = ["Healthy", "Warning", "Critical"];
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
