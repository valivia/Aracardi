import type { Addon, AddonSummary } from "lib/addon";
import type { Card } from "lib/card";
import { shuffle } from "lib/helpers";
import type { Player } from "lib/lobby/player";
import { CardController } from "./card/card.svelte";

export enum GameStage {
    addonSetup,
    playerSetup,
    game,
}

export class GameState {
    // Stage
    private stage: GameStage = $state(GameStage.addonSetup);
    public get currentStage() { return this.stage };

    // Content
    public cards: Card[] = $state([]);
    public players: Player[] = $state([]);

    // Current player
    public currentPlayerIndex: number = $state(0);
    public get currentPlayer(): Player {
        return this.players[this.currentPlayerIndex];
    }

    // Current card
    private currentCardIndex: number = $state(0);
    public currentCard: CardController | null = $state(null);

    // Active cards
    public activeCards: CardController[] = $state([]);

    // Setup
    public selectedAddons: AddonSummary[] = $state([]);


    constructor() { }

    // Addons
    public toggleAddon = (addon: AddonSummary) => {
        const index = this.selectedAddons.findIndex(a => a.id === addon.id);
        if (index === -1) {
            this.selectedAddons.push(addon);
        } else {
            this.selectedAddons.splice(index, 1);
        }
    }

    public hasAddon = (addon: AddonSummary) => {
        return this.selectedAddons.some(a => a.id === addon.id);
    }

    // Cards
    public loadCards = async () => {
        const addons = await Promise.all(this.selectedAddons.map(async addonInfo => {
            const json = await import(`assets/addons/${addonInfo.id}.json`)
            const addon = json.default as Addon;
            return addon;
        }));


        const cards = addons.flatMap(addon => addon.cards);
        // TODO: filter out card overrides

        this.cards = shuffle(cards);

        this.setStage(GameStage.playerSetup);
    };

    // Players
    public upsertPlayer = (player: Player) => {
        const index = this.players.findIndex(p => p.id === player.id);
        if (index === -1) {
            this.players.push(player);
        } else {
            this.players[index] = player;
        }
    }

    public removePlayer = (player: Player) => {
        this.players = this.players.filter(p => p !== player);
    }

    public shufflePlayers = () => {
        const current = this.currentPlayer;
        this.players = shuffle(this.players);
        this.currentPlayerIndex = this.players.findIndex(p => p === current);
    }

    // Game
    public nextTurn = () => {
        this.currentPlayerIndex = (this.currentPlayerIndex + 1) % this.players.length;

        if (this.currentCard?.turns) {
            this.activeCards.push(this.currentCard);
        }

        for (const card of this.activeCards) {
            card.nextTurn();

            if (card.turns === 0) {
                this.activeCards = this.activeCards.filter(c => c !== card);
            }
        }

        // Card
        this.currentCardIndex = (this.currentCardIndex + 1) % this.cards.length;
        this.currentCard = new CardController(this.cards[this.currentCardIndex], [...this.players], this.currentPlayerIndex);
    }

    public setStage(state: GameStage) {
        if (state === GameStage.game && !this.currentCard) {
            this.currentCard = new CardController(this.cards[this.currentCardIndex], [...this.players], this.currentPlayerIndex);
        }

        this.stage = state;
    }
}
