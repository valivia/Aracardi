import { version } from "$app/environment";
import { PUBLIC_SERVER_HTTP_URL, PUBLIC_SERVER_WS_URL } from "$env/static/public";
import {
    ConnectionClose,
    IncomingMessageTopic,
    OutgoingMessageTopic,
    type IncomingTopicMap,
    type OutgoingTopicMap,
} from "./protocol";

const CLIENT_ID_KEY = "clientId";
const TAG = "[ws]";

export enum ConnectionStatus {
    Idle = "IDLE",
    Connecting = "CONNECTING",
    Connected = "CONNECTED",
    Closed = "CLOSED",
}

export interface ConnectionCloseProtocol {
    message: string;
    canRecreate?: boolean;
    canReconnect?: boolean;
}

const connectionCloseProtocol = {
    // Normal
    GAME_ENDED: {
        message: "This session no longer exists",
        canRecreate: true,
    },

    //
    NOT_FOUND: {
        message: "Could not find game lobby",
        canRecreate: true,
    },
    GAME_FULL: {
        message: "This session is full",
    },

    // Network / timeout
    TIMED_OUT: {
        message: "Session timed out",
        canReconnect: true,
        canRecreate: true,
    },

    // Server issues
    SERVER_ERROR: {
        message: "An error occurred",
        canReconnect: true,
        canRecreate: true,
    },
    SERVER_RESTART: {
        message: "Session lost due to server maintenance",
        canRecreate: true,
    },

    RATE_LIMITED: {
        message: "Too many connection attempts — please wait a moment",
        canRecreate: true,
    },

    // Protocol / version errors
    INVALID_HANDSHAKE: {
        message: "Connection rejected (invalid handshake)",
    },
    VERSION_MISMATCH: {
        message: "A newer version is available, please recreate the game",
        canRecreate: false,
    },
} as const satisfies Record<ConnectionClose, ConnectionCloseProtocol>;

const connectionLost = { message: "Connection lost", canReconnect: true };

const closeCodeFallbacks: Partial<Record<number, ConnectionCloseProtocol>> = {
    1000: { message: "Session ended" },
    // 1001 — endpoint going away (e.g. page navigation); treat like a drop
    1001: { message: "Connection closed", canReconnect: true },
    1011: { message: "Server error", canReconnect: true, canRecreate: true },
    1012: { message: "Server restarting", canReconnect: true, canRecreate: true },
    // 1006 — abnormal closure: no close frame received (network drop, crash)
    1006: connectionLost,
};

class SocketError extends Error {
    override name = "SocketError";

    constructor(
        public readonly code: number,
        message: string,
    ) {
        super(message);
    }

    public get_protocol(): ConnectionCloseProtocol {
        // Known server reason token
        const serverReason = connectionCloseProtocol[this.message as ConnectionClose];
        if (serverReason) return serverReason;

        // Code-based fallback
        const codeFallback = closeCodeFallbacks[this.code];
        if (codeFallback) return codeFallback;

        //  Last resort — surface the raw message so it's at least visible
        return { message: this.message || "Connection closed unexpectedly", canReconnect: false };
    }
}

export type SocketState =
    | {
          status: ConnectionStatus.Closed;
          reason: ConnectionCloseProtocol;
      }
    | {
          status: Exclude<ConnectionStatus, ConnectionStatus.Closed>;
          reason?: never;
      };

const HANDSHAKE_TIMEOUT_MS = 5_000;

const RECONNECT_DELAY = 2_000;
const MAX_RECONNECT_DELAY = 20_000;
const MAX_RECONNECT_ATTEMPTS = Math.ceil((1000 * 60 * 15) / MAX_RECONNECT_DELAY);

export class WebsocketClient {
    public readonly session_id: string;
    public readonly isHost: boolean;

    private socket?: WebSocket;
    private clientId?: string;

    // Listeners
    private messageListeners = new Map<string, Set<(payload: unknown) => void>>();
    private closeListeners = new Set<() => void>();
    private readyListeners = new Set<() => void>();

    // Connection
    private isConnecting = false;
    public socketState: SocketState = $state({ status: ConnectionStatus.Idle });
    private reconnectAttempts = $state(0);
    private reconnectTimer?: ReturnType<typeof setTimeout>;
    private manualClose = false;

    public statusString = $derived.by(() => {
        switch (this.socketState.status) {
            case ConnectionStatus.Closed:
                return this.socketState.reason.message;
        }

        return "";
    });

    constructor(id: string, isHost = false) {
        this.session_id = id;
        this.isHost = isHost;
    }

