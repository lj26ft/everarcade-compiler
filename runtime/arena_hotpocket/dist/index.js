/******/ (() => { // webpackBootstrap
/******/ 	var __webpack_modules__ = ({

/***/ 875:
/***/ ((module, __unused_webpack_exports, __nccwpck_require__) => {

/******/ (() => { // webpackBootstrap
/******/ 	var __webpack_modules__ = ({

/***/ 782:
/***/ ((__unused_webpack_module, __webpack_exports__, __nccwpck_require2_) => {

"use strict";
__nccwpck_require2_.r(__webpack_exports__);
/* harmony export */ __nccwpck_require2_.d(__webpack_exports__, {
/* harmony export */   "controlMessages": () => (/* binding */ controlMessages),
/* harmony export */   "clientProtocols": () => (/* binding */ clientProtocols),
/* harmony export */   "constants": () => (/* binding */ constants),
/* harmony export */   "writeAsync": () => (/* binding */ writeAsync),
/* harmony export */   "writevAsync": () => (/* binding */ writevAsync),
/* harmony export */   "readAsync": () => (/* binding */ readAsync),
/* harmony export */   "invokeCallback": () => (/* binding */ invokeCallback),
/* harmony export */   "errHandler": () => (/* binding */ errHandler)
/* harmony export */ });
const fs = __nccwpck_require2_(147);

const controlMessages = {
    peerChangeset: "peer_changeset"
}
Object.freeze(controlMessages);

const clientProtocols = {
    json: "json",
    bson: "bson"
}
Object.freeze(clientProtocols);

const constants = {
    MAX_SEQ_PACKET_SIZE: 128 * 1024,
    PATCH_CONFIG_PATH: "../patch.cfg",
    POST_EXEC_SCRIPT_NAME: "post_exec.sh"
}
Object.freeze(constants);

function writeAsync(fd, buf) {
    return new Promise(resolve => fs.write(fd, buf, resolve));
}
function writevAsync(fd, bufList) {
    return new Promise(resolve => fs.writev(fd, bufList, resolve));
}
function readAsync(fd, buf, offset, size) {
    return new Promise(resolve => fs.read(fd, buf, 0, size, offset, resolve));
}

async function invokeCallback(callback, ...args) {
    if (!callback)
        return;

    if (callback.constructor.name === 'AsyncFunction') {
        await callback(...args).catch(errHandler);
    }
    else {
        callback(...args);
    }
}

function errHandler(err) {
    console.log(err);
}

/***/ }),

/***/ 244:
/***/ ((__unused_webpack_module, __webpack_exports__, __nccwpck_require2_) => {

"use strict";
// ESM COMPAT FLAG
__nccwpck_require2_.r(__webpack_exports__);

// EXPORTS
__nccwpck_require2_.d(__webpack_exports__, {
  "HotPocketContract": () => (/* binding */ HotPocketContract)
});

// EXTERNAL MODULE: ./src/common.js
var common = __nccwpck_require2_(782);
;// CONCATENATED MODULE: ./src/patch-config.js


const fs = __nccwpck_require2_(147);

// Handles patch config manipulation.
class PatchConfig {

    // Loads the config value if there's a patch config file. Otherwise throw error.
    getConfig() {
        if (!fs.existsSync(common.constants.PATCH_CONFIG_PATH))
            throw "Patch config file does not exist.";

        return new Promise((resolve, reject) => {
            fs.readFile(common.constants.PATCH_CONFIG_PATH, 'utf8', function (err, data) {
                if (err) reject(err);
                else resolve(JSON.parse(data));
            });
        });
    }

    updateConfig(config) {

        this.validateConfig(config);

        return new Promise((resolve, reject) => {
            // Format json to match with the patch.cfg json format created by HP at the startup.
            fs.writeFile(common.constants.PATCH_CONFIG_PATH, JSON.stringify(config, null, 4), (err) => {
                if (err) reject(err);
                else resolve();
            });
        });
    }

    validateConfig(config) {
        // Validate all config fields.
        if (!config.version)
            throw "Contract version is not specified.";
        if (!config.unl || !config.unl.length)
            throw "UNL list cannot be empty.";
        for (let publicKey of config.unl) {
            // Public keys are validated against length, ed prefix and hex characters.
            if (!publicKey.length)
                throw "UNL public key not specified.";
            else if (!(/^(e|E)(d|D)[0-9a-fA-F]{64}$/g.test(publicKey)))
                throw "Invalid UNL public key specified.";
        }
        if (!config.bin_path || !config.bin_path.length)
            throw "Binary path cannot be empty.";
        if (config.consensus.mode != "public" && config.consensus.mode != "private")
            throw "Invalid consensus mode configured in patch file. Valid values: public|private";
        if (config.consensus.roundtime < 1 && config.consensus.roundtime > 3600000)
            throw "Round time must be between 1 and 3600000ms inclusive.";
        if (config.consensus.stage_slice < 1 || config.consensus.stage_slice > 33)
            throw "Stage slice must be between 1 and 33 percent inclusive.";
        if (config.consensus.threshold < 1 || config.consensus.threshold > 100)
            throw "Consensus threshold must be between 1 and 100 percent inclusive.";
        if (config.npl.mode != "public" && config.npl.mode != "private")
            throw "Invalid npl mode configured in patch file. Valid values: public|private";
        if (config.round_limits.user_input_bytes < 0 || config.round_limits.user_output_bytes < 0 || config.round_limits.npl_output_bytes < 0 ||
            config.round_limits.proc_cpu_seconds < 0 || config.round_limits.proc_mem_bytes < 0 || config.round_limits.proc_ofd_count < 0)
            throw "Invalid round limits.";
        if (config.max_input_ledger_offset < 0)
            throw "Invalid max input ledger offset";
    }
}
;// CONCATENATED MODULE: ./src/contract-context.js



// HotPocket contract context which is passed into every smart contract invocation.

class ContractContext {

    #patchConfig = null;
    #controlChannel = null;

    constructor(hpargs, users, unl, controlChannel) {
        this.#patchConfig = new PatchConfig();
        this.#controlChannel = controlChannel;
        this.contractId = hpargs.contract_id;
        this.publicKey = hpargs.public_key;
        this.privateKey = hpargs.private_key;
        this.readonly = hpargs.readonly;
        this.timestamp = hpargs.timestamp;
        this.users = users;
        this.unl = unl; // Not available in readonly mode.
        this.lclSeqNo = hpargs.lcl_seq_no; // Not available in readonly mode.
        this.lclHash = hpargs.lcl_hash; // Not available in readonly mode.
    }

    // Returns the config values in patch config.
    getConfig() {
        return this.#patchConfig.getConfig();
    }

    // Updates the config with given config object and save the patch config.
    updateConfig(config) {
        return this.#patchConfig.updateConfig(config);
    }

    // Updates the known-peers this node must attempt connections to.
    // toAdd: Array of strings containing peers to be added. Each string must be in the format of "<ip>:<port>".
    updatePeers(toAdd, toRemove) {
        return this.#controlChannel.send({
            type: common.controlMessages.peerChangeset,
            add: toAdd || [],
            remove: toRemove || []
        });
    }
}
;// CONCATENATED MODULE: ./src/control.js
const control_fs = __nccwpck_require2_(147);


class ControlChannel {

    #fd = null;
    #readStream = null;

    constructor(fd) {
        this.#fd = fd;
    }

    consume(onMessage) {

        if (this.#readStream)
            throw "Control channel already consumed.";

        this.#readStream = control_fs.createReadStream(null, { fd: this.#fd, highWaterMark: common.constants.MAX_SEQ_PACKET_SIZE });
        this.#readStream.on("data", onMessage);
        this.#readStream.on("error", (err) => { });
    }

    send(obj) {
        const buf = Buffer.from(JSON.stringify(obj));
        if (buf.length > common.constants.MAX_SEQ_PACKET_SIZE)
            throw ("Control message exceeds max size " + common.constants.MAX_SEQ_PACKET_SIZE);
        return (0,common.writeAsync)(this.#fd, buf);
    }

    close() {
        this.#readStream && this.#readStream.close();
    }
}
;// CONCATENATED MODULE: ./src/npl.js


const npl_fs = __nccwpck_require2_(147);

// Represents the node-party-line that can be used to communicate with unl nodes.
class NplChannel {

    #fd = null;
    #readStream = null;

    constructor(fd) {
        this.#fd = fd;
    }

    consume(onMessage) {

        if (this.#readStream)
            throw "NPL channel already consumed.";

        this.#readStream = npl_fs.createReadStream(null, { fd: this.#fd, highWaterMark: common.constants.MAX_SEQ_PACKET_SIZE });

        // When hotpocket is sending the npl messages, first it sends the public key of the particular node
        // and then the message, First data buffer is taken as public key and the second one as message,
        // then npl message object is constructed and the event is emmited.
        let publicKey = null;

        this.#readStream.on("data", (data) => {
            if (!publicKey) {
                publicKey = data.toString();
            }
            else {
                onMessage(publicKey, data);
                publicKey = null;
            }
        });

        this.#readStream.on("error", (err) => { });
    }

    send(msg) {
        const buf = Buffer.from(msg);
        if (buf.length > common.constants.MAX_SEQ_PACKET_SIZE)
            throw ("NPL message exceeds max size " + common.constants.MAX_SEQ_PACKET_SIZE);
        return (0,common.writeAsync)(this.#fd, buf);
    }

    close() {
        this.#readStream && this.#readStream.close();
    }
}

;// CONCATENATED MODULE: ./src/unl.js


class UnlCollection {

    #readonly = null;
    #pendingTasks = null;
    #channel = null;

    constructor(readonly, unl, channel, pendingTasks) {
        this.nodes = {};
        this.#readonly = readonly;
        this.#pendingTasks = pendingTasks;

        if (!readonly) {
            for (const [publicKey, stat] of Object.entries(unl)) {
                this.nodes[publicKey] = new UnlNode(publicKey, stat.active_on);
            }

            this.#channel = channel;
        }
    }

    // Returns the unl node for the specified public key. Returns null if not found.
    find(publicKey) {
        return this.nodes[publicKey];
    }

    // Returns all the unl nodes.
    list() {
        return Object.values(this.nodes);
    }

    count() {
        return Object.keys(this.nodes).length;
    }

    // Registers for NPL messages.
    onMessage(callback) {

        if (this.#readonly)
            throw "NPL messages not available in readonly mode.";

        this.#channel.consume((publicKey, msg) => {
            this.#pendingTasks.push((0,common.invokeCallback)(callback, this.nodes[publicKey], msg));
        });
    }

    // Broadcasts a message to all unl nodes (including self if self is part of unl).
    async send(msg) {
        if (this.#readonly)
            throw "NPL messages not available in readonly mode.";

        await this.#channel.send(msg);
    }
}

// Represents a node that's part of unl.
class UnlNode {

    constructor(publicKey, activeOn) {
        this.publicKey = publicKey;
        this.activeOn = activeOn;
    }
}
;// CONCATENATED MODULE: ./src/user.js


class UsersCollection {

    #users = {};
    #infd = null;

    constructor(userInputsFd, usersObj, clientProtocol) {
        this.#infd = userInputsFd;

        Object.entries(usersObj).forEach(([publicKey, arr]) => {

            const outfd = arr[0]; // First array element is the output fd.
            arr.splice(0, 1); // Remove first element (output fd). The rest are pairs of msg offset/length tuples.

            const channel = new UserChannel(outfd, clientProtocol);
            this.#users[publicKey] = new User(publicKey, channel, arr);
        });
    }

    // Returns the User for the specified public key. Returns null if not found.
    find(publicKey) {
        return this.#users[publicKey]
    }

    // Returns all the currently connected users.
    list() {
        return Object.values(this.#users);
    }

    count() {
        return Object.keys(this.#users).length;
    }

    async read(input) {
        const [offset, size] = input;
        const buf = Buffer.alloc(size);
        await (0,common.readAsync)(this.#infd, buf, offset, size);
        return buf;
    }
}

class User {

    #channel = null;

    constructor(publicKey, channel, inputs) {
        this.publicKey = publicKey;
        this.inputs = inputs;
        this.#channel = channel;
    }

    async send(msg) {
        await this.#channel.send(msg);
    }
}

class UserChannel {

    #outfd = null;
    #clientProtocol = null;

    constructor(outfd, clientProtocol) {
        this.#outfd = outfd;
        this.#clientProtocol = clientProtocol;
    }

    send(msg) {
        const messageBuf = this.serialize(msg);
        let headerBuf = Buffer.alloc(4);
        // Writing message length in big endian format.
        headerBuf.writeUInt32BE(messageBuf.byteLength)
        return (0,common.writevAsync)(this.#outfd, [headerBuf, messageBuf]);
    }

    serialize(msg) {

        if (!msg)
            throw "Cannot serialize null content.";

        if (Buffer.isBuffer(msg))
            return msg;
        else if (this.#clientProtocol == common.clientProtocols.bson)
            return Buffer.from(msg);
        else // json
            return Buffer.from(JSON.stringify(msg));
    }
}
;// CONCATENATED MODULE: ./src/hotpocket-contract.js







const hotpocket_contract_fs = __nccwpck_require2_(147);
const tty = __nccwpck_require2_(224);

class HotPocketContract {

    #controlChannel = null;
    #clientProtocol = null;
    #forceTerminate = false;

    init(contractFunc, clientProtocol = common.clientProtocols.json, forceTerminate = false) {

        return new Promise(resolve => {
            if (this.#controlChannel) { // Already initialized.
                resolve(false);
                return;
            }

            this.#clientProtocol = clientProtocol;

            // Check whether we are running on a console and provide error.
            if (tty.isatty(process.stdin.fd)) {
                console.error("Error: HotPocket smart contracts must be executed via HotPocket.");
                resolve(false);
                return;
            }

            this.#forceTerminate = forceTerminate;

            // Parse HotPocket args.
            hotpocket_contract_fs.readFile(process.stdin.fd, 'utf8', (err, argsJson) => {
                const hpargs = JSON.parse(argsJson);
                this.#controlChannel = new ControlChannel(hpargs.control_fd);
                this.#executeContract(hpargs, contractFunc);
                resolve(true);
            });
        });
    }

    #executeContract(hpargs, contractFunc) {
        // Keeps track of all the tasks (promises) that must be awaited before the termination.
        const pendingTasks = [];
        const nplChannel = new NplChannel(hpargs.npl_fd);

        const users = new UsersCollection(hpargs.user_in_fd, hpargs.users, this.#clientProtocol);
        const unl = new UnlCollection(hpargs.readonly, hpargs.unl, nplChannel, pendingTasks);
        const executionContext = new ContractContext(hpargs, users, unl, this.#controlChannel);

        (0,common.invokeCallback)(contractFunc, executionContext).catch(common.errHandler).finally(() => {
            // Wait for any pending tasks added during execution.
            Promise.all(pendingTasks).catch(common.errHandler).finally(() => {
                nplChannel.close();
                this.#terminate();
            });
        });
    }

    #terminate() {
        this.#controlChannel.close();
        if (this.#forceTerminate)
            process.kill(process.pid, 'SIGINT');
    }
}

/***/ }),

/***/ 53:
/***/ ((module, __unused_webpack_exports, __nccwpck_require2_) => {

const { clientProtocols, constants } = __nccwpck_require2_(782);
const { HotPocketContract } = __nccwpck_require2_(244);

module.exports = {
    Contract: HotPocketContract,
    clientProtocols,
    POST_EXEC_SCRIPT_NAME: constants.POST_EXEC_SCRIPT_NAME,
}

/***/ }),

/***/ 147:
/***/ ((module) => {

"use strict";
module.exports = __nccwpck_require__(147);

/***/ }),

/***/ 224:
/***/ ((module) => {

"use strict";
module.exports = __nccwpck_require__(224);

/***/ })

/******/ 	});
/************************************************************************/
/******/ 	// The module cache
/******/ 	var __webpack_module_cache__ = {};
/******/ 	
/******/ 	// The require function
/******/ 	function __nccwpck_require2_(moduleId) {
/******/ 		// Check if module is in cache
/******/ 		var cachedModule = __webpack_module_cache__[moduleId];
/******/ 		if (cachedModule !== undefined) {
/******/ 			return cachedModule.exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = __webpack_module_cache__[moduleId] = {
/******/ 			// no module.id needed
/******/ 			// no module.loaded needed
/******/ 			exports: {}
/******/ 		};
/******/ 	
/******/ 		// Execute the module function
/******/ 		var threw = true;
/******/ 		try {
/******/ 			__webpack_modules__[moduleId](module, module.exports, __nccwpck_require2_);
/******/ 			threw = false;
/******/ 		} finally {
/******/ 			if(threw) delete __webpack_module_cache__[moduleId];
/******/ 		}
/******/ 	
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/ 	
/************************************************************************/
/******/ 	/* webpack/runtime/define property getters */
/******/ 	(() => {
/******/ 		// define getter functions for harmony exports
/******/ 		__nccwpck_require2_.d = (exports, definition) => {
/******/ 			for(var key in definition) {
/******/ 				if(__nccwpck_require2_.o(definition, key) && !__nccwpck_require2_.o(exports, key)) {
/******/ 					Object.defineProperty(exports, key, { enumerable: true, get: definition[key] });
/******/ 				}
/******/ 			}
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/hasOwnProperty shorthand */
/******/ 	(() => {
/******/ 		__nccwpck_require2_.o = (obj, prop) => (Object.prototype.hasOwnProperty.call(obj, prop))
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/make namespace object */
/******/ 	(() => {
/******/ 		// define __esModule on exports
/******/ 		__nccwpck_require2_.r = (exports) => {
/******/ 			if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 				Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 			}
/******/ 			Object.defineProperty(exports, '__esModule', { value: true });
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/compat */
/******/ 	
/******/ 	if (typeof __nccwpck_require2_ !== 'undefined') __nccwpck_require2_.ab = __dirname + "/";
/******/ 	
/************************************************************************/
/******/ 	
/******/ 	// startup
/******/ 	// Load entry module and return exports
/******/ 	// This entry module is referenced by other modules so it can't be inlined
/******/ 	var __webpack_exports__ = __nccwpck_require2_(53);
/******/ 	module.exports = __webpack_exports__;
/******/ 	
/******/ })()
;

/***/ }),

/***/ 472:
/***/ ((module, __unused_webpack_exports, __nccwpck_require__) => {

const { createHash } = __nccwpck_require__(5);
const { existsSync, readFileSync, writeFileSync } = __nccwpck_require__(561);
const { join } = __nccwpck_require__(411);

const SCHEMA = 'everarcade.hotpocket.arena-vanguard.v1';
const DIRECTIONS = Object.freeze({ north: [0, -1], south: [0, 1], east: [1, 0], west: [-1, 0] });
const GENESIS = Object.freeze({ tick: 0, players: {}, combat_events: [], last_sequence: {}, commitments: [] });
const STATE_FILE = 'arena-wrapper-state.json';
const JOURNAL_FILE = 'arena-hotpocket-journal.json';

function canonicalize(value) {
  if (value === undefined) return 'null';
  if (value === null || typeof value !== 'object') return JSON.stringify(value);
  if (Array.isArray(value)) return `[${value.map(canonicalize).join(',')}]`;
  return `{${Object.keys(value).sort().map((key) => `${JSON.stringify(key)}:${canonicalize(value[key])}`).join(',')}}`;
}

function sha256(value) {
  return createHash('sha256').update(String(value)).digest('hex');
}

function canonicalHash(value) {
  return sha256(canonicalize(value));
}

function clone(value) {
  return JSON.parse(JSON.stringify(value));
}

function genesisState() {
  return clone(GENESIS);
}

function validateEnvelope(input) {
  if (!input || typeof input !== 'object' || Array.isArray(input)) throw new Error('input must be a JSON object');
  const { action, player } = input;
  if (!['join', 'move', 'attack', 'disconnect'].includes(action)) throw new Error(`unsupported action: ${action}`);
  if (typeof player !== 'string' || player.length === 0) throw new Error(`${action} requires player`);
  if (action === 'move') {
    if (!Object.prototype.hasOwnProperty.call(DIRECTIONS, input.direction)) throw new Error('move requires direction north|south|east|west');
    return { action, direction: input.direction, player };
  }
  if (action === 'attack') {
    if (typeof input.target !== 'string' || input.target.length === 0) throw new Error('attack requires target');
    return { action, player, target: input.target };
  }
  return { action, player };
}

function ensurePlayer(state, player) {
  if (!state.players[player]) state.players[player] = { id: player, x: 0, y: 0, health: 100, connected: false, score: 0 };
  return state.players[player];
}

function applyArenaInput(state, envelope, tickOverride) {
  const action = validateEnvelope(envelope);
  const before = clone(state);
  const after = clone(state);
  let mutation = 'noop';
  const player = ensurePlayer(after, action.player);
  if (action.action === 'join') {
    player.connected = true;
    mutation = before.players[action.player] ? 'player_rejoined' : 'player_joined';
  }
  if (action.action === 'disconnect') {
    player.connected = false;
    mutation = 'player_disconnected';
  }
  if (action.action === 'move') {
    if (!player.connected) throw new Error(`cannot move disconnected player: ${action.player}`);
    const [dx, dy] = DIRECTIONS[action.direction];
    player.x += dx;
    player.y += dy;
    mutation = 'player_moved';
  }
  if (action.action === 'attack') {
    if (!player.connected) throw new Error(`cannot attack from disconnected player: ${action.player}`);
    const target = ensurePlayer(after, action.target);
    target.health = Math.max(0, target.health - 25);
    player.score += 10;
    after.combat_events.push({ tick: tickOverride ?? after.tick + 1, attacker: action.player, target: action.target, damage: 25, target_health: target.health });
    mutation = 'player_attacked';
  }
  after.tick = tickOverride ?? after.tick + 1;
  after.last_sequence[action.player] = after.tick;
  return { action, before, after, mutation };
}

function commitFor(state, receipts) {
  const world = { tick: state.tick, players: state.players, combat_events: state.combat_events };
  const world_hash = canonicalHash(world);
  const receipt_root = canonicalHash(receipts.map((receipt) => receipt.receipt_hash));
  const state_root = canonicalHash(state);
  const continuity_root = canonicalHash({ state_root, receipt_root, world_hash, tick: state.tick });
  return { tick: state.tick, state_root, receipt_root, world_hash, continuity_root };
}

function executeInput(state, envelope, sequence, priorReceipts = [], tickOverride) {
  const { action, before, after, mutation } = applyArenaInput(state, envelope, tickOverride);
  const canonical_input = canonicalize(action);
  const action_hash = sha256(canonical_input);
  const execution_id = `arena-hotpocket-${String(sequence).padStart(6, '0')}`;
  const baseReceipt = { schema: `${SCHEMA}.receipt`, execution_id, sequence, round: after.tick, status: 'accepted', generated_at: '1970-01-01T00:00:00.000Z', action_hash, state_before_hash: canonicalHash(before), mutation };
  const tempReceipt = { ...baseReceipt, state_root: canonicalHash(after) };
  const receipt = { ...tempReceipt, receipt_hash: canonicalHash(tempReceipt) };
  const commitments = commitFor(after, [...priorReceipts, receipt]);
  after.commitments.push(commitments);
  const output = { status: 'accepted', accepted: true, action: action.action, mutation, tick: after.tick, lclSeqNo: after.tick, players: clone(after.players), combat_events: clone(after.combat_events), ...commitments };
  Object.assign(receipt, { output, state_root: commitments.state_root, receipt_root: commitments.receipt_root, world_hash: commitments.world_hash, continuity_root: commitments.continuity_root });
  receipt.receipt_hash = canonicalHash({ ...receipt, receipt_hash: undefined });
  const journalBase = { schema: `${SCHEMA}.journal-entry`, execution_id, sequence, round: after.tick, canonical_input, canonical_state_before: canonicalize(before), canonical_state_after: canonicalize(after), action, state_before: before, state_after: after, action_hash, receipt_hash: receipt.receipt_hash, mutation, ...commitments };
  const journal = { ...journalBase, journal_hash: canonicalHash(journalBase) };
  return { state: after, receipt, journal, output, commitments };
}

function replayJournal(journal) {
  let state = genesisState();
  const receipts = [];
  const outputs = [];
  for (const [index, entry] of journal.entries()) {
    const result = executeInput(state, entry.action, index + 1, receipts, entry.round);
    receipts.push(result.receipt);
    outputs.push(result.output);
    state = result.state;
  }
  return { state, receipts, outputs, commitments: state.commitments.at(-1) || commitFor(state, receipts) };
}

class ArenaVanguard {
  constructor({ statePath = join(process.cwd(), STATE_FILE), journalPath = join(process.cwd(), JOURNAL_FILE) } = {}) {
    this.statePath = statePath;
    this.journalPath = journalPath;
    this.state = genesisState();
    this.receipts = [];
    this.journal = [];
    this.load();
  }

  load() {
    if (existsSync(this.statePath)) {
      const snapshot = JSON.parse(readFileSync(this.statePath, 'utf8'));
      this.state = snapshot.state || genesisState();
      this.receipts = snapshot.receipts || [];
      this.journal = snapshot.journal || [];
    } else if (existsSync(this.journalPath)) {
      this.journal = JSON.parse(readFileSync(this.journalPath, 'utf8'));
      const replayed = replayJournal(this.journal);
      this.state = replayed.state;
      this.receipts = replayed.receipts;
    }
    return this;
  }

  persist() {
    const snapshot = { state: this.state, receipts: this.receipts, journal: this.journal };
    writeFileSync(this.statePath, `${JSON.stringify(snapshot, null, 2)}\n`);
    writeFileSync(this.journalPath, `${JSON.stringify(this.journal, null, 2)}\n`);
  }

  async handleInput(publicKey, message, ctx = {}) {
    const tick = Number(ctx.lclSeqNo);
    if (!Number.isSafeInteger(tick)) throw new Error('ctx.lclSeqNo must be a safe integer tick');
    const action = validateEnvelope(message);
    const result = executeInput(this.state, action, this.journal.length + 1, this.receipts, tick);
    this.state = result.state;
    this.receipts.push(result.receipt);
    this.journal.push({ ...result.journal, hotpocket: { user: publicKey, readonly: Boolean(ctx.readonly), npl: ctx.npl } });
    if (!ctx.readonly) this.persist();
    return { schema: SCHEMA, input_id: inputId(action), user: publicKey, npl: ctx.npl, receipt: result.receipt, journal: result.journal, output: result.output, commitments: result.commitments, ...result.output };
  }

  snapshot() {
    const latest = this.state.commitments.at(-1) || commitFor(this.state, this.receipts);
    return { ...clone(this.state), receipts: this.receipts, journal_size: this.journal.length, replay_status: this.verify().ok ? 'verified' : 'mismatch', ...latest };
  }

  verify() {
    const replayed = replayJournal(this.journal);
    const live = this.state.commitments.at(-1) || commitFor(this.state, this.receipts);
    const ok = ['state_root', 'receipt_root', 'world_hash', 'continuity_root'].every((key) => replayed.commitments[key] === live[key]);
    return { ok, live, replayed: replayed.commitments };
  }
}

function inputId(envelope) {
  return `arena-${canonicalHash(envelope)}`;
}

module.exports = { ArenaVanguard, canonicalHash, canonicalize, commitFor, executeInput, genesisState, inputId, replayJournal, validateEnvelope };


/***/ }),

/***/ 513:
/***/ ((module, __unused_webpack_exports, __nccwpck_require__) => {

const HotPocket = __nccwpck_require__(875);
const { ArenaVanguard } = __nccwpck_require__(472);

async function contract(ctx) {
  const app = new ArenaVanguard();

  for (const user of ctx.users.list()) {
    for (const input of user.inputs) {
      try {
        const buf = await ctx.users.read(input);
        const message = JSON.parse(buf);
        const output = await app.handleInput(user.publicKey, message, {
          readonly: ctx.readonly,
          lclSeqNo: ctx.lclSeqNo,
          npl: ctx.npl
        });

        await user.send(JSON.stringify(output));
      } catch (error) {
        const output = {
          status: 'rejected',
          accepted: false,
          tick: Number(ctx.lclSeqNo),
          lclSeqNo: Number(ctx.lclSeqNo),
          error: error.message
        };
        await user.send(JSON.stringify(output));
      }
    }
  }
}

const hpc = new HotPocket.Contract();
hpc.init(contract);

module.exports = { contract };


/***/ }),

/***/ 147:
/***/ ((module) => {

"use strict";
module.exports = require("fs");

/***/ }),

/***/ 5:
/***/ ((module) => {

"use strict";
module.exports = require("node:crypto");

/***/ }),

/***/ 561:
/***/ ((module) => {

"use strict";
module.exports = require("node:fs");

/***/ }),

/***/ 411:
/***/ ((module) => {

"use strict";
module.exports = require("node:path");

/***/ }),

/***/ 224:
/***/ ((module) => {

"use strict";
module.exports = require("tty");

/***/ })

/******/ 	});
/************************************************************************/
/******/ 	// The module cache
/******/ 	var __webpack_module_cache__ = {};
/******/ 	
/******/ 	// The require function
/******/ 	function __nccwpck_require__(moduleId) {
/******/ 		// Check if module is in cache
/******/ 		var cachedModule = __webpack_module_cache__[moduleId];
/******/ 		if (cachedModule !== undefined) {
/******/ 			return cachedModule.exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = __webpack_module_cache__[moduleId] = {
/******/ 			// no module.id needed
/******/ 			// no module.loaded needed
/******/ 			exports: {}
/******/ 		};
/******/ 	
/******/ 		// Execute the module function
/******/ 		var threw = true;
/******/ 		try {
/******/ 			__webpack_modules__[moduleId](module, module.exports, __nccwpck_require__);
/******/ 			threw = false;
/******/ 		} finally {
/******/ 			if(threw) delete __webpack_module_cache__[moduleId];
/******/ 		}
/******/ 	
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/ 	
/************************************************************************/
/******/ 	/* webpack/runtime/compat */
/******/ 	
/******/ 	if (typeof __nccwpck_require__ !== 'undefined') __nccwpck_require__.ab = __dirname + "/";
/******/ 	
/************************************************************************/
/******/ 	
/******/ 	// startup
/******/ 	// Load entry module and return exports
/******/ 	// This entry module is referenced by other modules so it can't be inlined
/******/ 	var __webpack_exports__ = __nccwpck_require__(513);
/******/ 	module.exports = __webpack_exports__;
/******/ 	
/******/ })()
;