import type { Card, PrototypeCard } from "./card.svelte";

export interface Addon {
    id: string;
    title: string;
    description: string;
    isDefault: boolean;
    cards: Card[];
}

export interface PrototypeAddon extends Omit<Addon, "cards" | "id"> {
    id?: string;
    cards: PrototypeCard[];
}

export interface AddonSummary extends Omit<Addon, "cards"> {
    cardCount: number;
    nsfwCardCount: number;
}
