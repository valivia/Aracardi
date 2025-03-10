import type { Card, PrototypeCard } from "./card.svelte";

export interface Addon {
    id: string;
    title: string;
    description: string;
    isDefault: boolean;
    isOfficial: boolean;
    cards: Card[];
}

export interface PrototypeAddon extends Omit<Addon, "cards" | "id"> {
    id?: string;
    cards: PrototypeCard[];
}

export interface AddonSummary extends Omit<Addon, "cards"> {
    fileName: string;
    cardCount: number;
    nsfwCardCount: number;
}
