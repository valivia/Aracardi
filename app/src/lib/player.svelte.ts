import { avatars, type Avatar } from "assets/avatars/avatars.svelte";
import { nanoid } from 'nanoid'

interface JsonPlayer {
    id: string;
    name: string;
    avatar: string;
    isHandPicked: boolean;
}

export class Player {
    public id = $state(nanoid(16));
    public name = $state("");
    public avatar = $state(avatars[0]);
    public isHandPicked = false;

    constructor(name: string, avatar: Avatar | string) {
        this.name = name;

        if (typeof avatar === "string") {
            const found = avatars.find(a => a.name === avatar);
            if (!found) throw new Error(`Avatar ${avatar} not found`);
            avatar = found;
        }

        this.avatar = avatar;

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
            isHandPicked: this.isHandPicked,
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
        if (!json) return [];

        const input = JSON.parse(json) as JsonPlayer[];
        const players: Player[] = [];

        for (const item of input) {
            try {
                const player = new Player(item.name, item.avatar)
                player.isHandPicked = item.isHandPicked;

                if (players.find(p => p.avatar.name === player.avatar.name)) {
                    throw new Error(`Duplicate avatar ${player.avatar.name}`);
                }

                players.push(player);
            } catch (e) {
                console.warn("Failed to serialize player", e);
            }
        }

        this.savePlayers(players);
        return players;
    }

    public static getSavedPlayerCount() {
        const json = localStorage.getItem("players");
        if (json) {
            return JSON.parse(json).length;
        }
        return 0;
    }
}
