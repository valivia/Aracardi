import { avatars, type Avatar } from "assets/avatars/avatars.svelte";
import { nanoid } from 'nanoid'

interface JsonPlayer {
    id: string;
    name: string;
    avatar: string;
}

export class Player {
    public id = $state(nanoid(16));
    public name = $state("");
    public avatar = $state(avatars[0]);

    constructor(name: string, avatar: Avatar | string) {
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
        if (players.length === 0) {
            localStorage.removeItem("players");
            return;
        }

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
