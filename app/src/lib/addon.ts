import type { Card } from "./card";

export interface Addon {
    id: string;
    title: string;
    description: string;
    isOfficial: boolean;
    cards: Card[];
}


export interface AddonSummary extends Omit<Addon, "cards"> {
    cardCount: number;
    nsfwCardCount: number;
}
