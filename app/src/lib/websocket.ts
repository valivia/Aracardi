import { PUBLIC_SERVER_URL } from "$env/static/public";

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
        const response = await fetch(
            `http://${PUBLIC_SERVER_URL}/lobby`,
            { method: "POST" }
        );

        if (!response.ok) {
            return null;
        }

        const id = await response.text();

        console.log("Created session with ID: ", id);

        return await this.connectToSocket(id);
    }

    public static async connectToSession(sessionId: string): Promise<WebsocketClient | null> {
        return await this.connectToSocket(sessionId).catch(() => null);

    }

    private static async connectToSocket(id: string): Promise<WebsocketClient | null> {
        const url = `ws://${PUBLIC_SERVER_URL}/lobby/${id}/ws`;
        const socket = new WebSocket(url);
        let playerId = "";

        console.debug(`Connecting to websocket at ${url}`);

        try {
            // Wait for open event before returning the client, 5 sec timeout
            await new Promise((resolve, reject) => {
                const timeout = setTimeout(() => {
                    reject(new Error("Websocket connection timed out"));
                }, 5000);

                socket.addEventListener("open", () => {
                    const player_id = localStorage.getItem("player_id");
                    const connectMessage = `connect:${player_id ?? ""}`;
                    socket.send(connectMessage);
                });

                const onMessage = (event: MessageEvent) => {
                    const data: string = event.data;

                    if (data.startsWith("player_id:")) {
                        playerId = data.split(":")[1];
                        localStorage.setItem("player_id", playerId);
                    }

                    socket.removeEventListener("message", onMessage);

                    clearTimeout(timeout);
                    resolve(socket);
                }

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

        return new WebsocketClient(socket, id, playerId);
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

    public close() {
        this.socket.close();
    }
}
