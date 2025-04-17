import type { Addon, AddonSummary } from "lib/addon";
import { CardController, type Card } from "./card.svelte";
import { shuffle } from "./helpers";
import { Player } from "./player.svelte";
import { PUBLIC_TELEMETRY_URL } from "$env/static/public";
import { dev } from "$app/environment";

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
    private stage: GameStage = $state(GameStage.playerSetup);
    public settingsOpen = $state(false);
    public get currentStage() { return this.stage };

    // Content
    public cards: Card[] = $state([]);
    public disabledCards: Card[] = $state([]);
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
    public projectedCardCount = $derived.by(() => {
        return this.selectedAddons.reduce((acc, addon) => {
            return acc + addon.cardCount - (this.settings.allowNsfw ? 0 : addon.nsfwCardCount);
        }, 0);
    });

    public isOngoing = $derived.by(() => {
        return this.currentCard !== null;
    });

    constructor(addons: AddonSummary[]) {
        this.selectedAddons = addons.filter(a => a.isDefault);

        // Check if client side
        if (typeof window !== "undefined")
            this.hasPreviousPlayers = localStorage.getItem("players") !== null;
    }

    // Addons
    public toggleAddon = (addon: AddonSummary) => {
        const index = this.selectedAddons.findIndex(a => a.id === addon.id);
        if (index === -1) {
            this.selectedAddons.push(addon);
        } else {
            this.selectedAddons.splice(index, 1);
        }

        this.saveAddons();
    }

    public hasAddon = (addon: AddonSummary) => {
        return this.selectedAddons.some(a => a.id === addon.id);
    }

    private saveAddons() {
        const savedAddons = this.selectedAddons.map(a => a.id);
        localStorage.setItem("addons", JSON.stringify(savedAddons));
    }

    public restoreAddons(availableAddons: AddonSummary[]) {
        const json = localStorage.getItem("addons");
        if (json) {
            const savedAddons = JSON.parse(json) as string[];
            console.info(`- Addons loaded (${savedAddons.length})`);
            const selectedAddons = availableAddons.filter(addon => savedAddons.some(id => id === addon.id));
            this.selectedAddons = selectedAddons;
        }
    }

    // Cards
    public loadCards = async () => {
        const addons = await Promise.all(this.selectedAddons.map(async addonInfo => {
            const json = await import(`assets/addons/${addonInfo.fileName}.json`)
            const addon = json.default as Addon;
            return addon;
        }));

        this.cards = shuffle(addons.flatMap(addon => addon.cards));
        this.filterCards();
    };

    public filterCards = () => {

        const allCards = [...this.cards, ...this.disabledCards];

        type SeparatedCards = {
            validCards: Card[],
            invalidCards: Card[]
        }

        const overrideIds: Set<string> = new Set(allCards.flatMap(card => card.overrides || []));

        // Filter min and max players
        const { validCards, invalidCards } = allCards.reduce<SeparatedCards>((acc, card) => {
            let isInvalid = false;

            // Game settings
            isInvalid = isInvalid ||
                (card.isNsfw === true && !this.settings.allowNsfw) ||
                (!this.settings.allowDuplicates && overrideIds.has(card.id));

            // Players
            isInvalid = isInvalid ||
                (card.minPlayers !== undefined && this.players.length < card.minPlayers) ||
                (card.maxPlayers !== undefined && this.players.length > card.maxPlayers);

            // Push to correct array
            if (isInvalid) {
                acc.invalidCards.push(card);
            } else {
                acc.validCards.push(card);
            }

            return acc;
        }, { validCards: [], invalidCards: [] });

        console.log(`- Filtered cards: ${validCards.length} valid, ${invalidCards.length} invalid`);

        this.cards = validCards;
        this.disabledCards = invalidCards;
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
            this.filterCards();
        } else {
            this.players[index] = player;
        }

        this.savePlayers();
    }

    public removePlayer = (player: Player) => {
        this.players = this.players.filter(p => p !== player);
        this.savePlayers();
        this.filterCards();

        this.activeCards = this.activeCards.filter(card => !card.players.has(player));
    }

    private savePlayers() {
        Player.savePlayers(this.players);
        this.hasPreviousPlayers = this.players.length > 0;
    }

    public restorePlayers() {
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

        this.incrementActiveCards();

        if (this.currentCard?.turns) {
            this.activeCards.push(this.currentCard);
        }

        // Card
        this.currentCardIndex = (this.currentCardIndex + 1) % this.cards.length;
        this.currentCard = new CardController(this.cards[this.currentCardIndex], [...this.players], this.currentPlayerIndex);
    }

    public async setStage(state: GameStage) {
        if (state === GameStage.game && !this.currentCard) {
            await this.loadCards();
            if (this.cards.length < 10) {
                alert("Not enough cards");
                return;
            }
            this.currentCard = new CardController(this.cards[this.currentCardIndex], [...this.players], this.currentPlayerIndex);
            this.logGame();
        }

        this.stage = state;
    }

    // Settings
    public restoreSettings() {
        const settings = localStorage.getItem("settings");
        if (settings) {
            console.info("- Settings loaded");
            this.settings = JSON.parse(settings);
        }
    }


    // Telemetry
    public async logGame(action = "start") {
        if (dev) return;
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
