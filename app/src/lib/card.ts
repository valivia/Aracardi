export interface Card {
    id: string;
    title: string;
    text: string;
    turns: number | null;
    minimumPlayers: number;
    maximumPlayers: number;
    timeLimit: number | null;
    hasImage: boolean;
    isNsfw: boolean;
}
