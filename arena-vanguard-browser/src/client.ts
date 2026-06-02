export type RuntimePosition = { x: number; y: number; zone: string };
export type RuntimePlayer = { playerId: string; characterId: string; position: RuntimePosition; health: number; energy: number; xp: number; level: number; inventory: string[]; connected: boolean; resumeToken: string };
export type RuntimeEnemy = { enemyId: string; position: RuntimePosition; health: number; status: string; respawnTick?: number | null };
export type RuntimeLoot = { lootId: string; itemId: string; position: RuntimePosition; available: boolean; claimedBy?: string | null };
export type WorldStateFeed = { tick: number; session_id?: string; sessionId?: string; players: RuntimePlayer[]; enemies: RuntimeEnemy[]; zones: string[]; loot: RuntimeLoot[]; runtime_health?: unknown; checkpoint_age?: number; checkpointAge?: number; replay_size?: number; replaySize?: number };

export type BrowserAction =
  | { type: "join"; playerSeed: string }
  | { type: "move"; playerId: string; dx: number; dy: number }
  | { type: "attack"; playerId: string; enemyId: string }
  | { type: "interact"; playerId: string; itemId: string }
  | { type: "resume"; resumeToken: string }
  | { type: "heartbeat" };

export class ArenaVanguardBrowserClient {
  private sequence = 0;
  private socket?: WebSocket;
  public latestFeed?: WorldStateFeed;
  public resumeToken?: string;

  constructor(private readonly websocketUrl: string) {}

  connect() {
    this.socket = new WebSocket(this.websocketUrl);
    this.socket.binaryType = "arraybuffer";
    this.socket.onmessage = event => this.consume(event.data);
  }

  join(playerSeed: string) { this.send({ type: "join", playerSeed }); }
  move(playerId: string, dx: number, dy: number) { this.send({ type: "move", playerId, dx, dy }); }
  attack(playerId: string, enemyId: string) { this.send({ type: "attack", playerId, enemyId }); }
  loot(playerId: string, itemId: string) { this.send({ type: "interact", playerId, itemId }); }
  resume() { if (this.resumeToken) this.send({ type: "resume", resumeToken: this.resumeToken }); }
  heartbeat() { this.send({ type: "heartbeat" }); }

  send(action: BrowserAction) {
    const frame = JSON.stringify({ sequence: ++this.sequence, action });
    this.socket?.send(frame);
  }

  consume(raw: string | ArrayBuffer) {
    const text = typeof raw === "string" ? raw : new TextDecoder().decode(raw);
    const event = JSON.parse(text);
    const feed = event.feed ?? event.payload?.feed ?? (event.type === "WorldStateFeed" ? event.payload : undefined);
    if (feed) this.latestFeed = feed;
    const session = event.session ?? event.payload?.session;
    if (session?.resumeToken) this.resumeToken = session.resumeToken;
  }
}
