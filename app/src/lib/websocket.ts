import { PUBLIC_SERVER_URL } from "$env/static/public";
import { dev } from "$app/environment";

export class WebsocketClient {
    private socket: WebSocket;
    public readonly playerId: string;
    public readonly id: string;

    constructor(socket: WebSocket, id: string, playerId: string) {
        this.socket = socket;
        this.id = id;
        this.playerId = playerId;
    }

    public static async createSession(): Promise<WebsocketClient | null> {
        const response = await fetch(`${dev ? "http" : "https"}://${PUBLIC_SERVER_URL}/lobby`, { method: "POST" });

        if (!response.ok) {
            return null;
        }

        const payload = await response.json();

        console.log("Created session with ID: ", payload.gameId);

        return await this.connectToSocket(payload.gameId, payload.hostId);
    }

    public static async connectToSession(sessionId: string): Promise<WebsocketClient | null> {
        return await this.connectToSocket(sessionId).catch(() => null);
    }

    private static async connectToSocket(gameId: string, clientId: string = ""): Promise<WebsocketClient | null> {
        const url = `${dev ? "ws" : "wss"}://${PUBLIC_SERVER_URL}/lobby/${gameId}/ws`;
        const socket = new WebSocket(url);

        console.debug(`Connecting to websocket at ${url}`);

        try {
            // Wait for open event before returning the client, 5 sec timeout
            await new Promise((resolve, reject) => {
                const timeout = setTimeout(() => {
                    reject(new Error("Websocket connection timed out"));
                }, 5000);

                socket.addEventListener("open", () => {
                    const connectMessage = `connect\n${clientId}`;
                    socket.send(connectMessage);
                });

                const onMessage = (event: MessageEvent) => {
                    const data: string = event.data;

                    if (data.startsWith("client_id")) {
                        clientId = data.split("\n")[1];
                    }

                    socket.removeEventListener("message", onMessage);

                    clearTimeout(timeout);
                    resolve(socket);
                };

                socket.addEventListener("message", onMessage);

                socket.addEventListener("error", () => {
                    clearTimeout(timeout);
                    reject(new Error("Websocket connection error"));
                });
            });
        } catch (error) {
            console.error("Websocket connection failed: ", error);
            return null;
        }

        return new WebsocketClient(socket, gameId, clientId);
    }

    public send(type: string, payload: string | Record<string, unknown>) {
        const message = `${type}\n${typeof payload === "string" ? payload : JSON.stringify(payload)}`;
        this.socket.send(message);
    }

    public onMessage(callback: (type: string, payload: string) => void) {
        this.socket.addEventListener("message", (event) => {
            const data: string = event.data;

            const [type, payload] = data.split("\n");
            callback(type, payload);
        });
    }

    public onClose(callback: () => void) {
        this.socket.addEventListener("close", () => {
            callback();
        });
    }

    public close() {
        this.socket.close();
    }
}
