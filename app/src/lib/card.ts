export interface Card {
    id: string;
    title: string;
    text: string;
    turns?: number;
    minimumPlayers: number;
    maximumPlayers: number;
    timeLimit?: number;
    // Cards that get removed from the deck when this card is loaded
    overrides?: string[];
    hasImage: boolean;
    hasWheel: boolean;
    isNsfw: boolean;
}