    public static async createSession(): Promise<WebsocketClient> {
        let payload: { joinCode: string; hostId: string };

        try {
            const response = await fetch(`${PUBLIC_SERVER_HTTP_URL}/lobby`, { method: "POST" });
            if (!response.ok) throw new Error("Non 200 response");
            payload = await response.json();
        } catch (error) {
            throw { message: "Couldn't reach server" };
        }

        console.log(`${TAG} Created session with ID:`, payload.joinCode);

        const client = new this(payload.joinCode, true);
        try {
            await client.connectToSocket(payload.hostId);
        } catch {
            throw client.socketState.reason;
        }

        return client;
    }

    // ### Socket ###
    public async restartConnectionCycle() {
        clearTimeout(this.reconnectTimer);
        this.reconnectAttempts = 0;
        await this.connectToSocket();
    }

    public async connectToSocket(requestedClientId?: string): Promise<void> {
        if (this.isConnecting) {
            console.warn(`${TAG} double reconnect`, this.session_id);
            return;
        }

        console.debug(`${TAG} Connecting`);

        this.isConnecting = true;

        clearTimeout(this.reconnectTimer);

        if (this.socket) this.detachSocketListeners(this.socket);

        this.socketState = { status: ConnectionStatus.Connecting };
        this.manualClose = false;

        const clientId = requestedClientId ?? this.clientId ?? sessionStorage.getItem(CLIENT_ID_KEY) ?? undefined;

        const url = `${PUBLIC_SERVER_WS_URL}/lobby/${this.session_id}/ws`;

        this.socket = undefined;
        const socket = new WebSocket(url);

        try {
            const assignedClientId = await WebsocketClient.connectionHandshake(socket, clientId);

            sessionStorage.setItem(CLIENT_ID_KEY, assignedClientId);

            // Set state
            this.clientId = assignedClientId;
            this.socket = socket;
            this.socketState = { status: ConnectionStatus.Connected };
            this.reconnectAttempts = 0;
            this.attachSocketListeners(socket);
            this.isConnecting = false;

            console.log(`${TAG} Connected! `);
        } catch (error) {
            if (error instanceof SocketError && !error.get_protocol().canReconnect) {
                console.info(`${TAG} Connection refused:`, error.message);
                this.socketState = {
                    status: ConnectionStatus.Closed,
                    reason: error.get_protocol(),
                };
            } else {
                console.error(`${TAG} Connection failed:`, error);
                this.scheduleReconnect(requestedClientId);
            }

            this.isConnecting = false;
            throw error;
        }
    }

    private static connectionHandshake(socket: WebSocket, requestedClientId?: string): Promise<string> {
        return new Promise<string>((resolve, reject) => {
            const timeout = setTimeout(() => reject(new Error("WebSocket handshake timed out")), HANDSHAKE_TIMEOUT_MS);

            const cleanup = () => {
                clearTimeout(timeout);
                socket.removeEventListener("message", onMessage);
                socket.removeEventListener("error", onError);
                socket.removeEventListener("close", onClose);
            };

            const onMessage = (event: MessageEvent) => {
                const message = WebsocketClient.parseMessage(event.data);
                if (message?.topic !== IncomingMessageTopic.ClientId) return;
                cleanup();
                resolve(message.payload);
            };

            const onError = () => {
                cleanup();
                reject(new Error("WebSocket connection error during handshake"));
            };

            const onClose = (event: CloseEvent) => {
                cleanup();
                reject(new SocketError(event.code, event.reason));
            };

            socket.addEventListener("message", onMessage);
            socket.addEventListener("error", onError);
            socket.addEventListener("close", onClose);

            let connect_message: OutgoingTopicMap[OutgoingMessageTopic.Connect] = {
                clientId: requestedClientId,
                version,
            };

            socket.addEventListener("open", () => {
                socket.send(`${OutgoingMessageTopic.Connect}\n${JSON.stringify(connect_message)}`);
            });
        });
    }

    private socketOnMessage = (event: any) => {
        const data: string = event.data;
        const message = WebsocketClient.parseMessage(data);
        if (message === null) return;

        const listeners = this.messageListeners.get(message.topic);
        if (!listeners) return;

        try {
            const payload = JSON.parse(message.payload);
            for (const callback of listeners) callback(payload);
        } catch {
            console.error(`${TAG} Failed to parse message payload for topic:`, message.topic);
        }
    };

