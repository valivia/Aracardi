import { avatars, type Avatar } from "assets/avatars/avatars.svelte";

interface JsonPlayer {
    id: string;
    name: string;
    avatar: string;
}

export class Player {
    public id: string;
    public name: string;
    public avatar: Avatar;

    constructor(name: string, avatar: Avatar | string) {

        this.id = Math.random().toString(36).substring(2, 9);

        this.name = name;

        this.avatar = typeof avatar === "string"
            ? avatars.find((a) => a.name === avatar) ?? avatars[0]
            : avatar;
    }

    get htmlId() {
        return `player_${this.id}`;
    }

    // Storage
    public getSaveable() {
        return {
            id: this.id,
            name: this.name,
            avatar: this.avatar.name,
        };
    }

    public static savePlayers(players: Player[]) {
        const json = JSON.stringify(players.map(p => p.getSaveable()));
        localStorage.setItem("players", json);
    }

    public static loadPlayers() {
        const json = localStorage.getItem("players");
        if (json) {
            const players = JSON.parse(json);
            return players.map((p: JsonPlayer) => new Player(p.name, p.avatar));
        }
        return [];
    }

    public static getSavedPlayerCount() {
        const json = localStorage.getItem("players");
        if (json) {
            return JSON.parse(json).length;
        }
        return 0;
    }
}
