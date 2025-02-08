import type { Player } from "lib/lobby/player";
import type { Card } from "lib/card";
import { shuffle } from "lib/helpers";

export type PlayerSelector = "HOST"
    | "CURRENT"
    | "PREVIOUS"
    | "NEXT"
    | `RANDOM.${string}`;

export enum CardPartType {
    text = "text",
    currentPlayer = "currentPlayer",
    player = "player",
    turns = "turns",
}

export interface CardPart {
    value: string;
    type: CardPartType;
}

export class CardController implements Card {
    public id: string;
    public title: string;
    public text: string;
    public turns = $state<number | undefined>(undefined);
    public timeLimit?: number;
    public hasImage: boolean;
    public hasWheel: boolean;
    public isNsfw: boolean;
    public players: Set<Player> = new Set();

    public formattedText: CardPart[];

    constructor(card: Card, players: Player[], currentPlayerIndex: number) {
        this.id = card.id;
        this.title = card.title;
        this.text = card.text;
        this.turns = card.turns;
        this.timeLimit = card.timeLimit;
        this.hasImage = card.hasImage;
        this.hasWheel = card.hasWheel;
        this.isNsfw = card.isNsfw;

        this.formattedText = this.build(players, currentPlayerIndex);
    }

    private build(players: Player[], currentPlayerIndex: number): CardPart[] {
        const currentPlayer = players[currentPlayerIndex];

        let processed_text: CardPart[] = [{ value: this.text, type: CardPartType.text }];

        const replacePlaceholder = (regex: RegExp, getValue: () => CardPart["value"], type: CardPartType) => {
            processed_text = processed_text.flatMap((part) => {
                if (part.type === CardPartType.text && regex.test(part.value)) {
                    const result: CardPart[] = [];

                    // Reset regex lastIndex for global searches
                    regex.lastIndex = 0;
                    let lastIndex = 0;

                    for (const match of part.value.matchAll(regex)) {
                        const matchStart = match.index!;
                        const matchEnd = matchStart + match[0].length;

                        // Add text before the match
                        if (lastIndex < matchStart) {
                            result.push({ value: part.value.slice(lastIndex, matchStart), type: CardPartType.text });
                        }

                        // Add the placeholder replacement
                        result.push({ value: getValue(), type });

                        lastIndex = matchEnd;
                    }

                    // Add any remaining text after the last match
                    if (lastIndex < part.value.length) {
                        result.push({ value: part.value.slice(lastIndex), type: CardPartType.text });
                    }

                    return result;
                }
                return part;
            });
        };

        // Replace placeholders
        replacePlaceholder(/%SELF%/g, () => {
            this.players.add(currentPlayer);
            return currentPlayer.name;
        }, CardPartType.currentPlayer);

        replacePlaceholder(/%NEXT_PLAYER%/g, () => {
            const nextPlayer = players[(currentPlayerIndex + 1) % players.length];
            this.players.add(nextPlayer);
            return nextPlayer.name;
        }, CardPartType.player);

        replacePlaceholder(/%PREVIOUS_PLAYER%/g, () => {
            const prevPlayer = players[(currentPlayerIndex - 1 + players.length) % players.length];
            this.players.add(prevPlayer);
            return prevPlayer.name;
        }, CardPartType.player);

        // Handle %TURNS% placeholder
        if (this.turns !== undefined) {
            replacePlaceholder(/%TURNS%/g, () => `${this.turns}`, CardPartType.turns);
        }

        // Filter out used players
        const remainingPlayers = players.filter((player) => !this.players.has(player));

        // Random Players
        const randomPlayerRegex = /%PLAYER(\d+)%/g;
        const placeholders = [...new Set(this.text.match(randomPlayerRegex))];

        if (placeholders.length) {
            const shuffledPlayers = shuffle(remainingPlayers).slice(0, placeholders.length);
            shuffledPlayers.forEach(player => this.players.add(player));

            placeholders.forEach((placeholder, index) => {
                const player = shuffledPlayers[index];
                replacePlaceholder(new RegExp(placeholder, 'g'), () => player.name, CardPartType.player);
            });
        }

        return processed_text;
    }

    public nextTurn() {
        if (this.turns === undefined) return;
        if (this.turns <= 0) return;
        this.turns -= 1;
    }
}
