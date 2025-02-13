import type { Addon, AddonSummary } from "lib/addon";
import { CardController, type Card } from "./card.svelte";
import { shuffle } from "./helpers";
import { Player } from "./player.svelte";

export enum GameStage {
    addonSetup,
    playerSetup,
    game,
}

export class GameController {
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

    // Helpers
    public cardCount = $derived.by(() => {
        if (this.cards.length > 0) return this.cards.length;

        return this.selectedAddons.reduce((acc, addon) => {
            return acc + addon.cardCount;
        }, 0);
    });

    public isClean = $derived.by(() => {
        return this.selectedAddons.length === 0;
    });

    public isOngoing = $derived.by(() => {
        return this.currentCard !== null;
    });

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

    // Active cards
    public deleteActiveCard = (card: CardController) => {
        this.activeCards = this.activeCards.filter(c => c !== card);
    }

    private incrementActiveCards() {
        for (const card of this.activeCards) {
            card.nextTurn();

            if (card.turns === 0) {
                this.deleteActiveCard(card);
            }
        }
    }

    // Players
    public getPlayer(id: string) {
        return this.players.find(p => p.id === id);
    }

    public upsertPlayer = (player: Player) => {
        const index = this.players.findIndex(p => p.id === player.id);

        if (index === -1) {
            this.players.push(player);
        } else {
            this.players[index] = player;
        }

        this.savePlayers();
    }

    public removePlayer = (player: Player) => {
        this.players = this.players.filter(p => p !== player);
        this.savePlayers();

        this.activeCards = this.activeCards.filter(card => !card.players.has(player));
    }

    private savePlayers() {
        Player.savePlayers(this.players);
    }

    public loadPlayers() {
        this.players = Player.loadPlayers();
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

        this.incrementActiveCards();

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
