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

interface ConnectionCloseProtocol {
    message: string;
    canRecreate?: boolean;
    canReconnect?: boolean;
}

const connectionCloseProtocol: Record<ConnectionClose, ConnectionCloseProtocol> = {
    GAME_ENDED: {
        message: "This session no longer exists",
        canRecreate: true,
    },
    NOT_FOUND: {
        message: "This session no longer exists",
        canRecreate: true,
    },
    GAME_FULL: {
        message: "This session is full",
    },
    TIMED_OUT: {
        message: "Session timed out",
        canReconnect: true,
    },
    SERVER_ERROR: {
        message: "An error occurred",
        canReconnect: true,
        canRecreate: true,
    },
    SERVER_RESTART: {
        message: "Session lost due to server maintenance",
        canRecreate: true,
    },
};

const connectionCloseProtocolDefaults: Record<string, ConnectionCloseProtocol> = {
    connectionFailed: {
        message: "Failed to connect.",
        canReconnect: true,
    },
    abnormalClosure: {
        message: "Connection lost",
        canReconnect: true,
    },
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
        let serverError = connectionCloseProtocol[this.message as ConnectionClose];
        if (serverError) return serverError;

        // Abnormal closure
        if (this.code === 1006) {
            return connectionCloseProtocolDefaults.abnormalClosure;
        }

        return { message: "Unknown error", canReconnect: true };
    }
}

type SocketState =
    | {
          status: ConnectionStatus.Closed;
          reason: ConnectionCloseProtocol;
      }
    | {
          status: Exclude<ConnectionStatus, ConnectionStatus.Closed>;
          reason?: never;
      };

const HANDSHAKE_TIMEOUT_MS = 5000;
const RECONNECT_BASE_DELAY_MS = 1000;
const RECONNECT_MAX_DELAY_MS = 30_000;
const RECONNECT_MAX_ATTEMPTS = 10;

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
    public socketState: SocketState = $state({ status: ConnectionStatus.Idle });
    private reconnectAttempts = $state(0);
    private reconnectTimer?: ReturnType<typeof setTimeout>;
    private manualClose = false;

    public statusString = $derived.by(() => {
        switch (this.socketState.status) {
            case ConnectionStatus.Connecting: {
                let output = `Connecting..`;

                if (this.reconnectAttempts) {
                    output += ` (attempt ${this.reconnectAttempts}/${RECONNECT_MAX_ATTEMPTS})`;
                }
                return output;
            }
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
        let payload: { gameId: string; hostId: string };

        try {
            const response = await fetch(`${PUBLIC_SERVER_HTTP_URL}/lobby`, { method: "POST" });
            if (!response.ok) throw new Error("Non 200 response");
            payload = await response.json();
        } catch (error) {
            throw new Error(`Failed to create session: `, { cause: error });
        }

        console.log(`${TAG} Created session with ID:`, payload.gameId);

        const client = new this(payload.gameId, true);
        await client.connectToSocket(payload.hostId);
        return client;
    }

    // ### Socket ###
    public async restartConnectionCycle() {
        this.reconnectAttempts = 0;
        clearTimeout(this.reconnectTimer);
        await this.connectToSocket();
    }

    public async connectToSocket(requestedClientId?: string): Promise<void> {
        clearTimeout(this.reconnectTimer);

        this.socketState = { status: ConnectionStatus.Connecting };
        this.manualClose = false;

        const clientId = requestedClientId ?? this.clientId ?? sessionStorage.getItem(CLIENT_ID_KEY) ?? undefined;

        const url = `${PUBLIC_SERVER_WS_URL}/lobby/${this.session_id}/ws`;
        console.debug(`${TAG} Connecting to websocket at ${url}`);

        const socket = new WebSocket(url);

        try {
            const assignedClientId = await WebsocketClient.connectionHandshake(socket, clientId);

            sessionStorage.setItem(CLIENT_ID_KEY, assignedClientId);
            this.clientId = assignedClientId;
            this.socket = socket;
            this.socketState = { status: ConnectionStatus.Connected };
            this.reconnectAttempts = 0;

            this.attachSocketListeners(socket);
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

            socket.addEventListener("open", () => {
                socket.send(`${OutgoingMessageTopic.Connect}\n${requestedClientId ?? ""}`);
            });
        });
    }

    private attachSocketListeners(socket: WebSocket): void {
        socket.addEventListener("message", (event) => {
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
        });

        socket.addEventListener("close", (event) => {
            let error = new SocketError(event.code, event.reason);
            console.log({ event, protocol: error.get_protocol() });

            if (!error.get_protocol().canReconnect || this.manualClose) {
                this.socketState = { status: ConnectionStatus.Closed, reason: error.get_protocol() };

                for (const callback of this.closeListeners) callback();
                return;
            }

            this.scheduleReconnect();
        });

        for (const callback of this.readyListeners) callback();
    }

    private scheduleReconnect(requestedClientId?: string): void {
        if (this.reconnectAttempts >= RECONNECT_MAX_ATTEMPTS) {
            console.error(`${TAG} Max reconnect attempts reached.`);
            this.socketState = {
                status: ConnectionStatus.Closed,
                reason: connectionCloseProtocolDefaults.connectionFailed,
            };

            for (const cb of this.closeListeners) cb();
            return;
        }

        // Exponential backoff with jitter
        const delay = Math.min(
            RECONNECT_BASE_DELAY_MS * 2 ** this.reconnectAttempts + Math.random() * 500,
            RECONNECT_MAX_DELAY_MS,
        );

        this.reconnectAttempts++;
        console.debug(`${TAG} Reconnecting in ${Math.round(delay)}ms (attempt ${this.reconnectAttempts})`);

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
        if (document.visibilityState === "visible") {
            this.send(OutgoingMessageTopic.Check, "");
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
}
