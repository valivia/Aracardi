import type { Addon, AddonSummary } from "lib/addon";
import { CardController, type Card } from "./card.svelte";
import { shuffle } from "./helpers";
import { Player } from "./player.svelte";
import { PUBLIC_TELEMETRY_URL } from "$env/static/public";

export enum GameStage {
    addonSetup,
    playerSetup,
    game,
}

export interface Settings {
    allowNsfw: boolean;
    allowDuplicates: boolean;
    loadImages: boolean;
}

const defaultSettings: Settings = {
    allowNsfw: true,
    allowDuplicates: false,
    loadImages: true,
};

export class GameController {
    // Settings
    public settings: Settings = $state(defaultSettings);

    // Stage
    private stage: GameStage = $state(GameStage.addonSetup);
    public settingsOpen = $state(false);
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

    public hasPreviousPlayers = false;

    // Helpers
    public cardCount = $derived.by(() => {
        if (this.cards.length > 0) return this.cards.length;

        return this.selectedAddons.reduce((acc, addon) => {
            return acc + addon.cardCount - (this.settings.allowNsfw ? 0 : addon.nsfwCardCount);
        }, 0);
    });

    public isClean = $derived.by(() => {
        return this.selectedAddons.filter(a => !a.isDefault).length === 0 && this.players.length === 0;
    });

    public isOngoing = $derived.by(() => {
        return this.currentCard !== null;
    });

    constructor(addons: AddonSummary[]) {
        this.selectedAddons = addons.filter(a => a.isDefault);
    }

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


        let cards = addons.flatMap(addon => addon.cards);

        // Remove overridden cards
        const overrideIds = new Set(cards.flatMap(card => card.overrides || []));
        cards = cards.filter(card => {
            if (!this.settings.allowDuplicates && overrideIds.has(card.id)) return false;
            if (!this.settings.allowNsfw && card.isNsfw) return false;
            return true;
        });

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
        this.hasPreviousPlayers = this.players.length > 0;
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
            this.logGame();
        }

        if (state === GameStage.playerSetup) {
            this.hasPreviousPlayers = localStorage.getItem("players") !== null;
        }

        this.stage = state;
    }


    // Telemetry
    public async logGame(action = "start") {
        try {
            await fetch(`${PUBLIC_TELEMETRY_URL}/aracardi/start`, {
                method: "POST",
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    action,
                    addons: this.selectedAddons.map(a => a.title),
                    players: this.players.map(p => `${p.name} (${p.avatar.name})`),
                }),
            })
        } catch (e) {
            console.error(e);
        }
    }
}
