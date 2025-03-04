import { TimeLimitRegex, TurnsRegex, type PrototypeCard } from "lib/card.svelte";
import { nanoid } from "nanoid";
import { ImageService } from "./image";


export class CardManager implements PrototypeCard {
    public id: PrototypeCard["id"];
    public title: PrototypeCard["title"];
    public text: PrototypeCard["text"];
    public overrides: PrototypeCard["overrides"];
    public turns: PrototypeCard["turns"];
    public isNsfw: PrototypeCard["isNsfw"];
    public timeLimit: PrototypeCard["timeLimit"];
    public hasWheel: PrototypeCard["hasWheel"];
    public image: PrototypeCard["image"];

    constructor(card: PrototypeCard) {
        this.title = card.title;
        this.text = card.text;
        this.overrides = card.overrides
        this.turns = card.turns;
        this.isNsfw = card.isNsfw;
        this.timeLimit = card.timeLimit;
        this.hasWheel = card.hasWheel;
        this.image = card.image;
        this.id = card.id || nanoid(16);
    }


    public async check(transform = false): Promise<PrototypeCard | null> {

        // Id
        if (typeof this.id !== "string" && !transform) {
            console.error("- Card id is required");
            return null;
        }

        this.id = this.id || nanoid(16);

        // Title
        if ((this.title !== undefined && typeof this.title !== "string") || this.title?.length === 0) {
            console.error("- Card title must be a string or undefined");
            return null;
        }

        // Text
        if (typeof this.text !== "string" || this.text.length === 0) {
            console.error("- Card text is required");
            return null;
        }

        // Turns
        if (this.turns !== undefined && (this.turns < -1 || this.turns === 0)) {
            console.error("- Card turns must be -1 or greater than 0");
            return null;
        }

        // Turns placeholder
        if (this.text.match(TurnsRegex) && this.turns === undefined) {
            console.error("- Card has %TURNS% placeholder but no turns defined");
            return null;
        }

        // Time limit
        if (this.timeLimit !== undefined && this.timeLimit <= 0) {
            console.error("- Card time limit must be greater than 0");
            return null;
        }

        // Time limit placeholder
        if (this.text.match(TimeLimitRegex) && this.timeLimit === undefined) {
            console.error("- Card has %TIMELIMIT% placeholder but no time limit defined");
            return null;
        }

        // Overrides
        if (this.overrides !== undefined && !Array.isArray(this.overrides)) {
            console.error("- Card overrides must be an array or undefined");
            return null;
        }

        for (const override of this.overrides || []) {
            if (typeof override !== "string" || override.length === 0) {
                console.error("- Card override must be a string");
                return null;
            }
        }

        // NSFW
        if (this.isNsfw !== undefined && this.isNsfw !== true) {
            console.error("- Card isNsfw must be true or undefined");
            return null;
        }

        if (transform) {
            // Image
            if (typeof this.image === "string") {
                if (this.image.length === 0) {
                    console.error("- Empty image path");
                    return null;
                } else {
                    await ImageService.saveImage(this.id, this.image);
                    this.image = true;
                }
            }

        }

        // Image
        if (this.image !== true && this.image !== undefined) {
            console.error("- Card image must be true or undefined");
            return null;
        }

        return {
            id: this.id,
            title: this.title,
            text: this.text,
            overrides: this.overrides,
            turns: this.turns,
            isNsfw: this.isNsfw,
            timeLimit: this.timeLimit,
            hasWheel: this.hasWheel,
            image: this.image,
        }
    }
}
