import { readdirSync } from "fs";
import type { PrototypeAddon } from "lib/addon";
import { nanoid } from "nanoid";
import path from "path";
import { BASE_PATH } from ".";
import { CardManager } from "./card";
import { writeFile } from "fs/promises";

export class AddonManager implements PrototypeAddon {
    private fileName: string;
    public id: string;
    public title: PrototypeAddon["title"];
    public description: PrototypeAddon["description"];
    public isDefault: PrototypeAddon["isDefault"];
    public isOfficial: PrototypeAddon["isOfficial"];
    public cards: PrototypeAddon["cards"];

    constructor(addon: PrototypeAddon, fileName: string) {
        this.fileName = fileName;
        this.id = addon.id || nanoid(16);
        this.title = addon.title;
        this.description = addon.description;
        this.isDefault = addon.isDefault;
        this.isOfficial = addon.isOfficial
        this.cards = addon.cards;
    }

    async check(transform = false) {
        console.log(`🔃 Checking addon ${this.title}`);

        let hasErrors = false;

        if (typeof this.id !== "string") {
            console.error("❌ Addon ID must be a string");
            hasErrors = true;
        }

        if (!this.title) {
            console.error("❌ Addon title is required");
            hasErrors = true;
        }

        if (!this.description) {
            console.error("❌ Addon description is required");
            hasErrors = true;
        }

        if (typeof this.isDefault !== "boolean") {
            console.error("❌ Addon isDefault must be a boolean");
            hasErrors = true;
        }

        if (typeof this.isOfficial !== "boolean") {
            console.error("❌ Addon isOfficial must be a boolean");
            hasErrors = true;
        }

        if (!this.cards.length) {
            console.error("❌ Addon must have at least one card");
            hasErrors = true;
        }


        for (let i = 0; i < this.cards.length; i++) {
            try {
                const card = await new CardManager(this.cards[i]).check(transform);
                if (!card) {
                    console.log(`❌ Card "${this.cards[i].id || this.cards[i].title || this.cards[i].text.slice(0, 12)}" has errors`);
                    hasErrors = true;
                    continue;
                }
                this.cards[i] = card;
            } catch (error) {
                console.error(`❌ Error in card "${this.cards[i].id}" from addon "${this.title}"`);
                throw error;
            }
        }

        if (hasErrors) {
            return null;

        } else {
            console.log(`✅ Addon "${this.title}" is valid`);
        }

        return this;
    }

    public async save() {
        await writeFile(path.join(BASE_PATH, "addons", `${this.fileName}`), JSON.stringify(this, null, 4));
        console.log(`💾 Addon "${this.title}" saved`);
    }

    public static async getAddons(): Promise<Record<string, PrototypeAddon>> {
        const files = readdirSync(path.join(BASE_PATH, "addons"), { withFileTypes: true });

        const addonEntries = await Promise.all(
            files
                .filter((file) => file.isFile() && file.name.endsWith(".json"))
                .map(async (file) => {
                    const addon = await import(`../../../assets/addons/${file.name}`).then((mod) => mod.default as PrototypeAddon);
                    return [file.name, addon];
                })
        );

        return Object.fromEntries(addonEntries);
    }


}
