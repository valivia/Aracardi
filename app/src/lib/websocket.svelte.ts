import { PUBLIC_SERVER_HTTP_URL, PUBLIC_SERVER_WS_URL } from "$env/static/public";
import {
    ConnectionClose,
    IncomingMessageTopic,
    OutgoingMessageTopic,
    type IncomingTopicMap,
    type OutgoingTopicMap,
} from "./protocol";

const CLIENT_ID_KEY = "clientId";

export enum ConnectionStatus {
    Idle = "IDLE",
    Connecting = "CONNECTING",
    Connected = "CONNECTED",
    Refused = "REFUSED",
    Failed = "FAILED",
    Closed = "CLOSED",
}

class ServerConnectionError extends Error {
    override name = "ServerConnectionError";

    constructor(
        public readonly code: number,
        message: string,
    ) {
        super(message);
    }
}

const HANDSHAKE_TIMEOUT_MS = 5000;
const RECONNECT_BASE_DELAY_MS = 1000;
const RECONNECT_MAX_DELAY_MS = 30_000;
const RECONNECT_MAX_ATTEMPTS = 10;

export class WebsocketClient {
    public id: string;
    private socket?: WebSocket;
    private clientId?: string;

    // Listeners
    private messageListeners = new Map<string, Set<(payload: unknown) => void>>();
    private closeListeners = new Set<() => void>();

    // Connection
    public connectionStatus: ConnectionStatus = $state(ConnectionStatus.Idle);
    private connectionRefusedReason: string | null = $state(null);
    private reconnectAttempts = $state(0);
    private reconnectTimer?: ReturnType<typeof setTimeout>;
    private manualClose = false;

    public connectionStatusString = $derived.by(() => {
        switch (this.connectionStatus) {
            case ConnectionStatus.Connecting: {
                let output = `Connecting..`;

                if (this.reconnectAttempts) {
                    output += ` (attempt ${this.reconnectAttempts}/${RECONNECT_MAX_ATTEMPTS})`;
                }
                return output;
            }
            case ConnectionStatus.Refused:
                return this.connectionRefusedReason;
            case ConnectionStatus.Closed:
                return "Connection closed";
            case ConnectionStatus.Failed:
                return "Failed to connect";
        }

        return "";
    });

    constructor(id: string) {
        this.id = id;
    }

    public static async createSession(): Promise<WebsocketClient | null> {
        let payload: { gameId: string; hostId: string };

        try {
            const response = await fetch(`${PUBLIC_SERVER_HTTP_URL}/lobby`, { method: "POST" });
            if (!response.ok) return null;
            payload = await response.json();
        } catch (error) {
            console.error("Failed to create session: ", error);
            return null;
        }

        console.log("Created session with ID:", payload.gameId);

        const client = new this(payload.gameId);
        await client.connectToSocket(payload.hostId).catch(() => null);
        return client;
    }

    // ### Socket ###
    public async connectToSocket(requestedClientId?: string): Promise<void> {
        clearTimeout(this.reconnectTimer);

        this.connectionStatus = ConnectionStatus.Connecting;
        this.manualClose = false;

        const clientId = requestedClientId ?? this.clientId ?? sessionStorage.getItem(CLIENT_ID_KEY) ?? undefined;

        const url = `${PUBLIC_SERVER_WS_URL}/lobby/${this.id}/ws`;
        console.debug(`Connecting to WebSocket at ${url}`);

        const socket = new WebSocket(url);

        try {
            const assignedClientId = await WebsocketClient.connectionHandshake(socket, clientId);

            sessionStorage.setItem(CLIENT_ID_KEY, assignedClientId);
            this.clientId = assignedClientId;
            this.socket = socket;
            this.connectionStatus = ConnectionStatus.Connected;
            this.reconnectAttempts = 0;

            this.attachSocketListeners(socket);
        } catch (error) {
            if (error instanceof ServerConnectionError) {
                console.info("Websocket connection refused:", error.message);
                this.connectionRefusedReason = this.resolveRefusalReason(error.message);
                this.connectionStatus = ConnectionStatus.Refused;
            } else {
                console.error("Websocket connection failed:", error);
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
                reject(new ServerConnectionError(event.code, event.reason));
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
                console.error("Failed to parse message payload for topic:", message.topic);
            }
        });

        socket.addEventListener("close", (event) => {
            if (event.code === 1000 || event.code === 4000 || this.manualClose) {
                if (event.code === 4000) {
                    this.connectionStatus = ConnectionStatus.Refused;
                } else {
                    this.connectionStatus = ConnectionStatus.Closed;
                }

                for (const callback of this.closeListeners) callback();
                return;
            }
            this.scheduleReconnect();
        });
    }

    private scheduleReconnect(requestedClientId?: string): void {
        if (this.reconnectAttempts >= RECONNECT_MAX_ATTEMPTS) {
            console.error("Max reconnect attempts reached.");
            this.connectionStatus = ConnectionStatus.Failed;
            for (const cb of this.closeListeners) cb();
            return;
        }

        // Exponential backoff with jitter
        const delay = Math.min(
            RECONNECT_BASE_DELAY_MS * 2 ** this.reconnectAttempts + Math.random() * 500,
            RECONNECT_MAX_DELAY_MS,
        );

        this.reconnectAttempts++;
        console.debug(`Reconnecting in ${Math.round(delay)}ms (attempt ${this.reconnectAttempts})`);

        this.reconnectTimer = setTimeout(async () => {
            await this.connectToSocket(requestedClientId).catch(() => null);
        }, delay);
    }

    //  ### API ###
    public send<Topic extends keyof OutgoingTopicMap>(topic: Topic, payload: OutgoingTopicMap[Topic]) {
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

    public onClose(callback: () => void): () => void {
        this.closeListeners.add(callback);
        return () => this.closeListeners.delete(callback);
    }

    public close() {
        this.manualClose = true;
        clearTimeout(this.reconnectTimer);
        this.socket?.close();
        this.connectionStatus = ConnectionStatus.Closed;
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

    private resolveRefusalReason(serverMessage: string): string {
        switch (serverMessage) {
            case ConnectionClose.NotFound:
                return this.clientId ? "Session no longer exists" : "Session not found";
            case ConnectionClose.GameFull:
                return "This lobby is full";
        }

        return serverMessage;
    }
}