    private socketOnClose = (event: any) => {
        let error = new SocketError(event.code, event.reason);
        console.debug(`${TAG} Socket closed`, { event, protocol: error.get_protocol() });

        if (!error.get_protocol().canReconnect || this.manualClose) {
            this.socketState = { status: ConnectionStatus.Closed, reason: error.get_protocol() };

            for (const callback of this.closeListeners) callback();
            return;
        }

        this.scheduleReconnect();
    };

    private detachSocketListeners(socket: WebSocket): void {
        socket.removeEventListener("message", this.socketOnMessage);
        socket.removeEventListener("close", this.socketOnClose);
    }

    private attachSocketListeners(socket: WebSocket): void {
        socket.addEventListener("message", this.socketOnMessage);
        socket.addEventListener("close", this.socketOnClose);

        for (const callback of this.readyListeners) callback();
    }

    private scheduleReconnect(requestedClientId?: string): void {
        // Don't reconnect if tab is not active
        if (document.visibilityState === "hidden") return;

        if (this.reconnectAttempts >= MAX_RECONNECT_ATTEMPTS) {
            console.error(`${TAG} Max reconnect attempts reached.`);
            this.socketState = {
                status: ConnectionStatus.Closed,
                reason: { message: "Failed to connect.", canReconnect: false },
            };

            for (const cb of this.closeListeners) cb();
            return;
        }

        const delay = this.getReconnectDelay();
        this.reconnectAttempts++;

        console.debug(
            `${TAG} Reconnecting in ${delay}ms (attempt ${this.reconnectAttempts} / ${MAX_RECONNECT_ATTEMPTS})`,
        );

        this.reconnectTimer = setTimeout(async () => {
            await this.connectToSocket(requestedClientId).catch(() => null);
        }, delay);
    }

    //  ### API ###
    public send<Topic extends keyof OutgoingTopicMap>(topic: Topic, payload: OutgoingTopicMap[Topic]) {
        if (this.socketState.status !== ConnectionStatus.Connected) {
            console.warn(`${TAG} Tried sending package before handshake completion`);
            return;
        }

        const message = `${topic}\n${typeof payload === "string" ? payload : JSON.stringify(payload)}`;
        this.socket?.send(message);
    }

    public onMessage<Topic extends IncomingMessageTopic>(
        topic: Topic,
        callback: (payload: IncomingTopicMap[Topic]) => void,
    ): () => void {
        if (!this.messageListeners.has(topic)) {
            this.messageListeners.set(topic, new Set());
        }
        const listeners = this.messageListeners.get(topic)!;
        listeners.add(callback as (payload: unknown) => void);

        return () => listeners.delete(callback as (payload: unknown) => void);
    }

    public onReady(callback: () => void): () => void {
        this.readyListeners.add(callback);
        return () => this.readyListeners.delete(callback);
    }

    public onClose(callback: () => void): () => void {
        this.closeListeners.add(callback);
        return () => this.closeListeners.delete(callback);
    }

    public close() {
        this.manualClose = true;
        clearTimeout(this.reconnectTimer);
        this.socket?.close();
        this.socketState = { status: ConnectionStatus.Closed, reason: connectionCloseProtocol.GAME_ENDED };
    }

    // ### Events ###
    public onVisibilityChange() {
        // Going to sleep, cancel queued reconnects
        if (document.visibilityState === "hidden") {
            clearTimeout(this.reconnectTimer);
            return;
        }

        // Currently reconnecting, ignore
        if (this.isConnecting) {
            return; // Already in progress, do nothing
        }

        // Reconnect if the socket is dead
        const isSocketClosed = this.socket === undefined || this.socket.readyState === WebSocket.CLOSED;
        if (isSocketClosed && this.socketState.status !== ConnectionStatus.Closed) {
            console.debug(`${TAG} Woke up with dead socket, triggering reconnect`);
            this.restartConnectionCycle();
            return;
        }

        // Send pong to confirm state
        if (this.socketState.status === ConnectionStatus.Connected) {
            this.send(OutgoingMessageTopic.Pong, "");
            return;
        }
    }

    // ### Helpers ###
    public static parseMessage(data: string): { topic: string; payload: string } | null {
        const newLineIndex = data.indexOf("\n");
        if (newLineIndex === -1) return null;

        return {
            topic: data.slice(0, newLineIndex),
            payload: data.slice(newLineIndex + 1),
        };
    }

    private getReconnectDelay(): number {
        const base = Math.min(RECONNECT_DELAY * this.reconnectAttempts, MAX_RECONNECT_DELAY);
        const jitter = Math.random() * 1000;
        return Math.round(base + jitter);
    }
}
