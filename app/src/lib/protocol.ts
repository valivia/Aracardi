import type { JsonPlayer } from "./player.svelte";

export interface GameCard {
    id: string;
    title: string;
    text: string;
    image: boolean;
    players: string[];
    turns?: number;
}

export interface GameUpdate {
    players?: JsonPlayer[];
    currentPlayerId?: string;

    currentCard?: GameCard;
    activeCards?: GameCard[];

}
