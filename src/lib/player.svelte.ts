import type { Avatar } from "assets/avatars/avatars.svelte";

export class Player {
    public name: string;
    public avatar: Avatar;

    public get id() {
        return `this.name}_${this.avatar.name}`;
    }

    constructor(name: string, avatar: Avatar) {
        this.name = name;
        this.avatar = avatar;
    }
}
