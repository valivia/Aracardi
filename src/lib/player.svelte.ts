import type { Avatar } from "assets/avatars/avatars.svelte";

export class Player {
    public id: string;
    public name: string;
    public avatar: Avatar;

    constructor(name: string, avatar: Avatar) {
        this.name = name;
        this.avatar = avatar;
        this.id = Math.random().toString(36).substring(2, 9);
    }
}
